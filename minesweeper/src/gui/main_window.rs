use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

extern crate orbimage;
extern crate orbtk;

use orbtk::traits::{Click, Place, Text};
use orbtk::{
    Action, Button, Color, ComboBox, Grid, Image, Label, Point, ProgressBar, Rect, TextBox, Window,
};

use crate::logic::game::{Game, GameLevel};
use crate::logic::table::FieldState;

struct MainWindowInner {
    game: Game,
    grid: Arc<Grid>,
}

impl MainWindowInner {
    fn new() -> Self {
        MainWindowInner {
            game: Game::new(GameLevel::Beginner),
            grid: Grid::new(),
        }
    }
}

pub struct MainWindow {
    inner: Rc<RefCell<MainWindowInner>>,
}

impl MainWindow {
    pub fn new() -> Self {
        MainWindow {
            inner: Rc::new(RefCell::new(MainWindowInner::new())),
        }
    }

    pub fn show(&self) {
        let mut window = Window::new(Rect::new(100, 100, 420, 768), "Minesweeper");
        {
            let inner = self.inner.borrow();

            window.add(&inner.grid);
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

            inner.grid.position(x, y).spacing(1, 1);

            Self::update_grid(self.inner.clone());
        }

        window.exec();
    }

    fn update_grid(inner_param: Rc<RefCell<MainWindowInner>>) {
        let inner = inner_param.borrow();
        let width = inner.game.get_width();
        let height = inner.game.get_height();
        let grid = inner.grid.clone();

        for row_index in 0..height {
            for col_index in 0..width {
                let state = inner.game.get_field_state(row_index, col_index).unwrap();
                let cell = Self::create_cell(inner_param.clone(), state, row_index, col_index);
                grid.insert(col_index, row_index, &cell);
            }
        }
    }

    fn create_cell(
        inner_param: Rc<RefCell<MainWindowInner>>,
        state: FieldState,
        x: usize,
        y: usize,
    ) -> Arc<Image> {
        let inner_clone = inner_param.clone();
        let inner = inner_param.borrow();
        let color = match state {
            Closed => Color::rgb(0, 0, 255),
            Opened => Color::rgb(0, 255, 0),
            Flagged => Color::rgb(255, 0, 0),
        };
        let cell = Image::from_color(16, 16, color);

        cell.on_click(move |cell, _point| {
            inner_clone.borrow_mut().game.open(x, y);
            println!("Clicked {} {}!", x, y);
            MainWindow::update_grid(inner_clone.clone());
        });
        return cell;
    }
}
