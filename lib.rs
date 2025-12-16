//! Command Line Kakuro Game
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

const CELL_WIDTH: u16 = 5;
const CELL_DEPTH: u16 = 3;

#[derive(Clone)]
/// Puzzle grid cell
pub struct Cell {
    /// Value of cell
    value: Option<u8>,
    /// Across sum, or None
    across_sum: Option<u16>,
    /// Down sum, or None
    down_sum: Option<u16>,
    /// Is clue or playable
    is_clue: bool,
    /// Across sum is flagged incorrect
    a_incorrect: bool,
    /// Down sum is flagged incorrect
    d_incorrect: bool,
}

impl Cell {
    /// Creates a playable cell
    pub fn new_play() -> Self {
        Cell {
            value: None,
            across_sum: None,
            down_sum: None,
            is_clue: false,
            a_incorrect: false,
            d_incorrect: false,
        }
    }

    /// Creates a clue cell
    pub fn new_clue(across: Option<u16>, down: Option<u16>) -> Self {
        Cell {
            value: None,
            across_sum: across,
            down_sum: down,
            is_clue: true,
            a_incorrect: false,
            d_incorrect: false,
        }
    }

    // Print methods
    /// Returns strings to be printed from a clue cell
    pub fn clue_string(&self) -> (String, String) {
        let left_w: usize = ((CELL_WIDTH - 1) / 2) as usize;
        let right_w: usize = CELL_WIDTH as usize - 1 - left_w;

        let down = self
            .down_sum()
            .map(|d| d.to_string())
            .unwrap_or("".to_string());
        let across = self
            .across_sum()
            .map(|a| a.to_string())
            .unwrap_or("".to_string());
        if down.is_empty() && across.is_empty() {
            return (String::new(), String::new());
        }
        (
            format!("{:>width$}", down, width = left_w),
            format!("{:<width$}", across, width = right_w),
        )
    }

    /// Returns strings to be printed from a play cell
    pub fn play_strings(&self) -> (String, String, String) {
        let val = self
            .value()
            .map(|v| v.to_string())
            .unwrap_or("_".to_string());
        (
            format!("┌{}┐", &"─".repeat(CELL_WIDTH as usize)),
            format!("|{:^width$}|", val, width = CELL_WIDTH as usize),
            format!("└{}┘", &"─".repeat(CELL_WIDTH as usize)),
        )
    }

    // Accessor Methods
    /// Returns cell is_clue
    pub fn is_clue(&self) -> bool {
        self.is_clue
    }

    /// Returns cell value
    pub fn value(&self) -> Option<u8> {
        self.value
    }

    /// Returns cell across_sum
    pub fn across_sum(&self) -> Option<u16> {
        self.across_sum
    }

    /// Returns cell down_sum
    pub fn down_sum(&self) -> Option<u16> {
        self.down_sum
    }

    /// Returns cell a_incorrect
    pub fn is_a_incorrect(&self) -> bool {
        self.a_incorrect
    }

    /// Returns cell d_incorrect
    pub fn is_d_incorrect(&self) -> bool {
        self.d_incorrect
    }

    // Modifying Methods

    /// Sets cell value
    pub fn set_value(&mut self, v: Option<u8>) {
        self.value = v;
    }

    /// Sets cell a_incorrect
    pub fn set_a_incorrect(&mut self, incorrect: bool) {
        self.a_incorrect = incorrect;
    }

    /// Sets cell d_incorrect
    pub fn set_d_incorrect(&mut self, incorrect: bool) {
        self.d_incorrect = incorrect;
    }
}

#[cfg(test)]
mod cell_tests {
    use super::*;

    #[test]
    fn test_new_play_cell() {
        let cell = Cell::new_play();
        assert!(!cell.is_clue());
        assert_eq!(cell.value(), None);
    }

    #[test]
    fn test_new_clue_cell() {
        let cell = Cell::new_clue(Some(10), Some(20));
        assert!(cell.is_clue());
        assert_eq!(cell.across_sum(), Some(10));
        assert_eq!(cell.down_sum(), Some(20));
    }

