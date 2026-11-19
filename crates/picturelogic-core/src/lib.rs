use serde::{Deserialize, Serialize};

pub const MAX_SIZE: usize = 10;

#[derive(Clone, Copy)]
pub struct Puzzle {
    pub title: &'static str,
    rows: &'static [&'static str],
}

impl Puzzle {
    pub fn size(&self) -> usize {
        self.rows.len()
    }

    pub fn filled(&self, x: usize, y: usize) -> bool {
        self.rows[y].as_bytes()[x] == b'#'
    }

    pub fn clues(&self, index: usize, column: bool) -> Vec<usize> {
        runs((0..self.size()).map(|offset| {
            if column {
                self.filled(index, offset)
            } else {
                self.filled(offset, index)
            }
        }))
    }
}

fn runs(cells: impl IntoIterator<Item = bool>) -> Vec<usize> {
    let mut counts = Vec::new();
    let mut run = 0;
    for filled in cells.into_iter().chain([false]) {
        if filled {
            run += 1;
        } else if run != 0 {
            counts.push(run);
            run = 0;
        }
    }
    counts
}

pub const PUZZLES: &[Puzzle] = &[
    Puzzle {
        title: "Diamond",
        rows: &["..#..", ".###.", "#####", ".###.", "..#.."],
    },
    Puzzle {
        title: "Anchor",
        rows: &["..#..", ".###.", "..#..", "#.#.#", ".###."],
    },
    Puzzle {
        title: "Crown",
        rows: &["#.#.#", "#####", ".###.", ".###.", "#####"],
    },
    Puzzle {
        title: "Arrow",
        rows: &["..#..", ".###.", "#####", "..#..", "..#.."],
    },
    Puzzle {
        title: "Hourglass",
        rows: &["#####", ".#.#.", "..#..", ".#.#.", "#####"],
    },
    Puzzle {
        title: "Key",
        rows: &[".###.", ".#.#.", ".###.", "..#..", "..###"],
    },
    Puzzle {
        title: "Rocket",
        rows: &[
            "....##....",
            "...####...",
            "...####...",
            "...#..#...",
            "...#..#...",
            "...####...",
            "..######..",
            ".########.",
            ".##.##.##.",
            "....##....",
        ],
    },
    Puzzle {
        title: "Lighthouse",
        rows: &[
            "....##....",
            "...####...",
            "##########",
            "...####...",
            "...####...",
            "...#..#...",
            "...####...",
            "..######..",
            "..######..",
            "##########",
        ],
    },
    Puzzle {
        title: "Sailboat",
        rows: &[
            "....#.....",
            "....##....",
            "....###...",
            "....####..",
            "....#####.",
            "....######",
            "..#.#.....",
            "##########",
            ".########.",
            "..######..",
        ],
    },
    Puzzle {
        title: "Mountain pass",
        rows: &[
            "......##..",
            "..#...###.",
            ".###.#####",
            "##########",
            "##########",
            "##########",
            "####.#####",
            "###..#####",
            "###...####",
            "##.....###",
        ],
    },
    Puzzle {
        title: "Desk lamp",
        rows: &[
            "....#.###.",
            ".....#####",
            "....#####.",
            ".....###..",
            ".....##...",
            "....##....",
            "...##.....",
            "..##......",
            "..##......",
            "########..",
        ],
    },
    Puzzle {
        title: "Envelope",
        rows: &[
            "..........",
            "##########",
            "##......##",
            "###....###",
            "####..####",
            "##########",
            "#........#",
            "#........#",
            "##########",
            "..........",
        ],
    },
];

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum Cell {
    #[default]
    Unknown,
    Filled,
    Marked,
}

type Grid = [[Cell; MAX_SIZE]; MAX_SIZE];
const EMPTY: Grid = [[Cell::Unknown; MAX_SIZE]; MAX_SIZE];

#[derive(Serialize, Deserialize)]
pub struct Book {
    current: usize,
    pages: Vec<Grid>,
}

