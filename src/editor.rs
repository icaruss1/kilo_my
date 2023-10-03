use crate::document::Document;
use crate::terminal::Terminal;

use std::env;
use std::io::{Error, ErrorKind};
use std::process::exit; // for clearer program exit

use termion::event::Key;

const E_VERSION: &str = "0.0.1";

// move the screen state to the terminal
pub struct Editor {
    should_quit: bool,          // for exiting the program with ctrl+q
    terminal: Terminal,         // Terminal
    cursor_pos: (usize, usize), //  pos in document, needs to be larger than u16
    row_off: usize,             //row offset for scrolling
    mode: u16,                  // 0 for e, 1 for input
    document: Document,
}

impl Editor {
    pub fn run(&mut self) {
        if env::args().len() > 1 {
            let args: Vec<String> = env::args().collect();

            let filename = &args[1];
            if let Err(err) = self.document.open_file(filename) {
                die(err)
            }
        }
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
                self.terminal.suspend_raw_mode().as_ref().unwrap();
                exit(0)
            }
        }
    }

    fn process_keypress(&mut self) -> Result<(), Error> {
        //returns error if encountered, does not handle it
        let pressed_key = Terminal::read_key().unwrap();
        // Unwrap the result(Key) -> Key
        if self.mode == 0 {
            match pressed_key {
                Key::Char('h') => self.move_cursor(Key::Char('h')),
                Key::Char('j') => self.move_cursor(Key::Char('j')),
                Key::Char('k') => self.move_cursor(Key::Char('k')),
                Key::Char('l') => self.move_cursor(Key::Char('l')),
                Key::Char('i') => Ok(self.mode = 1),
                Key::PageUp => self.move_cursor(Key::PageUp),
                Key::PageDown => self.move_cursor(Key::PageDown),
                Key::Home => self.move_cursor(Key::Home),
                Key::End => self.move_cursor(Key::End),
                _ => Ok(()),
            }?;
        }
        match pressed_key {
            Key::Ctrl('q') => Ok(self.should_quit = true),
            Key::Esc => Ok(self.mode = 0),
            _ => Ok(()),
        }
        // else returns empty ok, no error returned
    }

    pub fn default() -> Self {
        let cnfg = Terminal::default().unwrap();
        Self {
            should_quit: false,
            terminal: cnfg,
            cursor_pos: (1, 1),
            row_off: 0,
            mode: 0,
            document: Document::default(),
        }
    }

    fn draw_welcome(&mut self) -> Result<(), Error> {
        // 1/3 down the screen, print the welcome message
        //let no_rows_string = format!("no of rows are {:?}", self.document.number_rows()?);
        let ver_string = format!("{} {}\r\n", "Kilo editor -- version", E_VERSION);
        // padding for printing the welcome msg in the middle of the screen
        let mut padding = (self.terminal.size().0 as usize - ver_string.len()) / 2;
        if padding > 0 {
            self.terminal.push_screen_state("~")?;
            padding -= 1;
        }
        for _ in 0..padding {
            self.terminal.push_screen_state(" ")?;
        }
        self.terminal.push_screen_state(&ver_string)?;
        //self.terminal.push_screen_state(&no_rows_string)?;
        Ok(())
    }

    fn draw_rows(&mut self) -> Result<(), Error> {
        // terminal.size().1 is height / no of cols
        for y in 0..self.terminal.size().1 - 1 {
            self.terminal.push_screen_state("\x1b[K")?; // clear row to right of cursor
            let filerow = y + self.row_off;
            if filerow >= self.document.number_rows()? {
                if y == self.terminal.size().1 / 3 && self.document.number_rows()? == 0 {
                    self.draw_welcome()?;
                } else {
                    self.terminal.push_screen_state("~\r\n")?; // print tilda and return newline
                }
                // IDK WHY READ_ROW(FILEROW) RENDERS THE ENTIRE SCREEN MOSTLY PERFECTLY
            } else {
                self.terminal
                    .push_screen_state(&self.document.read_row(filerow)?)?;
            }
        }
        Ok(())
    }

    fn editor_scroll(&mut self) -> Result<(), Error> {
        let cursor_cols = self.cursor_pos.0;
        let screen_rows = self.terminal.size().1;

        // if cursor is above visibile window, scrolls to wear the cursor is
        if cursor_cols < self.row_off {
            self.row_off = cursor_cols;
        };

        if cursor_cols >= self.row_off + screen_rows {
            self.row_off = cursor_cols - screen_rows;
        };
        Ok(())
    }

    // c for command (+ or -), x is number to add, axis is x-axis or y-axis
    fn modify_cursor(&mut self, c: char, xy: char, x: usize) -> Result<(), Error> {
        let e_xy = Error::new(ErrorKind::InvalidInput, "xy must be x or y Char");
        let e_command = Error::new(ErrorKind::InvalidInput, "c must be + or - Char");

        if c == '+' {
            match xy {
                'x' => Ok(self.cursor_pos.1 = self.cursor_pos.1.saturating_add(x)),
                'y' => Ok(self.cursor_pos.0 = self.cursor_pos.0.saturating_add(x)),
                _ => Err(e_xy),
            }
        } else if c == '-' {
            match xy {
                'x' => Ok(self.cursor_pos.1 = self.cursor_pos.1.saturating_sub(x)),
                'y' => Ok(self.cursor_pos.0 = self.cursor_pos.0.saturating_sub(x)),
                _ => Err(e_xy),
            }
        } else {
            Err(e_command)
        }
    }

    fn move_cursor(&mut self, k: Key) -> Result<(), Error> {
        match k {
            Key::Char('h') => self.modify_cursor('-', 'x', 1),
            Key::Char('l') => self.modify_cursor('+', 'x', 1),
            Key::Char('j') => self.modify_cursor('+', 'y', 1),
            Key::Char('k') => self.modify_cursor('-', 'y', 1),
            Key::PageDown => {
                let screen_height = self.terminal.size().1;
                self.modify_cursor('+', 'y', screen_height)
            }
            Key::PageUp => {
                let screen_height = self.terminal.size().1;
                self.modify_cursor('-', 'y', screen_height)
            }
            Key::Home => {
                let screen_width = self.terminal.size().0;
                self.modify_cursor('-', 'x', screen_width)
            }
            Key::End => {
                let screen_width = self.terminal.size().0;
                self.modify_cursor('+', 'x', screen_width)
            }
            _ => Ok(()),
        }
    }

    // using escape sequences (\x1b...), refresh the screen_state
    fn refresh_screen(&mut self) -> Result<(), Error> {
        self.editor_scroll()?;

        self.terminal.push_screen_state("\x1b[?25l")?; // turn off cursor
        self.terminal.push_screen_state("\x1b[H")?; // set cursor to top left of the screen
                                                    // clear line to the left of the cursor

        self.draw_rows()?;

        let cursor_string = format!("\x1b[{};{}H", self.cursor_pos.0, self.cursor_pos.1,);
        self.terminal.push_screen_state(&cursor_string)?;

        // must print Gbye after setting cursor at the top
        self.terminal.push_screen_state("\x1b[?25h")?; // turn cursor back on

        if self.should_quit {
            self.terminal.push_screen_state("\x1b[2J")?; //clear screen
            self.terminal.push_screen_state("\x1b[H")?; // reset cursor to top left of the screen
            self.terminal.push_screen_state("Gbye :) \r\n")?; // send message
        }
        // single write call handling all drawing and screen string formatting
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
