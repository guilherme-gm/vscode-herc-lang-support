use super::helpers::*;

pub fn format(cursor: &mut tree_sitter::TreeCursor, code: &String) -> String {
    cursor.goto_first_child();
    if cursor.field_name().is_some() {
        // map, x, y, dir
        // TODO: Add validations
        let pos_node = cursor.node();
        let map = code[pos_node.start_byte()..pos_node.end_byte()].to_string();

        let x = get_next_named(cursor, &code, "x").unwrap();
        let y = get_next_named(cursor, &code, "y").unwrap();
        let dir = get_next_named(cursor, &code, "dir");

        if let Some(dir) = dir {
            format!("{}, {}, {}, {}", map, x, y, dir)
        } else {
            format!("{}, {}, {}", map, x, y)
        }
    } else {
        String::from("-")
    }
}