#[cfg(test)]
mod uxmlwriter_tests {
    use crate::XmlWriter;
    use std::fs::{read_to_string, File};

    #[test]
    fn with_writer() {
        let file = File::create("test.xml").unwrap();
        let mut w = XmlWriter::with_writer(file);

        w.write_declaration();
        w.start_element("character");
        w.start_element("name");
        w.write_text("Goku");
        w.close_element();
        w.start_element("power_level");
        w.write_attribute("is_meme", "YES");
        w.write_text(9001);
        w.end_document_no_return();

        let expected = r#"<?xml version="1.0" encoding="UTF-8"?>
<character>
    <name>Goku</name>
    <power_level is_meme="YES">9001</power_level>
</character>"#;

        assert_eq!(read_to_string("test.xml").unwrap(), expected);
    }

    #[test]
    fn maria() {
        let mut w = XmlWriter::new();
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
        w.write_comment(
            "From where you're kneeling it must seem like an 18-carat run of bad luck.",
        );
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

        assert_eq!(w.end_document(), expected.to_string());
    }
}
