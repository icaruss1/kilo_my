use std::fs::read_to_string;
use std::io::{Error, ErrorKind};

struct Row {
    text: String,
}

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    //open file s
    pub fn open_file(&mut self, s: &str) -> Result<(), Error> {
        let rows: Vec<Row> = read_to_string(s)?
            .lines()
            .map(String::from)
            .map(|s| Row { text: s })
            .collect();

        self.rows = rows;
        Ok(())
    }

    pub fn read_rows(&mut self) -> Result<String, Error> {
        let mut text_string = String::new();
        for row in &self.rows {
            let text = row.text.clone();
            let format_sring = format!("{}\r\n", text);
            text_string.push_str(&format_sring);
        }
        Ok(text_string)
    }

    pub fn number_rows(&self) -> Result<usize, Error> {
        Ok(self.rows.len())
    }

    pub fn close_document(&mut self) -> Result<(), Error> {
        let rows = Vec::new();
        self.rows = rows;
        Ok(())
    }
}
