//! gui 界面
#![allow(dead_code)]
#![allow(unused_variables)]
use font_kit::properties::Style;
use raylib::{math::Vector2, RaylibHandle, RaylibThread};

use crate::{
    font::FontCache,
    pty::Pty,
    utils::{self, get_key_pressed, get_mouse_wheel_move},
};
use raylib::prelude::*;
use std::{
    cell::Cell,
    rc::Rc,
    sync::{Arc, Mutex, MutexGuard},
};

type Rayhead = Arc<Mutex<RaylibHandle>>;
type RayThread = Arc<RaylibThread>;
type MFontCache = Mutex<FontCache>;

pub struct Window {
    rayhead: Rayhead,
    raythread: RayThread,
    cursor: Vector2,
    pty: Pty,
    font_cache: MFontCache,
    roll: Rc<Cell<f32>>,
}

impl Window {
    pub fn new(width: i32, height: i32) -> Self {
        let (rl, thread) = utils::sim_creae_windows(width, height);
        let font_cache = FontCache::new();
        Self {
            rayhead: Arc::new(Mutex::new(rl)),
            raythread: Arc::new(thread),
            cursor: Vector2::zero(),
            pty: Pty::new("sh", 100, 100).start_reader(),
            font_cache: Mutex::new(FontCache::new()),
            roll: Rc::new(Cell::new(f32::default())),
        }
        .inti_output_handle()
    }

    fn inti_output_handle(self) -> Self {
        self
    }

    pub fn get_head(&self) -> MutexGuard<RaylibHandle> {
        self.rayhead.lock().unwrap()
    }

    pub fn get_thread(&self) -> RayThread {
        self.raythread.clone()
    }

    pub fn get_font_cache(&self) -> MutexGuard<FontCache> {
        self.font_cache.lock().unwrap()
    }

    pub fn join(&self) {
        let mut rl = self.get_head();
        let thread = self.get_thread();
        rl.set_target_fps(120);
        while !rl.window_should_close() {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::new(250, 244, 237, 1));
            Self::input_char(&self.pty, self.roll.clone());
            Self::wait_key(&self.pty, self.roll.clone());
            self.render_text(&self.pty, &mut d, self.roll.clone());
        }
    }

    pub fn wait_mouse(&self) {}

    pub fn wait_key(shell: &Pty, roll: Rc<Cell<f32>>) {
        let key = get_key_pressed();
        if key > 0 {
            let keyem = key_from_i32(key).unwrap();
            match keyem {
                KeyboardKey::KEY_ENTER => shell.send(b"\n".to_vec()),
                _ => {}
            }
        }
        let li = get_mouse_wheel_move();
        if li != 0.0 {
            roll.set(roll.get() + li * 50.0);
            println!("mouse {}", roll.get());
        }
    }

    pub fn input_char(shell: &Pty, roll: Rc<Cell<f32>>) {
        loop {
            let c_u32 = utils::get_char_pressed();
            if c_u32 > 0 {
                let c = char::from_u32(c_u32).unwrap();
                shell.send(c.to_string().as_bytes().to_vec());
            } else {
                break;
            }
        }
    }

    pub fn render_text(&self, pty: &Pty, head: &mut RaylibDrawHandle, roll: Rc<Cell<f32>>) {
        let x = head.get_screen_width();
        let y = head.get_screen_height();
        let mouse_x = head.get_mouse_x();
        let mouse_y = head.get_mouse_y();
        let s = head.get_fps();
        head.draw_fps(x - 100, y - 20);
        let font_size = 30;
        let font = self
            .get_font_cache()
            .search_system_font("MesloLGL Nerd Font".to_string(), Style::Italic, font_size)
            .unwrap();

        let mut row = 0.0;
        let mut col = 0.0;
        let scren_width = head.get_screen_width() as f32;
        for node in pty.get_buffer() {
            let size = utils::measuretextex(*font, &node.chars, font_size as f32, 1.0);
            utils::draw_text_ex(
                font.clone(),
                &node.chars,
                math::Vector2 {
                    x: col,
                    y: row + roll.get(),
                },
                font_size as f32,
                1.0,
                Color::RED,
            );
            col += size.x;
            if utils::inspect_wrap(node.chars) {
                row += size.y;
                col = 0.0;
            }
        }
    }
}
