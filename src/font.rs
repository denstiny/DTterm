//! 这是一个字体管理模块
#![allow(dead_code)]
#![allow(unused_variables)]

use std::{ffi::CString, hash::Hash, num::NonZeroUsize, rc::Rc};

use font_kit::{
    family_name::FamilyName,
    handle::Handle,
    properties::{Properties, Style},
    source::SystemSource,
};
use raylib::ffi::LoadFontEx;

#[derive(PartialEq, Hash, Debug)]
pub struct FontKey {
    pub font_name: String,
    pub size: i32,
    pub style: Style,
}
impl Eq for FontKey {}

#[derive()]
pub struct FontCache {
    cache: lru::LruCache<FontKey, Rc<raylib::ffi::Font>>,
    system_source: SystemSource,
}

impl FontCache {
    pub fn new() -> Self {
        Self {
            cache: lru::LruCache::new(NonZeroUsize::new(1024).unwrap()),
            system_source: SystemSource::new(),
        }
    }

    pub fn get_font(
        &mut self,
        font_name: String,
        style: Style,
        size: i32,
    ) -> Option<Rc<raylib::ffi::Font>> {
        self.cache
            .get(&FontKey {
                font_name,
                size,
                style,
            })
            .cloned()
    }

    pub fn add_font(
        &mut self,
        font_name: String,
        size: i32,
        style: Style,
        font: raylib::ffi::Font,
    ) -> Rc<raylib::ffi::Font> {
        let font = Rc::new(font);
        self.cache.put(
            FontKey {
                font_name,
                size,
                style,
            },
            font.clone(),
        );
        font
    }

    pub fn search_system_font(
        &mut self,
        font_name: String,
        style: Style,
        font_size: i32,
    ) -> Result<Rc<raylib::ffi::Font>, String> {
        match self.get_font(font_name.to_owned(), style, font_size) {
            Some(t) => {
                return Ok(t);
            }
            _ => {
                let resutl = self.system_source.select_best_match(
                    &[FamilyName::Title(font_name.to_owned())],
                    Properties::new().style(style),
                );
                match resutl {
                    Ok(e) => match e {
                        Handle::Path { path, font_index } => {
                            let font_path_str = path.to_str().unwrap();
                            let c_font_name = CString::new(font_path_str).unwrap();
                            let font = unsafe {
                                LoadFontEx(c_font_name.as_ptr(), font_size, std::ptr::null_mut(), 0)
                            };
                            Ok(self.add_font(font_name.to_owned(), font_size, style, font))
                        }
                        _ => Err("failed font path with font-kit".to_string()),
                    },
                    _ => Err("failed find font with font-kit".to_string()),
                }
            }
        }
    }

    pub fn load_all_system_font(&self) -> Vec<String> {
        self.system_source.all_families().unwrap()
    }
}

#[cfg(test)]
mod test {
    use std::time::Instant;

    use font_kit::properties::Style;

    use crate::window::Window;

    use super::FontCache;

    fn test_fand(font: &mut FontCache, font_name: &str, style: Style, font_size: i32) {
        let start = Instant::now();
        match font.search_system_font(font_name.to_owned(), style, font_size) {
            Ok(_) => {
                println!("查询成功");
            }
            Err(e) => {
                println!("查询失败 {}", e);
            }
        };
        println!("Function took: {}", start.elapsed().as_nanos());
    }
    #[test]
    #[ignore = "find_font"]
    fn test_search_font() {
        let win = Window::new(100, 100);
        let mut font = FontCache::new();

        for i in font.load_all_system_font() {
            test_fand(&mut font, i.as_str(), Style::Normal, 20);
        }
        test_fand(&mut font, "MesloLGL Nerd Font", Style::Italic, 20);
        test_fand(&mut font, "MesloLGL Nerd Font", Style::Oblique, 20);
        test_fand(&mut font, "MesloLGL Nerd Font", Style::Normal, 20);

        for i in 0..1024 {
            test_fand(&mut font, "MesloLGL Nerd Font", Style::Normal, 20);
        }
        font.get_font("MesloLGL Nerd Font".to_owned(), Style::Normal, 20)
            .unwrap();

        win.join();
    }
}
