mod logic;

extern crate orbimage;
extern crate orbtk;

use orbtk::traits::{Click, Place, Text};
use orbtk::{
    Action, Button, Color, ComboBox, Grid, Image, Label, Point, ProgressBar, Rect, TextBox, Window,
};

use logic::game::{Game, GameLevel};

fn main() {
    let mut game = Game::new(GameLevel::Beginner);
    game.open(1, 1).unwrap();
    game.toggle_flag(2, 1).unwrap();
    let mut window = Window::new(Rect::new(100, 100, 420, 768), "Minesweeper");

    let x = 10;
    let mut y = 0;

    let button = Button::new();
    button
        .position(x, y)
        .size(15, 15)
        .text("U")
        .text_offset(6, 6);
    window.add(&button);

    y = y + button.rect.get().height as i32;

    let grid = Grid::new();
    grid.position(x, y).spacing(1, 1);

    for row in 0..7 {
        for col in 0..7 {
            let cell = Image::from_color(16, 16, Color::rgb(255, 0, 0));
            cell.on_click(move |image, _point| {
                let r = row;
                let c = col;
                println!("Clicked {} {}!", r, c);
                image.image.replace_with(|_old_image| -> orbimage::Image {
                    orbimage::Image::from_color(16, 16, Color::rgb(0, 255, 0))
                });
            });
            grid.insert(col, row, &cell);
        }
    }
    grid.arrange(true);

    window.add(&grid);

    window.exec();
}
