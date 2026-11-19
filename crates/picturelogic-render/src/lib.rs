use std::{fs, io, path::PathBuf};

use picturelogic_core::{Book, Cell, PUZZLES};
use primitive_app::{App, Change};
use primitive_ui::{
    back_button, header, screen_rect, Canvas, Input, Rect, TextStyle, BLACK, BODY, CAPTION,
    HAIRLINE, PAPER,
};

const LEFT: i32 = 176;
const TOP: i32 = 365;
const SIDE: i32 = 1040;
const SMALL: TextStyle = TextStyle::new(26).bold();

pub struct View {
    pub book: Book,
    path: Option<PathBuf>,
    tool: Cell,
    reset: bool,
    save_failed: bool,
}

impl View {
    pub fn open(path: PathBuf) -> Self {
        let book = fs::read(&path)
            .ok()
            .and_then(|bytes| serde_json::from_slice::<Book>(&bytes).ok())
            .unwrap_or_default()
            .restored();
        Self::new(book, Some(path))
    }

    pub fn preview() -> Self {
        Self::new(Book::default(), None)
    }

    fn new(book: Book, path: Option<PathBuf>) -> Self {
        Self {
            book,
            path,
            tool: Cell::Filled,
            reset: false,
            save_failed: false,
        }
    }

    fn persist(&mut self) {
        self.save_failed = self.save().is_err();
    }

    fn page(&mut self, forward: bool) -> Change {
        let index = self.book.current();
        let next = if forward {
            index + 1
        } else {
            index.saturating_sub(1)
        };
        if self.book.go(next) {
            self.persist();
            Change::Full
        } else {
            Change::None
        }
    }

    fn tap(&mut self, x: f32, y: f32) -> Change {
        let width = primitive_app::WIDTH as i32;
        if self.reset {
            self.reset = false;
            if reset_confirm(width).contains(x, y) {
                self.book.clear();
                self.persist();
            }
            return Change::Full;
        }
        if back_button(width).contains(x, y) {
            return Change::Exit;
        }
        if fill(width).contains(x, y) {
            self.tool = Cell::Filled;
            return Change::Partial;
        }
        if mark(width).contains(x, y) {
            self.tool = Cell::Marked;
            return Change::Partial;
        }
        if previous(width).contains(x, y) {
            return self.page(false);
        }
        if next(width).contains(x, y) {
            return self.page(true);
        }
        if reset(width).contains(x, y) {
            self.reset = true;
            return Change::Full;
        }
        let board = screen_rect(width, LEFT, TOP, SIDE, SIDE);
        if !self.book.solved() && board.contains(x, y) {
            let size = self.book.puzzle().size();
            let col = ((x - board.x as f32) * size as f32 / board.w as f32) as usize;
            let row = ((y - board.y as f32) * size as f32 / board.h as f32) as usize;
            if self.book.toggle(col, row, self.tool) {
                self.persist();
                return Change::Partial;
            }
        }
        Change::None
    }

    fn draw_board(&self, canvas: &mut Canvas, width: i32) {
        let puzzle = self.book.puzzle();
        let size = puzzle.size();
        let unit = SIDE / size as i32;
        let page = self.book.page();
        let active = !self.reset && !self.book.solved();
        for (y, row) in page.iter().enumerate().take(size) {
            for (x, &cell) in row.iter().enumerate().take(size) {
                let bounds = screen_rect(
                    width,
                    LEFT + x as i32 * unit,
                    TOP + y as i32 * unit,
                    unit,
                    unit,
                );
                let pressed = active && canvas.control_pressed(bounds, false);
                let ink = if pressed { PAPER } else { BLACK };
                let paper = if pressed { BLACK } else { PAPER };
                let inset = if size == 10 { 3 } else { 4 };
                let edge = canvas.px(inset).max(1);
                let shade = if cell == Cell::Filled { ink } else { paper };
                canvas.fill(bounds.inset(edge), shade);
                if cell == Cell::Marked {
                    canvas.text(
                        bounds.inset(canvas.px(unit / 5)),
                        "×",
                        BODY.bold().center().ink(ink),
                    );
                }
            }
        }
        for index in 0..=size {
            let offset = index as i32 * unit;
            let major = index == 0 || index == size || index % 5 == 0;
            let line = if major { 4 } else { 2 };
            let ink = if major { BLACK } else { HAIRLINE };
            canvas.fill(screen_rect(width, LEFT + offset, TOP, line, SIDE + 4), ink);
            canvas.fill(screen_rect(width, LEFT, TOP + offset, SIDE + 4, line), ink);
        }
        for index in 0..size {
            let numbers = |column| {
                let clues = puzzle.clues(index, column);
                if clues.is_empty() {
                    vec!["0".into()]
                } else {
                    clues.iter().map(usize::to_string).collect()
                }
            };
            let row_style = if self.book.line_matches(index, false) {
                CAPTION.right()
            } else {
                SMALL.right()
            };
            let y = TOP + index as i32 * unit + (unit - 34) / 2;
            canvas.text(
                screen_rect(width, 15, y, LEFT - 29, 42),
                &numbers(false).join(" "),
                row_style,
            );
            let col_style = if self.book.line_matches(index, true) {
                CAPTION.center()
            } else {
                SMALL.center()
            };
            for (i, number) in numbers(true).iter().rev().enumerate() {
                let x = LEFT + index as i32 * unit;
                canvas.text(
                    screen_rect(width, x, TOP - 36 - i as i32 * 30, unit, 31),
                    number,
                    col_style,
                );
            }
        }
    }
}

