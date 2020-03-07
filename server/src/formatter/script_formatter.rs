use crate::script_commands::ScriptCommand;
use std::collections::HashMap;
use tower_lsp::lsp_types::*;
use tree_sitter::{Node, TreeCursor};

// Debugger
use std::io::prelude::*;
use std::net::TcpStream;

#[derive(Debug)]
pub enum FmtNode<'a> {
    Named(&'a str),
    Token(&'a str),
}

pub enum Spacing {
    None,
    Space,
    Indent,
}

pub struct ScriptFormatter<'a> {
    pub dbg: &'a mut TcpStream,
    pub code: &'a String,
    pub commands: &'a HashMap<String, ScriptCommand>,
    pub file_cursor: (u64, u64),
    pub edits: &'a mut Vec<TextEdit>,
    /**
     * Do not edit this directly out of this module
     */
    pub indent_level: u8,
    /**
     * Do not edit this directly
     */
    pub indent: String,
    /**
     * A stack structure of functions being formatted and
     * its nesting (last = the innermost function)
     */
    pub function_stack: Vec<String>,
}

impl<'a> ScriptFormatter<'a> {
    pub fn new(
        dbg: &'a mut TcpStream,
        code: &'a String,
        commands: &'a HashMap<String, ScriptCommand>,
        edits: &'a mut Vec<TextEdit>,
    ) -> Self {
        ScriptFormatter {
            dbg,
            code,
            commands,
            file_cursor: (0, 0),
            edits,
            indent_level: 0,
            indent: String::from(""),
            function_stack: Vec::new(),
        }
    }

    pub fn info(self: &mut ScriptFormatter<'a>, _text: String) {
        debug_!(self.dbg, _text);
    }

    pub fn get_node_text(self: &ScriptFormatter<'a>, node: &Node<'a>) -> String {
        self.code[node.start_byte()..node.end_byte()].to_string()
    }

    pub fn write_edit(self: &mut ScriptFormatter<'a>, base_text: String, spacing: Spacing) {
        let text;
        match spacing {
            Spacing::None => text = base_text,
            Spacing::Space => text = format!(" {}", base_text),
            Spacing::Indent => text = format!("{}{}", self.indent, base_text),
        }

        if text.len() == 0 {
            return;
        }

        let mut lines_num = text.lines().count() - 1;
        let last_line = text.lines().last();
        if last_line.is_none() {
            // Should never happen
            panic!("write_edit:: Missing last line");
        }

        let last_line = last_line.unwrap();
        let mut line_len = last_line.len();

        let last_char = text.chars().last();
        if last_char.is_none() {
            // Should never happen
            panic!("write_edit:: Missing last char");
        }

        let last_char = last_char.unwrap();
        if last_char.eq(&'\n') {
            line_len = 0;
            lines_num += 1;
        }

        let start_line = self.file_cursor.0;
        let start_char = self.file_cursor.1;

        self.file_cursor.0 += usize2u64!(lines_num);
        self.file_cursor.1 += usize2u64!(line_len);

        self.edits.push(TextEdit {
            range: Range {
                start: Position {
                    line: start_line,
                    character: start_char,
                },
                end: Position {
                    line: self.file_cursor.0,
                    character: self.file_cursor.1,
                },
            },
            new_text: text,
        });
    }
    pub fn write_space(self: &mut ScriptFormatter<'a>) {
        self.write_edit(String::from(" "), Spacing::None);
    }

    pub fn write_indent(self: &mut ScriptFormatter<'a>) {
        self.write_edit(String::from(""), Spacing::Indent);
    }

    pub fn write_newline(self: &mut ScriptFormatter<'a>) {
        self.write_edit(String::from("\n"), Spacing::None);
    }

    pub fn write_node(
        self: &mut ScriptFormatter<'a>,
        cursor: &mut TreeCursor,
        spacing: Spacing,
    ) -> bool {
        self.write_edit(self.get_node_text(&cursor.node()), spacing);
        cursor.goto_next_sibling()
    }

    fn write_as_is(self: &mut ScriptFormatter<'a>, node: &Node) {
        self.info(format!("skip:: Skipping {:?}", node));
        self.write_edit(self.get_node_text(node), Spacing::None);
        if node.kind().eq_ignore_ascii_case("comment") {
            self.write_newline();
        }
    }

