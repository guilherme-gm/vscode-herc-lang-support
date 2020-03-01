use tree_sitter::{Node};

pub fn goto_next_named(cursor: &mut tree_sitter::TreeCursor) -> bool {
    while cursor.goto_next_sibling() {
        if cursor.field_name().is_some() {
            return true;
        }
    }

    false
}

pub fn get_node_text(node: &Node, code: &String) -> String {
    code[node.start_byte()..node.end_byte()].to_string()
}

pub fn get_next_named(
    cursor: &mut tree_sitter::TreeCursor,
    code: &String,
    name: &str,
) -> Option<String> {
    while cursor.goto_next_sibling() {
        if let Some(field) = cursor.field_name() {
            if field.eq_ignore_ascii_case(&name.to_lowercase()) {
                return Some(get_node_text(&cursor.node(), &code));
            }
        }
    }

    None
}

pub fn goto_name(
    cursor: &mut tree_sitter::TreeCursor,
    name: &str,
) -> bool {
    while cursor.goto_next_sibling() {
        if let Some(field) = cursor.field_name() {
            if field.eq_ignore_ascii_case(&name.to_lowercase()) {
                return true;
            }
        }
    }

    false
}