    #[test]
    fn test_set_and_get_value() {
        let mut cell = Cell::new_play();
        cell.set_value(Some(5));
        let val = cell.value();
        assert_eq!(val, Some(5));
        cell.set_value(None);
        let val = cell.value();
        assert_eq!(val, None);
    }

    #[test]
    fn test_incorrect_flags() {
        let mut cell = Cell::new_clue(Some(10), Some(20));
        cell.set_a_incorrect(true);
        assert!(cell.is_a_incorrect());
        cell.set_d_incorrect(true);
        assert!(cell.is_d_incorrect());
        cell.set_a_incorrect(false);
        assert!(!cell.is_a_incorrect());
        cell.set_d_incorrect(false);
        assert!(!cell.is_d_incorrect());
    }

    #[test]
    fn test_string_creation() {
        let empty_clue = Cell::new_clue(None, None);
        assert_eq!(empty_clue.clue_string(), (String::new(), String::new()));
        let down_clue = Cell::new_clue(None, Some(20));
        assert_eq!(
            down_clue.clue_string(),
            ("20".to_string(), "  ".to_string())
        );
        let across_clue = Cell::new_clue(Some(20), None);
        assert_eq!(
            across_clue.clue_string(),
            ("  ".to_string(), 20.to_string())
        );
        let both_clue = Cell::new_clue(Some(10), Some(20));
        assert_eq!(
            both_clue.clue_string(),
            ("20".to_string(), "10".to_string())
        );
        let mut play_cell = Cell::new_play();
        assert_eq!(
            play_cell.play_strings(),
            (
                "┌─────┐".to_string(),
                "|  _  |".to_string(),
                "└─────┘".to_string()
            )
        );
        play_cell.set_value(Some(5));
        assert_eq!(
            play_cell.play_strings(),
            (
                "┌─────┐".to_string(),
                "|  5  |".to_string(),
                "└─────┘".to_string()
            )
        );
    }
}

use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode},
    execute, queue,
    style::{Attribute, Color, ResetColor, SetBackgroundColor, Stylize},
    terminal::{self},
};
use std::{
    collections::HashSet,
    io::{Stdout, Write, stdout},
    ops::Not,
};

#[derive(Clone)]
/// Playable Kakuro Board
pub struct Board {
    /// Kakuro grid of cells
    grid: Vec<Vec<Cell>>,
    /// Rows in grid
    rows: usize,
    /// Columns in grid
    cols: usize,
    /// Cursor row index
    cursor_row: usize,
    /// Cursor column index
    cursor_col: usize,
}

impl Board {
    /// Board constructor
    pub fn new(grid: Vec<Vec<Cell>>, rows: usize, cols: usize) -> Self {
        Self {
            grid,
            rows,
            cols,
            cursor_row: 0,
            cursor_col: 0,
        }
    }

    // Accessor Methods

    /// Gets a reference to a cell in the board
    pub fn cell(&self, row: usize, col: usize) -> &Cell {
        &self.grid[row][col]
    }

    /// Gets a mutable reference to a cell in the board
    pub fn cell_mut(&mut self, row: usize, col: usize) -> &mut Cell {
        &mut self.grid[row][col]
    }

    /// Returns a reference to the cell the cursor is at
    pub fn cur_cell(&mut self) -> &mut Cell {
        &mut self.grid[self.cursor_row][self.cursor_col]
    }

