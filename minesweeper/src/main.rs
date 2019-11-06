mod gui;
mod logic;

use gui::main_window::MainWindow;

fn main() {
    let mw = MainWindow::new();
    mw.show();
}
