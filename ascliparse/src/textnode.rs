#![allow(dead_code)]
#![allow(unused_variables)]

use raylib::color::Color;

#[derive(Debug, Clone)]
pub struct FontOption {
    pub color: Color, // 文本颜色
    pub height: f32,  // 高度
    pub width: f32,   // 宽度
    pub space: f32,   // 字符分隔
    pub font: String, // 使用的字体
    pub c_size: f32,  // 缓存每个字符的宽度
}

impl Default for FontOption {
    fn default() -> Self {
        Self {
            color: Color::WHEAT,
            height: 0.0,
            width: 0.0,
            space: 0.0,
            font: "".to_owned(),
            c_size: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TextNode {
    pub chars: Vec<u8>, // 节点文本内容
    pub option: FontOption,
}

impl Default for TextNode {
    fn default() -> Self {
        Self {
            chars: Vec::new(),
            option: FontOption::default(),
        }
    }
}

impl TextNode {
    pub fn push(&mut self, c: &u8) {
        self.chars.push(*c)
    }
    pub fn new() -> Self {
        Self::default()
    }
}
