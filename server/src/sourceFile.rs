use lsp_types::*;
use tree_sitter::{Parser, Tree, InputEdit, Point};
use std::convert::TryInto;

pub struct SourceFile {
    pub tree: Tree,
    pub code: String,
    pub line_bytes: Vec<u64>,
}

impl SourceFile {
    fn create_line_bytes(code: &String) -> Vec<u64> {
        let mut line_bytes: Vec<u64> = Vec::new();
    
        let mut char_iter = code.char_indices();
        let mut col = 0;
        
        while let Some(c) = char_iter.next() {
            col += 1;
            if c.1 == '\n' {
                line_bytes.push(col);
                col = 0;
            }
        }
        line_bytes.push(col);
        
        line_bytes
    }

    pub fn new(code: String, parser: &mut Parser) -> SourceFile {
        let tree = parser.parse(code.clone(), None).unwrap();
        let line_bytes = SourceFile::create_line_bytes(&code);
        
        SourceFile {
            tree,
            code,
            line_bytes,
        }
    }

    fn find_end_position(start_position: &Point, code_seg: &String) -> Point {
        let mut end_point = Point { row: start_position.row, column: start_position.column };
        let mut char_iter = code_seg.char_indices();
        
        while let Some(c) = char_iter.next() {
            end_point.column += 1;
            if c.1 == '\n' {
                end_point.row += 1;
                end_point.column = 0;
            }
        }

        end_point
    }

    fn handle_update(code: &String, line_bytes: &Vec<u64>, range: Range, text: &String) -> (String, InputEdit) {
        let mut start_byte: u64 = 0;
        let mut line: usize = 0;
        while line < range.start.line.try_into().unwrap() { //FIXME: Using try_into here is probably not a good idea
            start_byte += line_bytes.get(line).unwrap();
            line += 1;
        }
        let mut end_byte = start_byte; // Gets byte up to this line (not the characters from it)
        start_byte += range.start.character;

        while line < range.end.line.try_into().unwrap() {
            end_byte += line_bytes.get(line).unwrap();
            line += 1;
        }

        if line > range.start.line.try_into().unwrap() {
            end_byte += range.end.character;
        } else {
            end_byte += range.end.character;
        }

        let start_position = Point::new(range.start.line.try_into().unwrap(), range.start.character.try_into().unwrap());
        let removed_bytes: usize = (end_byte - start_byte).try_into().unwrap();
        let new_len: usize = code.len() - removed_bytes + text.len();
        let mut new_str = String::with_capacity(new_len);
        new_str.push_str(&code[0..start_byte.try_into().unwrap()]);
        new_str.push_str(&text);
        let new_end_position = SourceFile::find_end_position(&start_position, &text);
        new_str.push_str(&code[end_byte.try_into().unwrap()..]);

        let new_end_byte: usize = start_byte.try_into().unwrap();
        let new_end_byte: usize = new_end_byte + text.len();

        return (new_str, InputEdit {
            start_byte: start_byte.try_into().unwrap(), // FIXME
            old_end_byte: end_byte.try_into().unwrap(),
            new_end_byte,
            start_position: Point::new(range.start.line.try_into().unwrap(), range.start.character.try_into().unwrap()),
            old_end_position: Point::new(range.end.line.try_into().unwrap(), range.end.character.try_into().unwrap()),
            new_end_position,
        })
    }

