use std::io::{self, stdout, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::terminal_size;

pub struct Terminal {
    size: (usize, usize),
    _stdout: RawTerminal<std::io::Stdout>,
    screen_state: String,
    cursor_pos: (u16, u16), // pos on screen, can be u16
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        // returns (cols, rows)
        let ter_size = terminal_size()?;
        Ok(Self {
            size: (ter_size.0 as usize, ter_size.1 as usize),
            _stdout: stdout().into_raw_mode()?,
            screen_state: String::new(),
            cursor_pos: (1, 1),
        })
    }

    pub fn size(&self) -> &(usize, usize) {
        &self.size
    }

    pub fn suspend_raw_mode(&self) -> Result<(), std::io::Error> {
        self._stdout.suspend_raw_mode()
    }

    pub fn push_screen_state(&mut self, s: &str) -> Result<(), std::io::Error> {
        Ok(self.screen_state.push_str(s))
    }

    pub fn write(&mut self) -> Result<usize, std::io::Error> {
        io::stdout().write(self.screen_state.as_bytes())
    }

    pub fn flush(&mut self) -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            // locks handle to standard input stream, allowing us to read from it, returns error
            // if let -> match that handles only one match
            // if there is a (Some(Result(Key))) -> return Result
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    //returns cursor_pos of terminal
    pub fn gcursor_pos(&mut self) -> Result<(u16, u16), std::io::Error> {
        let x = self.cursor_pos.0.saturating_sub(1);
        let y = self.cursor_pos.1.saturating_sub(1);

        Ok((x, y))
    }
}
