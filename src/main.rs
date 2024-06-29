mod buffer;
mod font;
mod pty;
mod utils;
mod window;

use window::Window;

fn main() {
    let win = Window::new(1600, 600);
    win.join();
}
