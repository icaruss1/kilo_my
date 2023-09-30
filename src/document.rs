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
    pub fn open_file(&mut self, _s: &str) -> Result<(), Error> {
        let s = "hello world\n\r".to_string();
        let s1 = "how are you\n\r".to_string();
        let r1 = Row { text: s };
        let r2 = Row { text: s1 };

        self.rows.push(r1);
        self.rows.push(r2);
        Ok(())
    }

    pub fn read_rows(&mut self) -> Result<String, Error> {
        let mut text_string = String::new();
        for row in &self.rows {
            let text = row.text.clone();
            text_string.push_str(&text);
        }
        Ok(text_string)
    }

    pub fn number_rows(&self) -> Result<usize, Error> {
        Ok(self.rows.len())
    }
}
