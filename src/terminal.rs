use std::io::{self, stdout, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::terminal_size;

pub struct Terminal {
    pub size: (u16, u16),
    _stdout: RawTerminal<std::io::Stdout>,
    screen_state: String,
    cursor_pos: (usize, usize),
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let ter_size = terminal_size()?;
        Ok(Self {
            size: (ter_size.0, ter_size.1),
            _stdout: stdout().into_raw_mode()?,
            cursor_pos: (1, 1),
            screen_state: String::new(),
        })
    }

    pub fn size(&self) -> &(u16, u16) {
        &self.size
    }

    pub fn suspend_raw_mode(&self) -> &Result<(), std::io::Error> {
        &self._stdout.suspend_raw_mode()
    }

    pub fn push_screen_state(&self, s: &str) -> Result<(), std::io::Error> {
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

    pub fn cursor_pos(&mut self) -> Result<(usize, usize), std::io::Error> {
        let x = self.cursor_pos.0.saturating_sub(1);
        let y = self.cursor_pos.1.saturating_sub(1);

        Ok((x, y))
    }
}
