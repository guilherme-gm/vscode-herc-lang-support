use std::convert::TryInto;
use std::sync::{Arc, Mutex};
use std::vec::Vec;
use tower_lsp::lsp_types::*;
use tree_sitter::{Node, Point};
use crate::state::{State};
use crate::script_commands::ScriptCommand;

// Debugger
use std::io::prelude::*;
use std::net::TcpStream;

use crate::source_file::SourceFile;

#[derive(PartialEq)]
enum CodeContext {
    Invalid,
    Expression,
}

fn node_kind_to_context(kind: &str) -> CodeContext {
    match kind.to_lowercase().as_str() {
        "block"
        | "argument_list"
        | "parenthesized_expression"
        | "if_statement"
        | "case_statement"
        | "do_statement"
        | "for_statement"
        | "while_statement"
        | "return_statemenet"
        | "assignment_expression"
        | "binary_expression"
        | "unary_expression"
        | "function_expression"
        | "labeled_statement"
        | "update_expression" => return CodeContext::Expression,
        _ => CodeContext::Invalid,
    }
}

fn get_named_parent_kind(node: &Node) -> Option<String> {
    if let Some(parent) = node.parent() {
        if parent.is_named() {
            return Some(String::from(parent.kind()));
        }

        return get_named_parent_kind(&parent);
    }
    None
}

fn get_context(node: &Node, position: &Position, dbg: &mut TcpStream) -> CodeContext {
    let point = Point::new(
        position.line.try_into().unwrap(),
        position.character.try_into().unwrap(),
    );
    let cursor_node = node.descendant_for_point_range(point, point);
    debug_!(dbg, format!(">> Node: {:?}", cursor_node));
    debug_!(
        dbg,
        format!(">> Parent: {:?}", cursor_node.unwrap().parent())
    );
    if let Some(cursor_node) = cursor_node {
        // NOTE: Although we are checking for named with is_named,
        // we can't get the actual node name, only the node kind
        if cursor_node.is_named() {
            return node_kind_to_context(cursor_node.kind());
        } else if let Some(parent_kind) = get_named_parent_kind(&cursor_node) {
            return node_kind_to_context(&parent_kind);
        }
    }

    CodeContext::Invalid
}

fn make_script_cmd_completion(name: String, cmd: &ScriptCommand) -> CompletionItem {
    CompletionItem {
        label: name,
        kind: Some(CompletionItemKind::Function),
        detail: cmd.prototype.clone(),
        documentation: Some(Documentation::MarkupContent(MarkupContent {
            kind: MarkupKind::Markdown,
            value: cmd.doc.clone(),
        })),
        deprecated: None,
        preselect: None,
        sort_text: None,
        filter_text: None,
        insert_text: None,
        insert_text_format: None,
        text_edit: None,
        additional_text_edits: None,
        command: None,
        data: None,
        tags: None,
    }
}

pub fn get_completion(
    dbg: &Mutex<TcpStream>,
    state: &State,
    source: Arc<Mutex<SourceFile>>,
    position: Position,
) -> Vec<CompletionItem> {
    let mut dbg = dbg.lock().unwrap();
    let source = source.lock().unwrap();
    let tree = &source.tree;

    let mut items: Vec<CompletionItem> = Vec::new();
    debug_!(dbg, "Completing..");
    debug_!(dbg, format!("Position: {:?}", position));
    debug_!(dbg, format!("{:?}", tree.root_node().to_sexp()));

    let context = get_context(&tree.root_node(), &position, &mut dbg);

    if context == CodeContext::Expression {
        if let Some(commands) = &state.commands {
            for cmd_name in commands.keys() {
                items.push(make_script_cmd_completion(cmd_name.clone(), &commands.get(cmd_name).unwrap()));
            }
        }
    }

    debug_!(dbg, "--------");

    items
}
