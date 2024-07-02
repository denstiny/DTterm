#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

pub const PARENT: u8 = b'\x1b'; // 指令前缀
pub const BEGIN: u8 = b'[';
pub const END: u8 = b']';

#[derive(PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum AscliInstruction {
    // -- 前置符号
    PARENT = b'\x1b', // 机灵前缀
    BEGIN = b'[',     // 设置样式
    END = b']',       // 指令开始
    PRIVATE = b'?',   // 私有模式

    // --后置指令描述符号
    COLOR = b'm',     // 设置颜色
    MoveUp = b'A',    // 向上移动
    MoveDown = b'B',  // 向下移动
    MoveRight = b'C', // 向右移动
    MoveLeft = b'D',  // 向左移动

    VAIN, // 从u8 转换成 enum的错误值
    TEXT, // 实际内容
}

impl From<u8> for AscliInstruction {
    fn from(value: u8) -> Self {
        match value {
            b'\x1b' => AscliInstruction::PARENT,
            b'[' => AscliInstruction::BEGIN,
            b']' => AscliInstruction::END,
            b'?' => AscliInstruction::PRIVATE,
            b'm' => AscliInstruction::COLOR,
            b'A' => AscliInstruction::MoveUp,
            b'B' => AscliInstruction::MoveDown,
            b'C' => AscliInstruction::MoveRight,
            b'D' => AscliInstruction::MoveLeft,
            _ => AscliInstruction::VAIN,
        }
    }
}
