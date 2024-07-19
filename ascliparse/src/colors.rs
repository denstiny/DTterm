#![allow(dead_code)]
use lazy_static::*;

struct Rgb {
    r: u8,
    g: u8,
    b: u8,
    m: u8,
}

impl Rgb {
    fn new(r: u8, g: u8, b: u8, m: u8) -> Self {
        Self { r, g, b, m }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AnsiColor {
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
    BrightBlack = 8,
    BrightRed = 9,
    BrightGreen = 10,
    BrightYellow = 11,
    BrightBlue = 12,
    BrightMagenta = 13,
    BrightCyan = 14,
    BrightWhite = 15,
}

lazy_static! {
    static ref ANSICOLOR: Vec<Rgb> = vec![
        // 普通色彩
        Rgb::new(0, 0, 0, 29),
        Rgb::new(255, 0, 0, 29),
        Rgb::new(0, 255, 0, 29),
        Rgb::new(255, 255, 0, 29),
        Rgb::new(0, 0, 255, 29),
        Rgb::new(255, 0, 255, 29),
        Rgb::new(0, 255, 255, 29),
        Rgb::new(255, 255, 255, 29),
        // 亮色
        Rgb::new(127, 127, 127, 100),
        Rgb::new(255, 127, 127, 100),
        Rgb::new(127, 255, 127, 100),
        Rgb::new(255, 255, 127, 100),
        Rgb::new(127, 127, 255, 100),
        Rgb::new(255, 127, 255, 100),
        Rgb::new(127, 255, 255, 100),
        Rgb::new(255, 255, 255, 100),
    ];
}