impl Default for Book {
    fn default() -> Self {
        Self {
            current: 0,
            pages: vec![EMPTY; PUZZLES.len()],
        }
    }
}

impl Book {
    pub fn restored(mut self) -> Self {
        self.current = self.current.min(PUZZLES.len() - 1);
        self.pages.resize(PUZZLES.len(), EMPTY);
        self
    }

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn puzzle(&self) -> Puzzle {
        PUZZLES[self.current]
    }

    pub fn page(&self) -> &Grid {
        &self.pages[self.current]
    }

    pub fn go(&mut self, index: usize) -> bool {
        if index >= PUZZLES.len() || index == self.current {
            return false;
        }
        self.current = index;
        true
    }

    pub fn toggle(&mut self, x: usize, y: usize, tool: Cell) -> bool {
        let size = self.puzzle().size();
        if x >= size || y >= size || tool == Cell::Unknown {
            return false;
        }
        let cell = &mut self.pages[self.current][y][x];
        *cell = if *cell == tool { Cell::Unknown } else { tool };
        true
    }

    pub fn clear(&mut self) {
        self.pages[self.current] = EMPTY;
    }

    pub fn solved(&self) -> bool {
        solved(self.puzzle(), self.page())
    }

    pub fn solved_count(&self) -> usize {
        PUZZLES
            .iter()
            .zip(&self.pages)
            .filter(|(puzzle, page)| solved(**puzzle, page))
            .count()
    }

    pub fn line_matches(&self, index: usize, column: bool) -> bool {
        runs((0..self.puzzle().size()).map(|offset| {
            let (x, y) = if column {
                (index, offset)
            } else {
                (offset, index)
            };
            self.page()[y][x] == Cell::Filled
        })) == self.puzzle().clues(index, column)
    }
}

fn solved(puzzle: Puzzle, page: &Grid) -> bool {
    (0..puzzle.size())
        .all(|y| (0..puzzle.size()).all(|x| (page[y][x] == Cell::Filled) == puzzle.filled(x, y)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_puzzle_has_one_solution() {
        fn count(rows: &[Vec<u16>], cols: &[Vec<u16>], y: usize) -> usize {
            if y == rows.len() {
                return 1;
            }
            let mut total = 0;
            for mask in &rows[y] {
                let next: Vec<Vec<u16>> = cols
                    .iter()
                    .enumerate()
                    .map(|(x, choices)| {
                        choices
                            .iter()
                            .copied()
                            .filter(|column| (column >> y) & 1 == (mask >> x) & 1)
                            .collect()
                    })
                    .collect();
                if next.iter().all(|choices| !choices.is_empty()) {
                    total += count(rows, &next, y + 1);
                    if total > 1 {
                        break;
                    }
                }
            }
            total
        }
        for puzzle in PUZZLES {
            let size = puzzle.size();
            assert!(puzzle.rows.iter().all(|row| row.len() == size));
            let options = |column| -> Vec<Vec<u16>> {
                (0..size)
                    .map(|line| {
                        let clues = puzzle.clues(line, column);
                        (0..(1 << size))
                            .filter(|mask| runs((0..size).map(|i| mask & (1 << i) != 0)) == clues)
                            .collect()
                    })
                    .collect()
            };
            assert_eq!(
                count(&options(false), &options(true), 0),
                1,
                "{}",
                puzzle.title
            );
        }
    }

    #[test]
    fn marking_and_leaving_a_puzzle_keeps_its_progress() {
        let mut book = Book::default().restored();
        book.toggle(0, 0, Cell::Marked);
        book.go(1);
        assert_eq!(book.page()[0][0], Cell::Unknown);
        book.go(0);
        assert_eq!(book.page()[0][0], Cell::Marked);
        book.toggle(0, 0, Cell::Marked);
        assert_eq!(book.page()[0][0], Cell::Unknown);
    }
}
