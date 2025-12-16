
# Kakuro Game

## Description

This project is a Rust command-line implementation of the Kakuro (Cross Sums) logic puzzle. The program renders playable Kakuro boards in a terminal, accepts keyboard input for navigation and entry, validates runs against clue sums and uniqueness constraints, and includes unit/integration tests to verify behavior. 

Core concepts:

- Playable cells accept digits 1–9.

- Clue cells contain across and/or down sums and are used to validate runs.

- Board validation checks each run for correct sum and no repeated digits.

  

Primary files:

- src/lib.rs — Board and Cell models, drawing logic, and validation.

- src/main.rs — Terminal UI, levels(), and the main loop.

- tests/integration_test.rs — PTY-based tests that simulate terminal interaction.

  

## Installation

Common commands (run from project root):

- Build (debug):

```sh

cargo build

```

  

- Run (debug):

```sh

cargo run

```

  

- Run tests (unit + integration):

```sh

cargo test

```

  

## How to use

  
Start the program:

```sh

cargo  run

```

Workflow:

1. On startup a home menu lists available levels. Press the number for a level and then Enter to open it.

2. Inside a board, use the controls below to play or check the board.

3. Press q or Esc to quit a board or exit the program, respectively.

  

Controls:

- Arrow keys: move cursor (Up / Down / Left / Right)

- Number keys 1–9: place that digit in the selected playable cell

- Backspace: clear the selected playable cell

- Enter: run board validation (check runs against clues)

- Esc: exit board and go home from puzzle

- q: quit program (from home)

  

Game Rules:

- Fill in the cells so that they add to the given sum.

- Clues are in the format {down sum}\\{across sum}

- Cells must be filled with number 1-9

- Sums cannot be computed with duplicate numbers

- Sums will be highlighted red if they are not correct

- Some puzzles may have multiple correct solutions.

  

Troubleshooting:

- Resize your terminal if the board looks clipped - the UI expects enough rows/columns.

Kakuro Example Logic:
```text
                             20\    16\   
                            ┌─────┐┌─────┐
                      24\17 |  _  ||  _  |
                            └─────┘└─────┘
                     ┌─────┐┌─────┐┌─────┐
               14\24 |  _  ||  _  ||  _  |
                     └─────┘└─────┘└─────┘
              ┌─────┐┌─────┐┌─────┐
          \20 |  _  ||  _  ||  _  |
              └─────┘└─────┘└─────┘
              ┌─────┐┌─────┐
          \13 |  _  ||  _  |
              └─────┘└─────┘
```
The only non-repeating two digits 1-9 that sum to 16 are (7,9).
The only non-repeating two digits 1-9 that sum to 17 are (8,9).
```text
                         20\    16\   
                        ┌─────┐┌─────┐
                  24\17 |  8  ||  9  |
                        └─────┘└─────┘
                 ┌─────┐┌─────┐┌─────┐
           14\24 |  _  ||  _  ||  7  |
                 └─────┘└─────┘└─────┘
          ┌─────┐┌─────┐┌─────┐
      \20 |  _  ||  _  ||  _  |
          └─────┘└─────┘└─────┘
          ┌─────┐┌─────┐
      \13 |  _  ||  _  |
          └─────┘└─────┘
```
24 -7 = 18. The rest of the row must be (8,9)
```text
                         20\    16\   
                        ┌─────┐┌─────┐
                  24\17 |  8  ||  9  |
                        └─────┘└─────┘
                 ┌─────┐┌─────┐┌─────┐
           14\24 |  8  ||  9  ||  7  |
                 └─────┘└─────┘└─────┘
          ┌─────┐┌─────┐┌─────┐
      \20 |  _  ||  _  ||  _  |
          └─────┘└─────┘└─────┘
          ┌─────┐┌─────┐
      \13 |  _  ||  _  |
          └─────┘└─────┘
```
20 -8 -9 = 3. 
```text
                         20\    16\   
                        ┌─────┐┌─────┐
                  24\17 |  8  ||  9  |
                        └─────┘└─────┘
                 ┌─────┐┌─────┐┌─────┐
           14\24 |  8  ||  9  ||  7  |
                 └─────┘└─────┘└─────┘
          ┌─────┐┌─────┐┌─────┐
      \20 |  _  ||  _  ||  3  |
          └─────┘└─────┘└─────┘
          ┌─────┐┌─────┐
      \13 |  _  ||  _  |
          └─────┘└─────┘
```
20 -8 -9 = 3. 
```text
                         20\    16\   
                        ┌─────┐┌─────┐
                  24\17 |  8  ||  9  |
                        └─────┘└─────┘
                 ┌─────┐┌─────┐┌─────┐
           14\24 |  8  ||  9  ||  7  |
                 └─────┘└─────┘└─────┘
          ┌─────┐┌─────┐┌─────┐
      \20 |  _  ||  _  ||  3  |
          └─────┘└─────┘└─────┘
          ┌─────┐┌─────┐
      \13 |  _  ||  _  |
          └─────┘└─────┘
```
20 - 3 =  17, the rest of the row must be (8,9). 
```text
                         20\    16\   
                        ┌─────┐┌─────┐
                  24\17 |  8  ||  9  |
                        └─────┘└─────┘
                 ┌─────┐┌─────┐┌─────┐
           14\24 |  8  ||  9  ||  7  |
                 └─────┘└─────┘└─────┘
          ┌─────┐┌─────┐┌─────┐
      \20 |  8  ||  9  ||  3  |
          └─────┘└─────┘└─────┘
          ┌─────┐┌─────┐
      \13 |  _  ||  _  |
          └─────┘└─────┘
```
14 - 8 =  6.
24 - 8 - 9 = 7.
6 + 7 = 13.
```text                                 
                             20\    16\   
                            ┌─────┐┌─────┐
                      24\17 |  8  ||  9  |
                            └─────┘└─────┘
                     ┌─────┐┌─────┐┌─────┐
               14\24 |  8  ||  9  ||  7  |
                     └─────┘└─────┘└─────┘
              ┌─────┐┌─────┐┌─────┐
          \20 |  8  ||  9  ||  3  |
              └─────┘└─────┘└─────┘
              ┌─────┐┌─────┐
          \13 |  6  ||  7  |
              └─────┘└─────┘
```                            

  
