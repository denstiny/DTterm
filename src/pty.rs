#![allow(dead_code)]
#![allow(unused_variables)]
use std::{
    io::{Read, Write},
    sync::{Arc, Mutex},
    thread,
};

use ascliparse::textnode::TextNode;
use portable_pty::{CommandBuilder, MasterPty, PtySize};

use crate::buffer::Buffer;

pub struct Pty {
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
    reader: Arc<Mutex<Box<dyn Read + Send>>>,
    master: Arc<Mutex<Box<dyn MasterPty + Send>>>,
    text_buffer: Arc<Mutex<Buffer>>,
}

impl Pty {
    pub fn new(shell: &str, rows: u16, cols: u16) -> Self {
        let pty = portable_pty::native_pty_system();
        let terminal = pty
            .openpty(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0, // put your terminal dimension I change them later by a window resize event
            })
            .unwrap();

        let cmd = CommandBuilder::new(shell);
        let child = terminal.slave.spawn_command(cmd).unwrap();
        drop(terminal.slave);

        let writer = Arc::new(Mutex::new(terminal.master.take_writer().unwrap()));
        let reader = Arc::new(Mutex::new(terminal.master.try_clone_reader().unwrap()));
        let master = Arc::new(Mutex::new(terminal.master));

        Self {
            writer,
            reader,
            master,
            text_buffer: Arc::new(Mutex::new(Buffer::new())),
        }
    }

    pub fn start_reader(self) -> Self {
        let reader = self.reader.clone();
        let text_buffer = self.text_buffer.clone();
        thread::spawn(move || {
            let mut buffer: Vec<u8> = vec![0u8; 4013];
            loop {
                match reader.lock().unwrap().read(&mut buffer) {
                    Ok(size) => {
                        //if let Ok(message) = String::from_utf8(buffer[..size].to_vec()) {
                        //    text_buffer.lock().unwrap().put(message.clone());
                        //    print!("{}", message);
                        //}
                        text_buffer.lock().unwrap().put(buffer[..size].to_owned());
                    }
                    Err(e) => {
                        println!("{}", e.to_string());
                    }
                };
            }
        });
        self
    }

    pub fn send(&self, buf: Vec<u8>) {
        match self.writer.lock().unwrap().write(&buf) {
            _ => {}
        }
    }

    pub fn set_window_size(&mut self, size: PtySize) {
        match self.master.lock().unwrap().resize(size) {
            _ => {}
        }
    }

    pub fn get_buffer(&self) -> Vec<TextNode> {
        self.text_buffer.lock().unwrap().get()
    }
}

#[cfg(test)]
mod test {

    use std::{io::Read, thread};

    use super::Pty;

    #[test]
    #[ignore = "test_shell"]
    fn test_pty_shell() {
        let pty = Pty::new("zsh", 40, 100);
        let reader = pty.reader.clone();
        thread::spawn(move || loop {
            let mut buffer: Vec<u8> = vec![0u8; 1];
            match reader.lock().unwrap().read(&mut buffer) {
                Ok(size) => {
                    if let Ok(message) = String::from_utf8(buffer[..size].to_vec()) {
                        print!("{}", message);
                    }
                }
                Err(e) => {
                    println!("{}", e.to_string());
                }
            };
        });
        loop {
            let mut buf = vec![0u8; 1023];
            let size = std::io::stdin().read(&mut buf).unwrap();
            pty.writer.lock().unwrap().write(&buf[..size]).unwrap();
        }
    }
}
