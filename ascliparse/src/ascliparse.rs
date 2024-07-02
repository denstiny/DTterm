#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{instruction::AscliInstruction, param_parse::PramParse, textnode::TextNode};

pub trait Ascliparse {
    fn channel(nodes: &mut Vec<TextNode>, chars: Vec<u8>) {
        let mut instant = AscliInstruction::TEXT;
        let mut param = PramParse::new();
        let mut stack_state: Vec<TextNode> = Vec::new();
        println!("原始: {:?}", chars.to_owned());

        for i in chars {
            if i == AscliInstruction::PARENT as u8 {
                instant = AscliInstruction::PARENT
            } else {
                match instant {
                    AscliInstruction::PARENT => {
                        instant = AscliInstruction::from(i);
                        stack_state.push(TextNode::new());
                    }
                    AscliInstruction::BEGIN => {
                        if let Some(arg) = param.parse(i) {
                            match param.get_type() {
                                AscliInstruction::COLOR => {
                                    instant = AscliInstruction::TEXT;
                                }
                                _ => {
                                    //for v in arg {
                                    //    println!("{:?} {}", param.get_type(), v);
                                    //}
                                    instant = AscliInstruction::TEXT
                                }
                            };
                        }
                    }
                    AscliInstruction::END => instant = AscliInstruction::TEXT,
                    AscliInstruction::TEXT | AscliInstruction::VAIN => {
                        print!("{}", i as char);
                        if let Some(node) = stack_state.last_mut() {
                            node.chars.push(i);
                            if i == b'\n' {
                                stack_state.push(TextNode::new());
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        let mut new_nodes = stack_state
            .iter()
            .filter(|&x| !x.chars.is_empty())
            .cloned()
            .collect::<Vec<TextNode>>();
        //println!("结果: {:?}", new_nodes);
        nodes.append(&mut new_nodes);
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
