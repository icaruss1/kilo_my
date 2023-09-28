use crate::terminal::Terminal;

use std::process::exit; // for clearer program exit

const E_VERSION: &str = "0.0.1";

// move the screen state to the terminal
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,     // Terminal
    cursor_pos: (u16, u16), // first is y-axis, second is x-axis?? indexed from 1
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            // Handling errors in process keypress
            // if let = matches only condition
            // if there is error in processing keypress then die else continue processing
            if let Err(err) = self.refresh_screen() {
                die(err);
            }
            if let Err(err) = self.process_keypress() {
                die(err);
            }
            if self.should_quit == true {
                self.refresh_screen().unwrap();
                self.terminal.suspend_raw_mode().unwrap();
                exit(0)
            }
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        //returns error if encountered, does not handle it
        let pressed_key = Terminal::read_key()?;
        // Unwrap the result(Key) -> Key
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Char('h') => self.move_cursor(Key::Char('h')),
            Key::Char('j') => self.move_cursor(Key::Char('j')),
            Key::Char('k') => self.move_cursor(Key::Char('k')),
            Key::Char('l') => self.move_cursor(Key::Char('l')),
            _ => (),
        }
        // else returns empty ok, no error returned
        Ok(())
    }

    pub fn default() -> Self {
        let cnfg = Terminal::default().unwrap();
        Self {
            should_quit: false,
            terminal: cnfg,
        }
    }

    fn draw_rows(&mut self) {
        for y in 0..self.terminal.size.1 - 1 {
            self.terminal.push_screen_state("\x1b[K"); // clear row to right of cursor
            if y == self.terminal.size.1 / 3 {
                // 1/3 down the screen, print the welcome message
                let ver_string = format!("{} {}\r\n", "Kilo editor -- version", E_VERSION);

                // padding for printing the welcome msg in the middle of the screen
                let mut padding = (self.terminal.size.0 as usize - ver_string.len()) / 2;
                if padding > 0 {
                    self.terminal.push_screen_state("~");
                    padding -= 1;
                }
                for _ in 0..padding {
                    self.terminal.push_screen_state(" ");
                }
                self.terminal.push_screen_state(&ver_string);
            } else {
                self.terminal.push_screen_state("~\r\n"); // print tilda and return newline
            }
        }
    }

    fn move_cursor(&mut self, k: Key) {
        match k {
            Key::Char('h') => {
                if self.cursor_pos.1 > 1 {
                    self.cursor_pos.1 -= 1
                }
            }
            Key::Char('l') => {
                if self.cursor_pos.1 < self.terminal.size.0 as usize {
                    self.cursor_pos.1 += 1
                }
            }
            Key::Char('j') => {
                if self.cursor_pos.0 < self.terminal.size.1 as usize {
                    self.cursor_pos.0 += 1
                }
            }
            Key::Char('k') => {
                if self.cursor_pos.0 > 1 {
                    self.cursor_pos.0 -= 1
                }
            }
            _ => (),
        };
    }

    // using escape sequences (\x1b...), refresh the screen_state
    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        self.terminal.push_screen_state("\x1b[?25l"); // turn off cursor
        self.terminal.push_screen_state("\x1b[H"); // set cursor to top left of the screen

        self.draw_rows();

        let cur_string = format!(
            "\x1b[{};{}H",
            self.terminal.cursor_pos()?.0,
            self.terminal.cursor_pos()?.1,
        );
        self.terminal.push_screen_state(&cur_string);

        // must print Gbye after setting cursor at the top
        self.terminal.push_screen_state("\x1b[?25h"); // turn cursor back on

        if self.should_quit {
            self.terminal.push_screen_state("\x1b[2J"); //clear screen
            self.terminal.push_screen_state("\x1b[H"); // reset cursor to top left of the screen
            self.terminal.push_screen_state("Gbye :) \r\n"); // send message
        }
        self.terminal.write()?;
        // flush clears the screen of all previous writing in buffer of terminal?
        self.terminal.flush()
    }
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}

//                if c.is_control() {
//                    println!("{:?} \r", b);
//                } else {
//                    println!("{:?} ({})\r", b, c);
//                }
