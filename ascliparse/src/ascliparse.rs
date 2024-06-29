#![allow(dead_code)]
#![allow(unused_variables)]

use std::u8;

use crate::{instruction::AscliInstruction, textnode::TextNode};

pub trait Ascliparse {
    fn channel(nodes: &mut Vec<TextNode>, cursor: &mut (i32, i32), chars: Vec<u8>) {
        let mut result: Vec<u8> = Vec::new();
        let mut instant = AscliInstruction::TEXT;
        let mut param: Vec<u8> = Vec::new();
        let mut stack_state: Vec<TextNode> = Vec::new();
        stack_state.push(TextNode::new());
        println!("原始: {:?}", chars.to_owned());

        for i in chars {
            if i == AscliInstruction::PARENT as u8 {
                instant = AscliInstruction::PARENT;
            } else if instant == AscliInstruction::PARENT {
                instant = AscliInstruction::from(i);
            } else if instant == AscliInstruction::BEGIN {
                if i == AscliInstruction::COLOR as u8 {
                    //println!("param -> {}", String::from_utf8(param.to_owned()).unwrap());
                    param.clear();
                    instant = AscliInstruction::TEXT
                } else if i == AscliInstruction::MoveRight as u8 {
                    param.clear();
                } else if i == AscliInstruction::MoveLeft as u8 {
                    param.clear();
                } else if i == AscliInstruction::MoveUp as u8 {
                    param.clear();
                } else if i == AscliInstruction::MoveDown as u8 {
                    param.clear();
                } else if i == AscliInstruction::PRIVATE as u8 {
                    param.clear();
                    instant = AscliInstruction::PRIVATE
                } else {
                    param.push(i);
                }
            } else if instant == AscliInstruction::PRIVATE {
                if i == b'l' {
                    println!("param -> {}", String::from_utf8(param.to_owned()).unwrap());
                } else if i == b'h' {
                    println!("param -> {}", String::from_utf8(param.to_owned()).unwrap());
                } else {
                    param.push(i)
                }
            } else if instant == AscliInstruction::END {
                instant = AscliInstruction::TEXT
            } else {
                result.push(i);
            }
        }

        println!("result {}", String::from_utf8(result).unwrap());
    }
}

#[cfg(test)]
mod test {
    use crate::instruction::AscliInstruction;

    #[test]
    fn test_shell_word() {
        // 移动光标到 (10, 10) 的位置
        print!("\x1b[10;10H");

        // 保存当前光标位置
        print!("\x1b7");

        // 打印一些信息
        println!("This is the saved cursor position.");

        // 恢复之前保存的光标位置
        print!("\x1b8");

        // 继续输出其他内容
        println!("This is after restoring the cursor position.");
    }

    #[test]
    fn test_ascli_instruction_eq() {
        let instant = AscliInstruction::PRIVATE;
        assert!(instant == AscliInstruction::PRIVATE)
    }
}
