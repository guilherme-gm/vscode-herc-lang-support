use crate::state::State;
use std::convert::TryInto;
use std::sync::{Arc, Mutex};
use std::vec::Vec;
use tower_lsp::lsp_types::*;

// Debugger
use std::io::prelude::*;
use std::net::TcpStream;

use crate::source_file::SourceFile;

mod block;
mod duplicate_def;
mod expressions;
mod function_def;
mod helpers;
mod position;
mod script_def;
mod shop_def;
mod spawn_def;
mod statements;
mod warp_def;

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
    let mut formatter_info: (u64, u64) = (0, 0);
    let end_pos = tree.root_node().end_position();
    let mut cursor = tree.root_node().walk();

    let cmds = &state.commands.as_ref().unwrap();
    for top_level_node in tree.root_node().children(&mut cursor) {
        match top_level_node.kind().to_lowercase().as_str() {
            "duplicate_def" => {
                edits.push(duplicate_def::format(
                    &mut dbg,
                    &top_level_node,
                    &source.code,
                    &mut formatter_info,
                ));
            }
            "function_def" => {
                function_def::format(
                    &mut dbg,
                    &top_level_node,
                    &source.code,
                    &mut formatter_info,
                    cmds,
                    &mut edits  
                );
            },
            "script_def" => {
                script_def::format(
                    &mut dbg,
                    &top_level_node,
                    &source.code,
                    &mut formatter_info,
                    cmds,
                    &mut edits
                );
            },
            "shop_def" => {
                shop_def::format(
                    &mut dbg,
                    &top_level_node,
                    &source.code,
                    &mut formatter_info,
                    &mut edits
                );
            },
            "spawn_def" => {
                spawn_def::format(
                    &mut dbg,
                    &top_level_node,
                    &source.code,
                    &mut formatter_info,
                    &mut edits
                );
            },
            "warp_def" => {
                warp_def::format(
                    &mut dbg,
                    &top_level_node,
                    &source.code,
                    &mut formatter_info,
                    &mut edits
                );
            },
            _ => continue,
        }

        debug_!(dbg, format!("{:?}", top_level_node));
    }

    if formatter_info.0 < end_pos.row.try_into().unwrap() {
        edits.push(TextEdit {
            range: Range {
                start: Position {
                    line: formatter_info.0,
                    character: 0
                },
                end: Position {
                    line: end_pos.row.try_into().unwrap(),
                    character: end_pos.column.try_into().unwrap()
                }
            },
            new_text: String::from("")
        });
    } else if formatter_info.0 == (end_pos.row as u64) && formatter_info.1 < end_pos.column.try_into().unwrap() {
        edits.push(TextEdit {
            range: Range {
                start: Position {
                    line: formatter_info.0,
                    character: formatter_info.1
                },
                end: Position {
                    line: formatter_info.0,
                    character: end_pos.column.try_into().unwrap()
                }
            },
            new_text: String::from("")
        });
    }

    // debug_!(dbg, "Completing..");
    // debug_!(dbg, format!("Position: {:?}", position));
    // debug_!(dbg, format!("{:?}", tree.root_node().to_sexp()));

    // let context = get_context(&tree.root_node(), &position, &mut dbg);

    // if context == CodeContext::Expression {
    //     if let Some(commands) = &state.commands {
    //         for cmd_name in commands.keys() {
    //             items.push(make_script_cmd_completion(cmd_name.clone(), &commands.get(cmd_name).unwrap()));
    //         }
    //     }
    // }

    // debug_!(dbg, "--------");

    // items
    Some(edits)
}
