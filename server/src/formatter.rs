mod expressions;
mod script_formatter;
mod statements;
mod top_levels;

use crate::state::State;
use std::convert::TryInto;
use std::sync::{Arc, Mutex};
use std::vec::Vec;
use tower_lsp::lsp_types::*;

// Debugger
use std::io::prelude::*;
use std::net::TcpStream;

use crate::source_file::SourceFile;

use script_formatter::ScriptFormatter;

pub fn get_edits(
    dbg: &Mutex<TcpStream>,
    source: Arc<Mutex<SourceFile>>,
    state: &State,
    _options: FormattingOptions,
) -> Option<Vec<TextEdit>> {
    let mut dbg = dbg.lock().unwrap();
    let source = source.lock().unwrap();
    let tree = &source.tree;
    let mut edits: Vec<TextEdit> = Vec::new();
    let cmds = &state.commands.as_ref().unwrap();
    let mut fmter = ScriptFormatter {
        dbg: &mut dbg,
        code: &source.code,
        commands: cmds,
        file_cursor: (0, 0),
        edits: &mut edits,
        indent_level: 0,
        indent: String::from(""),
    };
    let end_pos = tree.root_node().end_position();
    let mut cursor = tree.root_node().walk();
    for top_level_node in tree.root_node().children(&mut cursor) {
        debug_!(fmter.dbg, format!(":> Top: {:?}", top_level_node));
        top_levels::resolve(&mut fmter, &top_level_node);
    }

    if fmter.file_cursor.0 < end_pos.row.try_into().unwrap() {
        fmter.edits.push(TextEdit {
            range: Range {
                start: Position {
                    line: fmter.file_cursor.0,
                    character: fmter.file_cursor.1,
                },
                end: Position {
                    line: end_pos.row.try_into().unwrap(),
                    character: end_pos.column.try_into().unwrap(),
                },
            },
            new_text: String::from(""),
        });
    } else if fmter.file_cursor.0 == (end_pos.row as u64)
        && fmter.file_cursor.1 < end_pos.column.try_into().unwrap()
    {
        fmter.edits.push(TextEdit {
            range: Range {
                start: Position {
                    line: fmter.file_cursor.0,
                    character: fmter.file_cursor.1,
                },
                end: Position {
                    line: fmter.file_cursor.0,
                    character: end_pos.column.try_into().unwrap(),
                },
            },
            new_text: String::from(""),
        });
    }

    // tODO: This is recreating the entire thing, how can we avoid it?
    // debug_!(fmter.dbg, format!("{:?}", fmter.edits));
    Some(fmter.edits.to_vec())
}
