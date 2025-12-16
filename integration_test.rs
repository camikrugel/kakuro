use core::panic;
use ntest::timeout;
use pseudoterminal::CommandExt;
use std::io::{Read, Write};
use std::process::Command;

fn match_output<R: Read>(reader: &mut R, pattern: &[u8]) {
    let mut buffer = Vec::new();
    let mut temp_buffer = [0u8; 256];

    loop {
        let bytes_read = reader.read(&mut temp_buffer).unwrap();
        if bytes_read == 0 {
            panic!("End of stream reached before pattern was found");
        }
        buffer.extend_from_slice(&temp_buffer[..bytes_read]);

        if buffer
            .windows(pattern.len())
            .any(|window| window == pattern)
        {
            break;
        }
    }
}

#[test]
#[timeout(2000)] // 2 seconds timeout
fn test1() {
    // Create a new pseudoterminal
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_kakuro"));
    let mut terminal = cmd.spawn_terminal().unwrap();

    let (mut input, mut output) = terminal.split().unwrap();

    // Wait for the home screen to load, check welcome message
    match_output(&mut output, b"Welcome to Kakuro");

    // Select level 1, check bottom navigation
    input.write_all(b"1\n").unwrap();
    input.flush().unwrap();
    match_output(&mut output, b"home");

    input.write_all(b"q\n").unwrap();

    let _ = terminal.close();
}

#[test]
#[timeout(2000)]
fn test_menu_and_quit() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_kakuro"));
    let mut terminal = cmd.spawn_terminal().unwrap();
    let (mut input, mut output) = terminal.split().unwrap();

    // Check welcome message
    match_output(&mut output, b"Welcome to Kakuro");

    // Quit from menu
    input.write_all(b"q\n").unwrap();
    input.flush().unwrap();
    // Optionally check for exit message or just ensure no panic
    let _ = terminal.close();
}

#[test]
#[timeout(3000)]
fn test_level_selection_and_help() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_kakuro"));
    let mut terminal = cmd.spawn_terminal().unwrap();
    let (mut input, mut output) = terminal.split().unwrap();

    // Wait for menu
    match_output(&mut output, b"Welcome to Kakuro");

    // Select level 2
    input.write_all(b"2\n").unwrap();
    input.flush().unwrap();

    // Check for help/navigation line in board UI
    match_output(&mut output, b"13");
    match_output(&mut output, b"home");

    // Quit from board
    input.write_all(b"q\n").unwrap();
    input.flush().unwrap();
    let _ = terminal.close();
}

#[test]
#[timeout(4000)]
fn test_invalid_level_then_valid() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_kakuro"));
    let mut terminal = cmd.spawn_terminal().unwrap();
    let (mut input, mut output) = terminal.split().unwrap();

    match_output(&mut output, b"Welcome to Kakuro");

    // Try invalid level
    input.write_all(b"9\n").unwrap();
    input.flush().unwrap();
    // Should stay at menu, so check menu text again
    match_output(&mut output, b"Welcome to Kakuro");

    // Now select valid level
    input.write_all(b"1\n").unwrap();
    input.flush().unwrap();
    match_output(&mut output, b"home");

    input.write_all(b"q\n").unwrap();
    let _ = terminal.close();
}