    pub fn update(&mut self, parser: &mut Parser, changes: Vec<TextDocumentContentChangeEvent>) {
        if changes.len() < 1 {
            return;
        }
        
        for change in changes {
            if let Some(range) = change.range {
                let (new_str, input_edit) = SourceFile::handle_update(&self.code, &self.line_bytes, range, &change.text);

                self.tree.edit(&input_edit);
                
                self.tree = parser.parse(&new_str, Some(&self.tree)).unwrap();
                self.code = new_str;
                // FIXME: We may just update the vector instead of generating an entire new vector.
                self.line_bytes = SourceFile::create_line_bytes(&self.code);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use lsp_types::*;
    use tree_sitter::Point;
    use crate::sourceFile::SourceFile;

    #[test]
    fn it_adds_text_correctly() {
        let code = String::from("a()\nb() {}");
        let new_text = String::from("{}");
        let range = Range {
            start: Position {
                line: 0,
                character: 3,
            },
            end: Position {
                line: 0,
                character: 3,
            }
        };
        
        let line_bytes = SourceFile::create_line_bytes(&code);
        let (new_str, edit) = SourceFile::handle_update(&code, &line_bytes, range, &new_text);

        assert_eq!(new_str, "a(){}\nb() {}");
        assert_eq!(edit.start_byte, 3);
        assert_eq!(edit.old_end_byte, 3);
        assert_eq!(edit.new_end_byte, 5);
        assert_eq!(edit.start_position, Point::new(0, 3));
        assert_eq!(edit.old_end_position, Point::new(0, 3));
        assert_eq!(edit.new_end_position, Point::new(0, 5));
    }

    #[test]
    fn it_adds_text_correctly_multiline() {
        let code = String::from("a()\nb()\nz");
        let new_text = String::from("{\n    \n}");
        let range = Range {
            start: Position {
                line: 1,
                character: 3,
            },
            end: Position {
                line: 1,
                character: 3,
            }
        };
        
        let line_bytes = SourceFile::create_line_bytes(&code);
        let (new_str, edit) = SourceFile::handle_update(&code, &line_bytes, range, &new_text);

        assert_eq!(new_str, "a()\nb(){\n    \n}\nz");
        assert_eq!(edit.start_byte, 7);
        assert_eq!(edit.old_end_byte, 7);
        assert_eq!(edit.new_end_byte, 15);
        assert_eq!(edit.start_position, Point::new(1, 3));
        assert_eq!(edit.old_end_position, Point::new(1, 3));
        assert_eq!(edit.new_end_position, Point::new(3, 1));
    }

    #[test]
    fn it_dels_text_correctly() {
        let code = String::from("a()\nb() {}");
        let new_text = String::from("");
        let range = Range {
            start: Position {
                line: 0,
                character: 1,
            },
            end: Position {
                line: 0,
                character: 3,
            }
        };
        
        let line_bytes = SourceFile::create_line_bytes(&code);
        let (new_str, edit) = SourceFile::handle_update(&code, &line_bytes, range, &new_text);

        assert_eq!(new_str, "a\nb() {}");
        assert_eq!(edit.start_byte, 1);
        assert_eq!(edit.old_end_byte, 3);
        assert_eq!(edit.new_end_byte, 1);
        assert_eq!(edit.start_position, Point::new(0, 1));
        assert_eq!(edit.old_end_position, Point::new(0, 3));
        assert_eq!(edit.new_end_position, Point::new(0, 1));
    }

    #[test]
    fn it_dels_text_correctly_multiline() {
        let code = String::from("a() {\n    \n}\nb() {}");
        let new_text = String::from("");
        let range = Range {
            start: Position {
                line: 0,
                character: 3,
            },
            end: Position {
                line: 2,
                character: 1,
            }
        };
        
        let line_bytes = SourceFile::create_line_bytes(&code);
        let (new_str, edit) = SourceFile::handle_update(&code, &line_bytes, range, &new_text);

        assert_eq!(new_str, "a()\nb() {}");
        assert_eq!(edit.start_byte, 3);
        assert_eq!(edit.old_end_byte, 12);
        assert_eq!(edit.new_end_byte, 3);
        assert_eq!(edit.start_position, Point::new(0, 3));
        assert_eq!(edit.old_end_position, Point::new(2, 1));
        assert_eq!(edit.new_end_position, Point::new(0, 3));
    }

    #[test]
    fn it_replaces_text_with_less_correctly() {
        let code = String::from("a()\nb() {}");
        let new_text = String::from("{");
        let range = Range {
            start: Position {
                line: 0,
                character: 1,
            },
            end: Position {
                line: 0,
                character: 3,
            }
        };
        
        let line_bytes = SourceFile::create_line_bytes(&code);
        let (new_str, edit) = SourceFile::handle_update(&code, &line_bytes, range, &new_text);

        assert_eq!(new_str, "a{\nb() {}");
        assert_eq!(edit.start_byte, 1);
        assert_eq!(edit.old_end_byte, 3);
        assert_eq!(edit.new_end_byte, 2);
        assert_eq!(edit.start_position, Point::new(0, 1));
        assert_eq!(edit.old_end_position, Point::new(0, 3));
        assert_eq!(edit.new_end_position, Point::new(0, 2));
    }

    #[test]
    fn it_replaces_text_with_more_correctly() {
        let code = String::from("a()\nb() {}");
        let new_text = String::from("{{{");
        let range = Range {
            start: Position {
                line: 0,
                character: 1,
            },
            end: Position {
                line: 0,
                character: 3,
            }
        };
        
        let line_bytes = SourceFile::create_line_bytes(&code);
        let (new_str, edit) = SourceFile::handle_update(&code, &line_bytes, range, &new_text);

        assert_eq!(new_str, "a{{{\nb() {}");
        assert_eq!(edit.start_byte, 1);
        assert_eq!(edit.old_end_byte, 3);
        assert_eq!(edit.new_end_byte, 4);
        assert_eq!(edit.start_position, Point::new(0, 1));
        assert_eq!(edit.old_end_position, Point::new(0, 3));
        assert_eq!(edit.new_end_position, Point::new(0, 4));
    }
}