# μxmlwriter

A simple XML writer library for Rust. It has an API almost identical to that of the `xmlwriter` library, with minor difference:

1. Adds `XmlWriter::with_writer()` to accept, e.g., a `File` object
2. Adds `XmlWriter::end_document_no_return()` to support the above
3. Does not include the `write_XXX_fmt` methods

New usage (recommended):

```
let file = File::create("test.xml").unwrap();
let mut w = XmlWriter::with_writer(file);

w.write_declaration();
w.start_element("character");
w.start_element("name");
w.write_text("Goku");
w.end_element();
w.start_element("power_level");
w.write_attribute("is_meme", "YES");
w.write_text(9001);
w.end_document_no_return();

assert_eq!(read_to_string("test.xml").unwrap(), expected);
```

Compatibility usage:

```
let mut w = XmlWriter::new();

w.write_declaration();
w.start_element("character");
w.start_element("name");
w.write_text("Goku");
w.end_element();
w.start_element("power_level");
w.write_attribute("is_meme", "YES");
w.write_text(9001);

let s = w.end_document();
let expected = r#"<?xml version="1.0" encoding="UTF-8"?>
<character>
  <name>Goku</name>
  <power_level is_meme="YES">9001</power_level>
</character>"#;

assert_eq!(s, expected);
*/
```
