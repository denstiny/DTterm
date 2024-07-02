#![allow(dead_code)]
#![allow(unused_variables)]
use std::{
    collections::HashMap,
    io::{Read, Write},
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

use ascliparse::textnode::TextNode;
use portable_pty::{Child, CommandBuilder, MasterPty, PtySize};

use crate::buffer::Buffer;

#[derive(PartialEq, Eq, Hash)]
pub enum PtySignal {
    NewLine,
}

pub struct Pty {
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
    reader: Arc<Mutex<Box<dyn Read + Send>>>,
    master: Arc<Mutex<Box<dyn MasterPty + Send>>>,
    child: Arc<Mutex<Box<dyn Child + Send + Sync>>>,
    text_buffer: Arc<Mutex<Buffer>>,
    _task_map: Arc<Mutex<HashMap<PtySignal, Vec<Box<dyn FnOnce() + Send>>>>>,
    task_sender: Arc<Mutex<Sender<PtySignal>>>,
    task_receiver: Arc<Mutex<Receiver<PtySignal>>>,
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
        let child = Arc::new(Mutex::new(terminal.slave.spawn_command(cmd).unwrap()));
        drop(terminal.slave);

        let writer = Arc::new(Mutex::new(terminal.master.take_writer().unwrap()));
        let reader = Arc::new(Mutex::new(terminal.master.try_clone_reader().unwrap()));
        let master = Arc::new(Mutex::new(terminal.master));

        let (tx, rx) = mpsc::channel();
        Self {
            writer,
            reader,
            master,
            child,
            text_buffer: Arc::new(Mutex::new(Buffer::new())),
            _task_map: Arc::new(Mutex::new(HashMap::new())),
            task_sender: Arc::new(Mutex::new(tx)),
            task_receiver: Arc::new(Mutex::new(rx)),
        }
    }

    pub fn start_reader(self) -> Self {
        let reader = self.reader.clone();
        let text_buffer = self.text_buffer.clone();
        let child = self.child.clone();
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
                        //println!("进程状态: {:?}", child.lock().unwrap().process_id());
                    }
                    Err(e) => {
                        println!("{}", e.to_string());
                    }
                };
            }
        });

        let task_channel = self.task_receiver.clone();
        let task_map = self._task_map.clone();
        thread::spawn(move || {
            let chan = task_channel.lock().unwrap();
            loop {
                match chan.recv() {
                    Ok(msg) => {
                        if let Some(vtask) = task_map.lock().unwrap().get_mut(&msg) {
                            while let Some(task) = vtask.pop() {
                                task()
                            }
                        }
                    }
                    Err(e) => {
                        return;
                    }
                }
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

    pub fn with_signal(&self, signal: PtySignal, callback: Box<dyn FnOnce() + Send>) {
        if let Some(node) = self._task_map.lock().unwrap().get_mut(&signal) {
            node.push(callback);
            return;
        }
        let mut tasks = Vec::new();
        tasks.push(callback);
        self._task_map.lock().unwrap().insert(signal, tasks);
    }

    pub fn emit_signal(&self, signal: PtySignal) {
        let _ = self.task_sender.lock().unwrap().send(signal);
    }
}

#[cfg(test)]
mod test {

    use std::{io::Read, thread};

    use super::{Pty, PtySignal};

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

    #[test]
    fn test_with_signal() {
        let pty = Pty::new("bash", 0, 0).start_reader();
        pty.with_signal(
            PtySignal::NewLine,
            Box::new(|| {
                println!("new line");
            }),
        );
        pty.emit_signal(PtySignal::NewLine)
    }
}
