use crate::terminal::EConfig;

use std::io::Write; // for flush() in clearing screen
use std::io::{self, stdout};
use std::process::exit; // for clearer program exit

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
    should_quit: bool,
    cfg: EConfig,
}

impl Editor {
    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();

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
                _stdout.suspend_raw_mode().unwrap();
                exit(0)
            }
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        //returns error if encountered, does not handle it
        let pressed_key = read_key()?;
        // Unwrap the result(Key) -> Key
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        // else returns empty ok, no error returned
        Ok(())
    }

    pub fn default() -> Self {
        let cnfg = EConfig::default().unwrap();
        Self {
            should_quit: false,
            cfg: cnfg,
        }
    }

    fn draw_rows(&mut self) {
        for _y in 0..self.cfg.size.1 + 1 {
            print!("~\r\n");
        }
    }

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        print!("{}[2J", 27 as char); //27 as char is escape character
        print!("\x1b[H"); // \x1 is binary escape char; set cursor to top left of the screen
        if !self.should_quit {
            self.draw_rows();
        }
        print!("\x1b[H");
        // must print Gbye after setting cursor at the top
        if self.should_quit {
            println!("Gbye :)\r");
        }

        // flush clears the screen of all previous writing in buffer of terminal?
        io::stdout().flush()
    }
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        // locks handle to standard input stream, allowing us to read from it, returns error
        // if let -> match that handles only one match
        // if there is a (Some(Result(Key))) -> return Result
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
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