    // Drawing Methods
    /// Draw a clue cell
    pub fn draw_clue(&self, stdout: &mut Stdout, row: u16, col: u16, cell: &Cell) {
        let x: u16 = col * (CELL_WIDTH + 2);
        let y: u16 = row * (CELL_DEPTH);

        for line in 0..CELL_DEPTH {
            // Move to top line
            queue!(stdout, MoveTo(x, y + line)).unwrap();
            if line == 1 {
                // Write content line
                let (left, right) = cell.clue_string();
                if left.is_empty() && right.is_empty() {
                    // No down or across sum
                    queue!(
                        stdout,
                        crossterm::style::Print(&" ".repeat(CELL_WIDTH as usize + 2))
                    )
                    .unwrap();
                } else {
                    queue!(stdout, crossterm::style::Print(" ")).unwrap();
                    // Print down sum
                    if cell.is_d_incorrect() {
                        // If incorrect, set to red
                        queue!(stdout, SetBackgroundColor(Color::Red)).unwrap();
                        queue!(stdout, crossterm::style::Print(&left)).unwrap();
                        queue!(stdout, ResetColor).unwrap();
                    } else {
                        queue!(stdout, crossterm::style::Print(&left)).unwrap();
                    }
                    queue!(stdout, crossterm::style::Print('\\')).unwrap();

                    // Print across sum
                    if cell.is_a_incorrect() {
                        // If incorrect, set to red
                        queue!(stdout, SetBackgroundColor(Color::Red)).unwrap();
                        queue!(stdout, crossterm::style::Print(&right)).unwrap();
                        queue!(stdout, ResetColor).unwrap();
                    } else {
                        queue!(stdout, crossterm::style::Print(&right)).unwrap();
                    }

                    queue!(stdout, crossterm::style::Print(' ')).unwrap();
                }
            } else {
                // Leave other lines blank
                queue!(
                    stdout,
                    crossterm::style::Print(" ".repeat(CELL_WIDTH as usize))
                )
                .unwrap();
            }

            queue!(stdout, ResetColor).unwrap(); // Reset color
        }
    }

    /// Draw a play cell
    pub fn draw_play(&self, stdout: &mut Stdout, row: u16, col: u16, cell: &Cell) {
        let x: u16 = col * (CELL_WIDTH + 2);
        let y: u16 = row * (CELL_DEPTH);

        queue!(stdout, MoveTo(x, y)).unwrap();
        // Print content line
        let (line1, line2, line3) = cell.play_strings();
        queue!(
            stdout,
            crossterm::style::Print(&line1),
            crossterm::cursor::MoveTo(x, y + 1)
        )
        .unwrap();
        queue!(
            stdout,
            crossterm::style::Print(&line2),
            crossterm::cursor::MoveTo(x, y + 2)
        )
        .unwrap();
        queue!(
            stdout,
            crossterm::style::Print(&line3),
            crossterm::cursor::MoveTo(x, y + CELL_DEPTH)
        )
        .unwrap();
        //
    }

    /// Draw the Kakuro board to the terminal
    pub fn draw(&self) {
        let mut stdout = stdout();

        // Clear terminal
        execute!(
            stdout,
            terminal::Clear(terminal::ClearType::Purge),
            MoveTo(0, 0)
        )
        .unwrap();

        // Draw each cell
        for r in 0..self.rows {
            for c in 0..self.cols {
                let cell = self.cell(r, c);
                if cell.is_clue() {
                    self.draw_clue(
                        &mut stdout,
                        r.try_into().unwrap(),
                        c.try_into().unwrap(),
                        cell,
                    );
                } else {
                    self.draw_play(
                        &mut stdout,
                        r.try_into().unwrap(),
                        c.try_into().unwrap(),
                        cell,
                    );
                }
            }
        }

        // Print help text one line below the grid
        let help_y = (self.rows as u16) * CELL_DEPTH + 1;
        queue!(stdout, MoveTo(0, help_y)).unwrap();
        queue!(
            stdout,
            crossterm::style::Print(format!(
                "{}{}{}{}",
                "esc".attribute(Attribute::Bold),
                ": home | ",
                "return".attribute(Attribute::Bold),
                ": check"
            ))
        )
        .unwrap();

        stdout.flush().unwrap();

        // Set playable cells to blinking
        if self.grid[self.cursor_row][self.cursor_col].is_clue() {
            execute!(stdout, crossterm::cursor::DisableBlinking).unwrap();
        } else {
            execute!(stdout, crossterm::cursor::EnableBlinking).unwrap();
        }

        // Move cursor to the cell
        execute!(
            stdout,
            MoveTo(
                self.cursor_col as u16 * (CELL_WIDTH + 2) + 3,
                self.cursor_row as u16 * CELL_DEPTH + 1
            )
        )
        .unwrap();

        terminal::enable_raw_mode().unwrap();
    }

