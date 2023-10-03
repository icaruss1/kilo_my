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
        let rows: Vec<Row> = read_to_string(s)? //string file
            .lines()
            .map(String::from)
            .map(|s| Row { text: s })
            .collect();

        self.rows = rows;
        Ok(())
    }

    // give read_rows a start, stop to read with any draw call
    pub fn read_rows(&mut self) -> Result<String, Error> {
        let mut text_string = String::new();
        for row in &self.rows {
            let text = row.text.clone();
            let format_sring = format!("{}\r\n", text);
            text_string.push_str(&format_sring);
        }
        Ok(text_string)
    }

    pub fn read_row(&mut self, i: usize) -> Result<String, Error> {
        let text = &self.rows[i].text.clone();
        Ok(format!("{}\r\n", text))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_test_file() {
        let mut doc = Document::default();
        assert_eq!(doc.rows.len(), 0);

        doc.open_file("./src/test_file.txt");
        assert_eq!(doc.rows.len(), 3);
    }

    #[test]
    fn open_editor_file() {
        let mut doc = Document::default();

        doc.open_file("./src/editor.rs");
        assert!(doc.rows.len() > 5);
    }
}