    pub fn is_stop(
        self: &mut ScriptFormatter<'a>,
        cursor: &mut TreeCursor,
        stop: &FmtNode,
    ) -> bool {
        match stop {
            FmtNode::Named(name) => {
                if let Some(node_name) = cursor.field_name() {
                    return node_name.eq_ignore_ascii_case(name);
                }

                false
            }
            FmtNode::Token(tok) => {
                return cursor.node().kind().eq_ignore_ascii_case(tok);
            }
        }
    }

    fn is_one_stop(
        self: &mut ScriptFormatter<'a>,
        cursor: &mut TreeCursor,
        stops: &[FmtNode],
    ) -> bool {
        for stop in stops {
            if self.is_stop(cursor, stop) {
                return true;
            }
        }

        false
    }

    /**
     * Tries to match "stop", and return.
     * If "required" is true and "stop" is not found, Panics.
     *
     * Returns false if it couldn't find the "stop"
     *
     * Once the end of the cursor is reached, additional calls
     * to this function (or any other "match") will make it rerun
     * over the last node.
     */
    pub fn match_until(
        self: &mut ScriptFormatter<'a>,
        cursor: &mut TreeCursor,
        stop: FmtNode,
        required: bool,
    ) -> bool {
        self.info(format!("::> {:?}", cursor.node()));
        while !self.is_stop(cursor, &stop) {
            self.info(format!("::::> {:?}", cursor.node()));
            self.write_as_is(&cursor.node());

            if !cursor.goto_next_sibling() {
                if required {
                    panic!(format!("Failed to find {:?}", stop));
                }

                self.info(format!(
                    "match_until:: Reached end while searching {:?}",
                    stop
                ));
                return false;
            }
        }

        true
    }

    /**
     * Tries to match one of the "stop", and return.
     * If "required" is true and "stop" is not found, Panics.
     *
     * Returns false if it couldn't find the "stop"
     *
     * Once the end of the cursor is reached, additional calls
     * to this function (or any other "match") will make it rerun
     * over the last node.
     */
    pub fn match_until_one(
        self: &mut ScriptFormatter<'a>,
        cursor: &mut TreeCursor,
        stops: &[FmtNode],
        required: bool,
    ) -> bool {
        while !self.is_one_stop(cursor, &stops) {
            self.write_as_is(&cursor.node());

            if !cursor.goto_next_sibling() {
                if required {
                    panic!(format!("Failed to find one of: {:?}", stops));
                }

                self.info(format!(
                    "match_until:: Reached end while searching for one of {:?}",
                    stops
                ));
                return false;
            }
        }

        true
    }

    /**
     * Tries to mach "stop", if it matches, writes the node
     * itself and advances the cursor.
     * If "required" is true and "stop" is not found, Panics.
     *
     * Returns false if it couldn't find the "stop" OR IT DID,
     * WROTE STR AND THE END OF CURSOR WAS REACHED.
     *
     * Additional calls to this function (or any other "match")
     * will make it rerun over the last node.
     */
    pub fn match_until_and_write_node(
        self: &mut ScriptFormatter<'a>,
        cursor: &mut TreeCursor,
        stop: FmtNode,
        spacing: Spacing,
        required: bool,
    ) -> bool {
        if !self.match_until(cursor, stop, required) {
            return false;
        }
        self.write_node(cursor, spacing)
    }

    /**
     * Tries to mach "stop", if it matches, writes "replace",
     * and advances the cursor.
     * If "required" is true and "stop" is not found, Panics.
     *
     * Returns false if it couldn't find the "stop" OR IT DID,
     * WROTE STR AND THE END OF CURSOR WAS REACHED.
     *
     * Additional calls to this function (or any other "match")
     * will make it rerun over the last node.
     */
    pub fn match_until_and_write_str(
        self: &mut ScriptFormatter<'a>,
        cursor: &mut TreeCursor,
        stop: FmtNode,
        replace: &str,
        spacing: Spacing,
        required: bool,
    ) -> bool {
        if !self.match_until(cursor, stop, required) {
            return false;
        }
        self.write_edit(String::from(replace), spacing);
        cursor.goto_next_sibling()
    }

    pub fn set_indent(self: &mut ScriptFormatter<'a>, level: u8) {
        self.indent_level = level;
        self.indent = str::repeat("\t", level as usize);
    }

    pub fn is_command(self: &mut ScriptFormatter<'a>, node: &Node) -> bool {
        if !node.kind().eq_ignore_ascii_case("identifier") {
            return false;
        }

        if !self.commands.contains_key(&self.get_node_text(node)) {
            return false;
        }

        true
    }
}
