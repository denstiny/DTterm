//! 终端输出暂存
#![allow(dead_code)]
#![allow(unused_variables)]

use ascliparse::{ascliparse::Ascliparse, textnode::TextNode};

#[derive(Debug)]
pub struct Buffer {
    cache: Vec<TextNode>,
    cur_node: (i32, i32), // 当前索引节点
    max_line: u32,        // 保存的最大行数
}

impl Ascliparse for Buffer {}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            cache: Vec::new(),
            cur_node: (-1, -1),
            max_line: 100,
        }
    }

    pub fn put(&mut self, c: Vec<u8>) {
        Self::channel(&mut self.cache, c);
    }

    pub fn get(&self) -> Vec<TextNode> {
        self.cache.clone()
    }
}
