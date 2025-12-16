use kakuro::{Board, Cell};

use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode},
    execute,
    style::{Print, Stylize},
    terminal,
};
use std::io::stdout;

fn main() {
    home()
}

/// Returns a list of predefined levels & board constructors.
fn levels() -> Vec<(&'static str, Board)> {
    let board1 = vec![
        vec![
            Cell::new_clue(None, None),
            Cell::new_clue(None, None),
            Cell::new_clue(None, Some(30)),
            Cell::new_clue(None, Some(10)),
            Cell::new_clue(None, None),
            Cell::new_clue(None, None),
            Cell::new_clue(None, Some(3)),
            Cell::new_clue(None, Some(13)),
        ],
        vec![
            Cell::new_clue(None, None),
            Cell::new_clue(Some(7), Some(17)),
            Cell::new_play(),
            Cell::new_play(),
            Cell::new_clue(None, None),
            Cell::new_clue(Some(10), Some(10)),
            Cell::new_play(),
            Cell::new_play(),
        ],
        vec![
            Cell::new_clue(Some(24), None),
            Cell::new_play(),
            Cell::new_play(),
            Cell::new_play(),
            Cell::new_clue(Some(7), Some(23)),
            Cell::new_play(),
            Cell::new_play(),
            Cell::new_play(),
        ],
        vec![
            Cell::new_clue(Some(17), None),
            Cell::new_play(),
            Cell::new_play(),
            Cell::new_clue(Some(10), Some(8)),
            Cell::new_play(),
            Cell::new_play(),
            Cell::new_clue(None, Some(30)),
            Cell::new_clue(None, None),
        ],
        vec![
            Cell::new_clue(None, None),
            Cell::new_clue(Some(35), None),
            Cell::new_play(),
            Cell::new_play(),
            Cell::new_play(),
            Cell::new_play(),
            Cell::new_play(),
            Cell::new_clue(None, Some(10)),
        ],
        vec![
            Cell::new_clue(None, None),
            Cell::new_clue(None, Some(4)),
            Cell::new_clue(Some(10), Some(13)),
            Cell::new_play(),
            Cell::new_play(),
            Cell::new_clue(Some(7), Some(15)),
            Cell::new_play(),
            Cell::new_play(),
        ],
        vec![
            Cell::new_clue(Some(7), None),
            Cell::new_play(),
            Cell::new_play(),
            Cell::new_play(),
            Cell::new_clue(Some(24), None),
            Cell::new_play(),
            Cell::new_play(),
            Cell::new_play(),
        ],
        vec![
            Cell::new_clue(Some(12), None),
            Cell::new_play(),
            Cell::new_play(),
            Cell::new_clue(None, None),
            Cell::new_clue(Some(16), None),
            Cell::new_play(),
            Cell::new_play(),
            Cell::new_clue(None, None),
        ],
    ];

    let board2 = vec![
        vec![
            Cell::new_clue(None, None),
            Cell::new_clue(None, Some(3)),
            Cell::new_clue(None, Some(14)),
            Cell::new_clue(None, None),
            Cell::new_clue(None, None),
        ],
        vec![
            Cell::new_clue(Some(9), None),
            Cell::new_play(),
            Cell::new_play(),
            Cell::new_clue(None, Some(13)),
            Cell::new_clue(None, None),
        ],
        vec![
            Cell::new_clue(Some(7), None),
            Cell::new_play(),
            Cell::new_play(),
            Cell::new_play(),
            Cell::new_clue(None, Some(9)),
        ],
        vec![
            Cell::new_clue(None, None),
            Cell::new_clue(Some(6), None),
            Cell::new_play(),
            Cell::new_play(),
            Cell::new_play(),
        ],
        vec![
            Cell::new_clue(None, None),
            Cell::new_clue(None, None),
            Cell::new_clue(Some(17), None),
            Cell::new_play(),
            Cell::new_play(),
        ],
    ];

    let board3 = vec![
        vec![
            Cell::new_clue(None, None),
            Cell::new_clue(None, Some(12)),
            Cell::new_clue(None, Some(3)),
        ],
        vec![
            Cell::new_clue(Some(9), None),
            Cell::new_play(),
            Cell::new_play(),
        ],
        vec![
            Cell::new_clue(Some(6), None),
            Cell::new_play(),
            Cell::new_play(),
        ],
    ];

    vec![
        ("Level 1 (3 x 3)", Board::new(board3, 3, 3)),
        ("Level 2 (5 x 5)", Board::new(board2, 5, 5)),
        ("Level 3 (8 x 8)", Board::new(board1, 8, 8)),
    ]
}

/// Displays the home menu and handles level selection.
fn home() {
    let mut stdout = stdout();

    loop {
        execute!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            MoveTo(0, 0)
        )
        .unwrap();
        println!("{}\n", "Welcome to Kakuro".dark_magenta());
        execute!(
            stdout,
            Print("Select a level by pressing its number and entering (q to quit):\n\n")
        )
        .unwrap();

        let mut lvls = levels();
        for (i, (label, _ctor)) in lvls.iter().enumerate() {
            execute!(stdout, Print(format!("  {}) {}\n", i + 1, label))).unwrap();
        }

        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Char('q') => {
                    execute!(
                        stdout,
                        terminal::Clear(terminal::ClearType::All),
                        MoveTo(0, 0)
                    )
                    .unwrap();
                    return;
                }
                KeyCode::Char(c) if c.is_ascii_digit() && c != '0' => {
                    let idx = (c.to_digit(10).unwrap() as usize) - 1;
                    if idx < lvls.len() {
                        let board = &mut lvls[idx].1;
                        board.run();
                    }
                }
                _ => {}
            }
        }
    }
}
