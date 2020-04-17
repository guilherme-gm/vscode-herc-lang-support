use std::convert::TryInto;
use std::sync::{Arc, Mutex};
use std::vec::Vec;
use tower_lsp::lsp_types::*;
use tree_sitter::{Node, Point, TreeCursor};
use crate::state::{State};
use crate::script_commands::ScriptCommand;
use std::collections::HashMap;

// Debugger
use std::io::prelude::*;
use std::net::TcpStream;

use crate::source_file::SourceFile;

const ARGUMENT_LIST: &str = "argument_list";
const FUNCTION_EXPR: &str = "function_expression";

fn get_named_parent_kind(node: &Node) -> Option<String> {
    if let Some(parent) = node.parent() {
        if parent.is_named() {
            return Some(String::from(parent.kind()));
        }

        return get_named_parent_kind(&parent);
    }
    None
}

#[derive(Debug)]
struct FunctionContext {
    function: String,
    param: Option<u32>,
}

fn is_pos(token_end: &Point, cursor: &Position) -> bool {
    (cursor.line <= (token_end.row as u64)) && (cursor.character <= token_end.column as u64) //FIXME: This is unsaef
}

fn get_context(node: &Node, position: &Position, code: &String, dbg: &mut Option<TcpStream>) -> Option<FunctionContext> {
    let point = Point::new(
        position.line.try_into().unwrap(),
        position.character.try_into().unwrap(),
    );
    let cursor_node = node.descendant_for_point_range(point, point);
    
    if let Some(cursor_node) = cursor_node {
        let mut prev_node: Option<Node> = None;
        let mut fargs_node = cursor_node;
        while !fargs_node.kind().eq_ignore_ascii_case(ARGUMENT_LIST) {
            if let Some(pr_arg_node) = fargs_node.parent() {
                prev_node = Some(fargs_node);
                fargs_node = pr_arg_node;
                continue;
            } else {
                return None;
            }
        }

        let fn_node = fargs_node.parent().unwrap(); // FIXME: Should be safe, but better check...
        let fname_node = fn_node.child(0).unwrap(); // FIXME: THIS MUST BE CHECKED
        let fname = code[fname_node.start_byte()..fname_node.end_byte()].to_string();
        
        let mut arg_num: i32;
        // if let Some(arg_node) = prev_node {
        //     debug_!(dbg, "if");
        //     arg_num = -1;
        //     let mut cursor = fargs_node.walk();
        //     if cursor.goto_first_child() {
        //         while !cursor.node().eq(&arg_node) && cursor.goto_next_sibling() {
        //             if cursor.node().is_named() {
        //                 arg_num += 1;
        //             }
        //         }

        //         if arg_num == -1 { // Probably you just opened the ()
        //             arg_num = 0;
        //         }
        //     }
        // } else {
        // debug_!(dbg, "else");
        // this means fargs_node was already argument_list, we have to find the position
        arg_num = 0;
        let mut cursor = fargs_node.walk();
        if cursor.goto_first_child() {
            while !is_pos(&cursor.node().end_position(), position) && cursor.goto_next_sibling() {
                // debug_!(dbg, format!("Kind: {:?}", cursor.node().kind()));
                if cursor.node().kind().eq_ignore_ascii_case(&",") {
                    arg_num += 1;
                }
            }

            // debug_!(dbg, format!("Kind: {:?}", cursor.node().kind()));
            // if is_pos(&cursor.node().end_position(), position) && cursor.node().kind().eq_ignore_ascii_case(&",") {
            //     arg_num += 1;
            // }
        }
        // }

        if arg_num >= 0 && arg_num < fargs_node.child_count().try_into().unwrap() {
            return Some(FunctionContext {
                function: fname,
                param: Some(arg_num as u32),
            });
        } else {
            return Some(FunctionContext {
                function: fname,
                param: None,
            });
        }
        
        // NOTE: Although we are checking for named with is_named,
        // we can't get the actual node name, only the node kind
        // if cursor_node.is_named() {
        //     return node_kind_to_context(cursor_node.kind());
        // } else if let Some(parent_kind) = get_named_parent_kind(&cursor_node) {
        //     return node_kind_to_context(&parent_kind);
        // }
    }

    None
}

