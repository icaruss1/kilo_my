use termion::terminal_size;

pub struct EConfig {
    pub size: (u16, u16),
}

impl EConfig {
    pub fn default() -> Result<Self, std::io::Error> {
        let ter_size = terminal_size()?;
        Ok(Self {
            size: (ter_size.0, ter_size.1),
        })
    }
}
