mod status;

use crate::status::Status;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct XmlWriter {
    file: BufWriter<File>,
    status: Status,
    tags: Vec<String>,
}

impl XmlWriter {
    pub fn new(file: File) -> Self {
        Self {
            file: BufWriter::new(file),
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

    pub fn write_attribute<T: Display>(&mut self, key: &str, value: T) {
        self.write(format!(" {}=\"{}\"", key, value).as_str());
    }

    pub fn write_text<T: Display>(&mut self, text: T) {
        if self.status != Status::Inside {
            self.write(">");
            self.status = Status::Inside;
        }

        self.write(&text.to_string());
    }

    pub fn write_comment(&mut self, comment: &str) {
        self.indent();
        self.write(&format!("<!-- {} -->", comment));
    }

    pub fn write_declaration(&mut self) {
        self.write("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    }

    pub fn end_document(&mut self) {
        while !self.tags.is_empty() {
            self.close_element();
        }

        match self.file.flush() {
            Ok(_) => (),
            Err(e) => eprintln!("Error flushing file: {}", e),
        }
    }

    fn write(&mut self, s: &str) {
        match self.file.write_all(s.as_bytes()) {
            Ok(_) => (),
            Err(e) => eprintln!("Error writing to file: {}", e),
        }
    }

    fn indent(&mut self) {
        self.write("\n");
        (0..self.tags.len()).for_each(|_| self.write("    "));
    }
}

#[test]
fn maria() {
    let file = File::create("test.xml").unwrap();
    let mut w = XmlWriter::new(file);
    let expected = r#"<?xml version="1.0" encoding="UTF-8"?>
<compendium xmlns:exsl="http://exslt.org/common" version="5" auto_indent="NO">
    <item>
        <name>Maria</name>
        <text/>
        <!-- You've made your last delivery kid. -->
        <!-- Sorry you got twisted up in this scene. -->
        <!-- From where you're kneeling it must seem like an 18-carat run of bad luck. -->
        <!-- Truth is... -->
        <!-- The game was rigged from the start. -->
        <type>Pistol</type>
        <damage>20</damage>
        <rate>3.8</rate>
        <weight>1.5</weight>
        <value>999</value>
        <owner>Benny</owner>
    </item>
</compendium>"#;

    w.write_declaration();
    w.start_element("compendium");
    w.write_attribute("xmlns:exsl", "http://exslt.org/common");
    w.write_attribute("version", 5);
    w.write_attribute("auto_indent", "NO");
    w.start_element("item");
    w.start_element("name");
    w.write_text("Maria");
    w.close_element();
    w.start_element("text");
    w.close_element();
    w.write_comment("You've made your last delivery kid.");
    w.write_comment("Sorry you got twisted up in this scene.");
    w.write_comment("From where you're kneeling it must seem like an 18-carat run of bad luck.");
    w.write_comment("Truth is...");
    w.write_comment("The game was rigged from the start.");
    w.start_element("type");
    w.write_text("Pistol");
    w.close_element();
    w.start_element("damage");
    w.write_text("20");
    w.close_element();
    w.start_element("rate");
    w.write_text(3.8);
    w.close_element();
    w.start_element("weight");
    w.write_text(1.5);
    w.close_element();
    w.start_element("value");
    w.write_text(999);
    w.close_element();
    w.start_element("owner");
    w.write_text("Benny");
    w.end_document();

    assert_eq!(std::fs::read_to_string("test.xml").unwrap(), expected);
}
