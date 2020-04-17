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
use std::net::TcpStream;

use crate::source_file::SourceFile;

use script_formatter::{FmtNode, ScriptFormatter};

pub fn get_edits(
    dbg: &Mutex<Option<TcpStream>>,
    source: Arc<Mutex<SourceFile>>,
    state: &State,
    _options: FormattingOptions,
) -> Option<Vec<TextEdit>> {
    let mut dbg = dbg.lock().unwrap();
    let source = source.lock().unwrap();
    let tree = &source.tree;
    let mut edits: Vec<TextEdit> = Vec::new();
    let cmds = &state.commands.as_ref().unwrap();
    let mut fmter = ScriptFormatter::new(&mut dbg, &source.code, cmds, &mut edits);
    let end_pos = tree.root_node().end_position();
    let mut cursor = tree.root_node().walk();
    let mut last_kind: &str = &"";
    if cursor.goto_first_child() {
        while fmter.match_until(&mut cursor, FmtNode::Named("definition"), false) {
            fmter.info(format!(":> Top: {:?}", cursor.node()));

            if !last_kind.eq_ignore_ascii_case("")
                && (last_kind.eq_ignore_ascii_case("script_def")
                    || cursor.node().kind().eq_ignore_ascii_case("script_def"))
            {
                fmter.write_newline();
            }
            top_levels::resolve(&mut fmter, &cursor.node());
            last_kind = cursor.node().kind();

            if !cursor.goto_next_sibling() {
                break;
            }
        }
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