fn buildSignatureHelp(context: FunctionContext, commands: &HashMap<String, ScriptCommand>) -> Option<SignatureHelp> {
    if let Some(command) = commands.get(&context.function){
        let mut signatures: Vec<SignatureInformation> = Vec::new();
        let mut show_sign: i64 = -1;
        let mut min_param: i64 = 0;
        let mut active_parameter: Option<i64> = None;

        if let Some(t_min_param) = context.param {
            min_param = t_min_param.try_into().unwrap();
            active_parameter = Some(min_param);
        }
        
        let sign_count = command.signatures.len();
        if sign_count < 1 {
            return None;
        }

        for i in 0..sign_count {
            let mut lbl = format!("{}(", &context.function);
            let mut param_list: Vec<ParameterInformation> = Vec::new();
            
            let mut param_count = 0;
            let sign_lbl = command.signatures[i].clone();
            for param in &command.signature_params[i] {
                if let Some(param_info) = command.params.get(param) {
                    let param_lbl = format!("{}: {}", param.clone(), param_info.param_type.clone());
                    let param_lbl = ParameterLabel::Simple(param_lbl);
                    let doc = param_info.doc.clone();
                    if doc.eq("") {
                        param_list.push(ParameterInformation {
                            label: param_lbl,
                            documentation: None
                        });
                    } else {
                        param_list.push(ParameterInformation {
                            label: param_lbl,
                            documentation: Some(Documentation::MarkupContent(MarkupContent {
                                kind: MarkupKind::Markdown,
                                value: doc,
                            }))
                        });
                    }
                }

                param_count += 1;
            }

            if param_count > min_param {
                show_sign = i.try_into().unwrap();
            }

            lbl.push(')');

            if command.doc.eq("") {
                signatures.push(SignatureInformation {
                    label: sign_lbl,
                    documentation: None,
                    parameters: Some(param_list),
                });
            } else {
                signatures.push(SignatureInformation {
                    label: sign_lbl,
                    documentation: Some(Documentation::MarkupContent(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: command.doc.clone(),
                    })),
                    parameters: Some(param_list),
                });
            }
        }

        let mut active_signature = None;
        if show_sign >= 0 {
            active_signature = Some(show_sign);
        }

        return Some(SignatureHelp {
            signatures,
            active_signature,
            active_parameter,
        });
    }

    None
}

pub fn get_signature(
    dbg: &Mutex<Option<TcpStream>>,
    state: &State,
    source: Arc<Mutex<SourceFile>>,
    position: Position,
) -> Option<SignatureHelp> {
    let mut dbg = dbg.lock().unwrap();
    let source = source.lock().unwrap();
    let tree = &source.tree;
    
    if let Some(commands) = &state.commands {
        // debug_!(dbg, "Get Sign..");
        // debug_!(dbg, format!("Position: {:?}", position));
        // debug_!(dbg, format!("{:?}", tree.root_node().to_sexp()));
    
        let context = get_context(&tree.root_node(), &position, &source.code, &mut dbg);


        // debug_!(dbg, format!("context: {:?}", context));
        if let Some(context) = context {
            return buildSignatureHelp(context, &commands);
        }
    }
    
    None

    // Some(SignatureHelp {

    // })

    // if context == CodeContext::Expression {
    //     if let Some(commands) = &state.commands {
    //         for cmd_name in commands.keys() {
    //             items.push(make_script_cmd_completion(cmd_name.clone(), &commands.get(cmd_name).unwrap()));
    //         }
    //     }
    // }

    // debug_!(dbg, "--------");

    // Some(items)
}
