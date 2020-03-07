use super::super::script_formatter::*;
use tree_sitter::Node;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("> exp_string: {:?}", node));
    let mut cursor = node.walk();
    cursor.goto_first_child();

    let mut multiline_format = false;
    if fmter.function_stack.len() > 0 {
        if let Some(cur_function) = fmter.function_stack.last() {
            if cur_function.eq_ignore_ascii_case("mes") || cur_function.eq_ignore_ascii_case("mesf") {
                multiline_format = true;
            }
        }
    }

    if fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("line"), Spacing::None, true) {
        
        if multiline_format {
            fmter.write_newline();
            fmter.set_indent(fmter.indent_level + 1);
        }

        while fmter.match_until_and_write_node(
            &mut cursor,
            FmtNode::Named("line"),
            if multiline_format { Spacing::Indent } else { Spacing::Space },
            false,
        ) {
            if !fmter.match_until(&mut cursor, FmtNode::Named("line"), false) {
                break;
            }
        }

        if multiline_format {
            fmter.set_indent(fmter.indent_level - 1);
        }
    }
}
