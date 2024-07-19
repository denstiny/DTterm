#![allow(dead_code)]
#![allow(unused_variables)]

use crate::textnode::TextNode;
use anstyle_parse::{DefaultCharAccumulator, Parser, Perform};

pub struct RawHandle<'a> {
    nodes: &'a Vec<TextNode>,
}

impl<'a> Perform for RawHandle<'a> {
    fn put(&mut self, _byte: u8) {}
    fn hook(
        &mut self,
        _params: &anstyle_parse::Params,
        _intermediates: &[u8],
        _ignore: bool,
        _action: u8,
    ) {
    }
    fn print(&mut self, _c: char) {}
    fn unhook(&mut self) {}
    fn execute(&mut self, _byte: u8) {}
}

#[derive(Debug)]
pub struct AscliInstructionParer(Parser);

impl AscliInstructionParer {
    pub fn new() -> Self {
        Self(Parser::<DefaultCharAccumulator>::new())
    }

    pub fn handle(&mut self, nodes: &mut Vec<TextNode>, chars: Vec<u8>) {
        let mut buffer = RawHandle { nodes };
        for char in chars {
            self.0.advance(&mut buffer, char)
        }
        return;
    }
}