    // Play Methods

    /// Moves the cursor to the right
    pub fn right(&mut self) {
        self.cursor_col = if self.cursor_col < self.cols - 1 {
            self.cursor_col + 1
        } else {
            0
        };
    }

    /// Moves the cursor to the left
    pub fn left(&mut self) {
        self.cursor_col = if self.cursor_col > 0 {
            self.cursor_col - 1
        } else {
            self.cols - 1
        };
    }

    /// Moves the cursor up
    pub fn up(&mut self) {
        self.cursor_row = if self.cursor_row > 0 {
            self.cursor_row - 1
        } else {
            self.rows - 1
        };
    }

    /// Moves the cursor down
    pub fn down(&mut self) {
        self.cursor_row = if self.cursor_row < self.rows - 1 {
            self.cursor_row + 1
        } else {
            0
        };
    }

    /// Deletes the current value
    pub fn backspace(&mut self) {
        if self.cur_cell().is_clue().not() {
            self.cur_cell().set_value(None);
        }
    }

    /// Adds a value to the current cell
    pub fn number(&mut self, c: char) {
        if !self.cur_cell().is_clue() {
            self.cur_cell()
                .set_value(Some(c.to_digit(10).unwrap() as u8));
        }
    }

    /// Begin game play and control input
    pub fn run(&mut self) {
        terminal::enable_raw_mode().unwrap();
        let mut stdout: std::io::Stdout = stdout();
        execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
        let y = self.rows * 3 + 2;

        let mut win = false;
        loop {
            if win {
                execute!(stdout, MoveTo(0, y.try_into().unwrap())).unwrap();
                print!("You win!");
            }
            self.draw();

            // Handle user input
            if let Event::Key(even) = event::read().unwrap() {
                match even.code {
                    KeyCode::Right => {
                        self.right();
                    }
                    KeyCode::Left => {
                        self.left();
                    }
                    KeyCode::Down => {
                        self.down();
                    }
                    KeyCode::Up => {
                        self.up();
                    }
                    KeyCode::Backspace => {
                        self.backspace();
                    }
                    KeyCode::Enter => {
                        win = self.check();
                    }
                    KeyCode::Char(c) if c.is_ascii_digit() && c != '0' => {
                        self.number(c);
                    }
                    KeyCode::Esc => break,
                    _ => {}
                };
            };
        }
        terminal::disable_raw_mode().unwrap();
        execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    }

