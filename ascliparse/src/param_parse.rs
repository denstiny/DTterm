#![allow(dead_code)]

use core::fmt;
use std::error::Error;

use crate::instruction::AscliInstruction;

#[derive(Clone, Debug)]
pub struct Param {
    pub param: Vec<u8>,
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.to_i32() {
            Ok(v) => {
                write!(f, "{}", v)
            }
            Err(e) => {
                write!(
                    f,
                    "Error {} => {}",
                    self.to_string().unwrap(),
                    e.to_string()
                )
            }
        }
    }
}

impl Param {
    fn to_i32(&self) -> Result<i32, Box<dyn Error>> {
        let parse_str = String::from_utf8(self.param.clone())?;
        parse_str
            .parse()
            .map_err(|e: std::num::ParseIntError| e.into())
    }

    fn to_string(&self) -> Result<String, Box<dyn Error>> {
        Ok(String::from_utf8(self.param.clone())?)
    }
}

#[derive(Clone, Debug)]
pub struct PramParse {
    value: Vec<Param>,
    pub end: u8,
    is_private: bool,
}

impl PramParse {
    pub fn new() -> Self {
        Self {
            value: Vec::new(),
            is_private: false,
            end: 0,
        }
    }

    pub fn get_param(&self) -> Vec<Param> {
        self.value.clone()
    }

    pub fn get_type(&self) -> AscliInstruction {
        if self.is_private {
            AscliInstruction::PRIVATE
        } else {
            AscliInstruction::from(self.end)
        }
    }

    /// 解析指令参数
    ///
    /// * `value`: 指令ascli字符
    pub fn parse(&mut self, value: u8) -> Option<Vec<Param>> {
        let rtype = AscliInstruction::from(value);
        if rtype == AscliInstruction::PRIVATE {
            self.is_private = true;
            Option::None
        } else if rtype != AscliInstruction::VAIN {
            let value_clone = self.value.clone();
            self.end = value;
            self.value.clear();
            return Some(value_clone);
        } else {
            if self.value.is_empty() {
                self.value.push(Param { param: Vec::new() });
            }
            if self.is_private && (value >= b'A' && value <= b'z') {
                // 隐私模式的特殊处理
                let value_clone = self.value.clone();
                self.end = value;
                self.value.clear();
                return Some(value_clone);
            }

            if value == b';' {
                self.value.push(Param { param: Vec::new() })
            } else {
                if let Some(vn) = self.value.last_mut() {
                    vn.param.push(value);
                }
            }
            Option::None
        }
    }
}