impl App for View {
    fn draw(&self, canvas: &mut Canvas) {
        let width = canvas.bounds().w;
        let puzzle = self.book.puzzle();
        header(canvas, "Picture Logic", "Done");
        canvas.text(
            screen_rect(width, 40, 155, 620, 54),
            &format!(
                "Puzzle {} of {}  ·  {} × {}",
                self.book.current() + 1,
                PUZZLES.len(),
                puzzle.size(),
                puzzle.size()
            ),
            BODY.bold(),
        );
        canvas.text(
            screen_rect(width, 665, 161, 554, 46),
            &format!("{} solved", self.book.solved_count()),
            CAPTION.right(),
        );
        self.draw_board(canvas, width);
        let message = if self.save_failed {
            "Could not save progress. Check the Kindle storage.".into()
        } else if self.book.solved() {
            format!("Solved · {}", puzzle.title)
        } else {
            "Match each line to its runs of filled squares.".into()
        };
        canvas.text(
            screen_rect(width, 40, 1419, 1184, 40),
            &message,
            CAPTION.center(),
        );
        canvas.button(fill(width), "Fill", self.tool == Cell::Filled, false);
        canvas.button(mark(width), "Mark ×", self.tool == Cell::Marked, false);
        if self.book.current() > 0 {
            canvas.button(previous(width), "Previous", false, false);
        }
        canvas.button(reset(width), "Reset", false, false);
        if self.book.current() + 1 < PUZZLES.len() {
            canvas.button(next(width), "Next", self.book.solved(), false);
        }
        if self.reset {
            canvas.popup(screen_rect(width, 190, 590, 884, 350));
            canvas.text(
                screen_rect(width, 240, 630, 790, 66),
                "Reset this puzzle?",
                TextStyle::new(50).bold(),
            );
            canvas.text(
                screen_rect(width, 240, 717, 790, 50),
                "Clear the squares on this page.",
                CAPTION,
            );
            canvas.button(
                screen_rect(width, 240, 802, 365, 100),
                "Cancel",
                false,
                false,
            );
            canvas.button(reset_confirm(width), "Reset", true, false);
        }
    }

    fn input(&mut self, input: Input) -> Change {
        match input {
            Input::Tap(x, y) => self.tap(x, y),
            Input::PageBack if self.reset => {
                self.reset = false;
                Change::Full
            }
            Input::PageBack => self.page(false),
            Input::PageForward if !self.reset => self.page(true),
            _ => Change::None,
        }
    }

    fn save(&mut self) -> io::Result<()> {
        if let Some(path) = &self.path {
            primitive_app::save(path, &serde_json::to_vec(&self.book)?)?;
        }
        Ok(())
    }
}

fn fill(width: i32) -> Rect {
    screen_rect(width, 40, 1467, 575, 100)
}
fn mark(width: i32) -> Rect {
    screen_rect(width, 649, 1467, 575, 100)
}
fn previous(width: i32) -> Rect {
    screen_rect(width, 40, 1575, 365, 100)
}
fn reset(width: i32) -> Rect {
    screen_rect(width, 449, 1575, 365, 100)
}
fn next(width: i32) -> Rect {
    screen_rect(width, 859, 1575, 365, 100)
}
fn reset_confirm(width: i32) -> Rect {
    screen_rect(width, 657, 802, 365, 100)
}