    /// Check Kakuro board
    pub fn check(&mut self) -> bool {
        let mut correct: bool = true;
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.cell(r, c).is_clue() {
                    // Check each clue cell
                    if self.cell(r, c).across_sum().is_some() {
                        // Calculate across sum
                        let mut c2 = c + 1;
                        let mut sum = 0;
                        let mut items: HashSet<u8> = HashSet::new();

                        while c2 < self.cols {
                            // Move to the end of the board
                            if self.cell(r, c2).is_clue() {
                                // Stop at next clue cell
                                break;
                            }
                            if !items.insert(self.cell(r, c2).value().unwrap_or(0)) {
                                // Duplicate item found
                                self.cell_mut(r, c).set_a_incorrect(true);
                                correct = false;
                                break;
                            }
                            sum += self.cell(r, c2).value().unwrap_or(0);
                            c2 += 1;
                        }
                        // Check sum
                        if u16::from(sum) != self.cell(r, c).across_sum().unwrap() {
                            self.cell_mut(r, c).set_a_incorrect(true);
                            correct = false;
                        } else {
                            self.cell_mut(r, c).set_a_incorrect(false);
                        }
                    }
                    if self.cell(r, c).down_sum().is_some() {
                        let mut r2 = r + 1;
                        let mut sum2 = 0;
                        let mut items2: HashSet<u8> = HashSet::new();

                        while r2 < self.rows {
                            // Move to the end of the board
                            if self.cell(r2, c).is_clue() {
                                // Stop at the next clue
                                break;
                            }
                            if !items2.insert(self.cell(r2, c).value().unwrap_or(0)) {
                                // Duplicate item found
                                self.cell_mut(r, c).set_d_incorrect(true);
                                correct = false;
                                break;
                            }
                            sum2 += self.cell(r2, c).value().unwrap_or(0);
                            r2 += 1;
                        }
                        // Check sum
                        if u16::from(sum2) != self.cell(r, c).down_sum().unwrap() {
                            self.cell_mut(r, c).set_d_incorrect(true);
                            correct = false;
                        } else {
                            self.cell_mut(r, c).set_d_incorrect(false);
                        }
                    }
                }
            }
        }
        correct
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;

    fn sample_board() -> Board {
        let grid = vec![
            vec![
                Cell::new_clue(Some(4), None),
                Cell::new_play(),
                Cell::new_play(),
            ],
            vec![
                Cell::new_clue(None, Some(7)),
                Cell::new_play(),
                Cell::new_play(),
            ],
        ];
        Board::new(grid, 2, 3)
    }

    #[test]
    fn test_board_creation_and_access() {
        let board = sample_board();
        assert_eq!(board.rows, 2);
        assert_eq!(board.cols, 3);
        assert!(board.cell(0, 0).is_clue());
        assert!(!board.cell(0, 1).is_clue());
    }

    #[test]
    fn test_cursor_and_cell_mut() {
        let mut board = sample_board();
        board.cursor_row = 1;
        board.cursor_col = 2;
        board.cell_mut(1, 2).set_value(Some(5));
        assert_eq!(board.cell(1, 2).value(), Some(5));
        board.cur_cell().set_value(None);
        let val = board.cur_cell().value();
        assert_eq!(val, None);
    }

    #[test]
    fn test_cursor_wraps() {
        let mut board = sample_board();
        board.cursor_col = board.cols - 1;
        board.right();
        assert_eq!(board.cursor_col, 0);
        board.left();
        assert_eq!(board.cursor_col, board.cols - 1);
        board.cursor_row = board.rows - 1;
        board.down();
        assert_eq!(board.cursor_row, 0);
        board.up();
        assert_eq!(board.cursor_row, board.rows - 1);
    }

    #[test]
    fn test_check_across_sum() {
        let mut board = sample_board();
        board.cell_mut(0, 1).set_value(Some(2));
        board.cell_mut(0, 2).set_value(Some(2));
        board.check();
        // Should be incorrect because sum is 4, but digits repeat
        assert!(board.cell(0, 0).is_a_incorrect());
    }

    #[test]
    fn test_key_response() {
        let mut board = sample_board();

        {
            let cur = &board.cur_cell();
            assert_eq!(cur.value, None);
            assert_eq!(cur.across_sum, Some(4));
            assert_eq!(cur.down_sum, None);
        }
        {
            board.right();
            assert_eq!(board.cur_cell().value, None);
            assert_eq!(board.cur_cell().across_sum, None);
            assert_eq!(board.cur_cell().down_sum, None);
        }
        {
            board.number('7');
            assert_eq!(board.cur_cell().value, Some(7_u8));
            assert_eq!(board.cur_cell().across_sum, None);
            assert_eq!(board.cur_cell().down_sum, None);
        }
        {
            board.backspace();
            assert_eq!(board.cur_cell().value, None);
            assert_eq!(board.cur_cell().across_sum, None);
            assert_eq!(board.cur_cell().down_sum, None);
        }
        {
            board.down();
            assert_eq!(board.cur_cell().value, None);
            assert_eq!(board.cur_cell().across_sum, None);
            assert_eq!(board.cur_cell().down_sum, None);
        }

        {
            board.number('8');
            assert_eq!(board.cur_cell().value, Some(8_u8));
            assert_eq!(board.cur_cell().across_sum, None);
            assert_eq!(board.cur_cell().down_sum, None);
        }

        {
            board.left();
            assert_eq!(board.cur_cell().value, None);
            assert_eq!(board.cur_cell().across_sum, None);
            assert_eq!(board.cur_cell().down_sum, Some(7));
        }

        {
            board.left();
            assert_eq!(board.cur_cell().value, None);
            assert_eq!(board.cur_cell().across_sum, None);
            assert_eq!(board.cur_cell().down_sum, None);
        }
        {
            board.right();
            assert_eq!(board.cur_cell().value, None);
            assert_eq!(board.cur_cell().across_sum, None);
            assert_eq!(board.cur_cell().down_sum, Some(7));
        }
        {
            board.down();
            assert_eq!(board.cur_cell().value, None);
            assert_eq!(board.cur_cell().across_sum, Some(4));
            assert_eq!(board.cur_cell().down_sum, None);
        }

        {
            board.up();
            assert_eq!(board.cur_cell().value, None);
            assert_eq!(board.cur_cell().across_sum, None);
            assert_eq!(board.cur_cell().down_sum, Some(7));
        }
    }

    #[test]
    fn test_check_duplicates() {
        let board = vec![
            vec![
                Cell::new_clue(None, None),
                Cell::new_clue(None, Some(8)),
                Cell::new_clue(None, Some(8)),
            ],
            vec![
                Cell::new_clue(Some(8), None),
                Cell::new_play(),
                Cell::new_play(),
            ],
            vec![
                Cell::new_clue(Some(8), None),
                Cell::new_play(),
                Cell::new_play(),
            ],
        ];

        let mut board = Board::new(board, 3, 3);
        board.cell_mut(1, 1).set_value(Some(4));
        board.cell_mut(2, 1).set_value(Some(4));
        board.cell_mut(1, 2).set_value(Some(4));
        board.cell_mut(2, 2).set_value(Some(4));

        assert!(!board.check())
    }

    #[test]
    fn test_check_down_sum() {
        let grid = vec![
            vec![Cell::new_clue(None, Some(7))],
            vec![Cell::new_play()],
            vec![Cell::new_play()],
        ];
        let mut board = Board::new(grid, 3, 1);
        board.cell_mut(1, 0).set_value(Some(3));
        board.cell_mut(2, 0).set_value(Some(4));
        assert!(board.check());
        board.cell_mut(2, 0).set_value(Some(3));
        assert!(!board.check());
    }

    #[test]
    fn test_check_both_sums() {
        let grid = vec![
            vec![
                Cell::new_clue(Some(7), Some(7)),
                Cell::new_play(),
                Cell::new_play(),
            ],
            vec![Cell::new_play(), Cell::new_play(), Cell::new_play()],
        ];
        let mut board = Board::new(grid, 2, 3);
        board.cell_mut(0, 1).set_value(Some(3));
        board.cell_mut(0, 2).set_value(Some(4));
        board.cell_mut(1, 0).set_value(Some(7));
        assert!(board.check());
        board.cell_mut(0, 2).set_value(Some(3));
        assert!(!board.check());
    }

    #[test]
    fn test_clue_cells_not_mutated() {
        let mut board = sample_board();
        board.cursor_row = 0;
        board.cursor_col = 0; // clue cell
        board.number('5');
        assert_eq!(board.cur_cell().value(), None);
        board.backspace();
        assert_eq!(board.cur_cell().value(), None);
    }

    #[test]
    fn test_check_with_empty_cells() {
        let mut board = sample_board();
        board.cell_mut(0, 1).set_value(Some(2));
        // board.cell_mut(0, 2) left empty
        assert!(!board.check());
    }
}
