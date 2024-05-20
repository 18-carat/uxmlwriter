mod status;
mod tests;

use crate::status::Status;
use std::fmt::Display;
use std::io::{BufWriter, Cursor, Write};

pub struct XmlWriter<T: Write> {
    writer: BufWriter<T>,
    status: Status,
    tags: Vec<String>,
}

impl Default for XmlWriter<Cursor<Vec<u8>>> {
    fn default() -> Self {
        Self::new()
    }
}

impl XmlWriter<Cursor<Vec<u8>>> {
    pub fn new() -> Self {
        let cursor = Cursor::new(Vec::new());
        Self {
            writer: BufWriter::new(cursor),
            status: Status::Closed,
            tags: Vec::new(),
        }
    }

    pub fn end_document(&mut self) -> String {
        self.end_document_no_return();
        let buffer = self.writer.get_ref().get_ref().clone();
        String::from_utf8(buffer).unwrap()
    }
}

impl<T: Write> XmlWriter<T> {
    pub fn with_writer(writer: T) -> Self {
        Self {
            writer: BufWriter::new(writer),
            status: Status::Closed,
            tags: Vec::new(),
        }
    }

    pub fn start_element(&mut self, tag: &str) {
        if self.status != Status::Closed {
            self.write(">");
        }

        self.indent();
        self.write("<");
        self.write(tag);
        self.tags.push(tag.to_string());
        self.status = Status::Open;
    }

    pub fn close_element(&mut self) {
        if let Some(tag) = self.tags.pop() {
            if self.status == Status::Open {
                self.write("/>");
                self.status = Status::Closed;
                return;
            }

            if self.status == Status::Closed {
                self.indent();
            }

            self.write(format!("</{}>", tag).as_str());
            self.status = Status::Closed;
        }
    }

    pub fn write_attribute<U: Display>(&mut self, key: &str, val: U) {
        self.write(format!(" {}=\"{}\"", key, Self::xml_encode(val)).as_str());
    }

    pub fn write_text<U: Display>(&mut self, text: U) {
        if self.status != Status::Inside {
            self.write(">");
            self.status = Status::Inside;
        }

        self.write(&Self::xml_encode(text));
    }

    pub fn write_comment(&mut self, comment: &str) {
        self.indent();
        self.write(&format!("<!-- {} -->", comment));
    }

    pub fn write_declaration(&mut self) {
        self.write("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    }

    pub fn end_document_no_return(&mut self) {
        while !self.tags.is_empty() {
            self.close_element();
        }

        match self.writer.flush() {
            Ok(_) => (),
            Err(e) => eprintln!("Error flushing writer: {}", e),
        }
    }

    fn write(&mut self, s: &str) {
        match self.writer.write_all(s.as_bytes()) {
            Ok(_) => (),
            Err(e) => eprintln!("Error writing to writer: {}", e),
        }
    }

    fn indent(&mut self) {
        self.write("\n");
        (0..self.tags.len()).for_each(|_| self.write("    "));
    }

    fn xml_encode<U: Display>(text: U) -> String {
        text.to_string()
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
    }
}
