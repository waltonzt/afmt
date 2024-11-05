use crate::data_model::*;
use colored::Colorize;
#[allow(unused_imports)]
use log::debug;
use std::cell::Cell;
use tree_sitter::{Node, Tree, TreeCursor};

thread_local! {
    static THREAD_SOURCE_CODE: Cell<Option<&'static str>> = Cell::new(None);
}

/// Sets the source code for the current thread.
/// This should be called once per thread before processing.
pub fn set_thread_source_code(code: String) {
    // Leak the `String` to obtain a `&'static str`
    let leaked_code: &'static str = Box::leak(code.into_boxed_str());
    THREAD_SOURCE_CODE.with(|sc| {
        sc.set(Some(leaked_code));
    });
}

/// Retrieves the source code for the current thread.
/// Panics if the source code has not been set.
pub fn source_code() -> &'static str {
    THREAD_SOURCE_CODE.with(|sc| sc.get().expect("Source code not set for this thread"))
}

pub fn collect_comments(cursor: &mut TreeCursor, comments: &mut Vec<Comment>) {
    loop {
        let node = cursor.node();
        if node.is_named() && node.is_extra() {
            comments.push(Comment::from_node(node));
        }

        if cursor.goto_first_child() {
            collect_comments(cursor, comments);
            cursor.goto_parent();
        }

        if !cursor.goto_next_sibling() {
            break;
        }
    }
}

pub fn enrich(ast_tree: &Tree) -> Root {
    let root_node = ast_tree.root_node();
    Root::new(root_node)
    //eprintln!("Root={:#?}", std::mem::size_of::<Root>());
    //eprintln!("Class={:#?}", std::mem::size_of::<FieldDeclaration>());
}

pub fn assert_check(node: Node, expected_kind: &str) {
    assert!(
        node.kind() == expected_kind,
        "Expected node kind '{}', found '{}'",
        expected_kind.yellow(),
        node.kind().red()
    );
}

//pub fn visit_root(context: &FmtContext) -> String {
//    let mut result = String::new();
//    let shape = Shape::empty(&context.config);
//    let root = &context.ast_tree.root_node();
//
//    let mut cursor = root.walk();
//    let children = root
//        .named_children(&mut cursor)
//        .map(|child| -> _ {
//            let mut child_shape = shape.clone_with_standalone(true);
//            child._visit(&mut child_shape, context)
//        })
//        .collect::<Vec<_>>()
//        .join("");
//
//    result.push_str(&children);
//
//    // remove the extra "\n" introduced by the top-level class declaration
//    result.truncate(result.trim_end_matches('\n').len());
//    result
//}

//pub fn get_length_before_brace(s: &str) -> usize {
//    s.find('{')
//        .map(|i| s[..i].trim_end().len())
//        .unwrap_or(s.len())
//}
//
//pub fn try_add_pref(result: &mut String, shape: &mut Shape, context: &FmtContext) {
//    if shape.standalone {
//        add_prefix(result, shape, context);
//    }
//}
//
//pub fn try_add_pref_and_offset(result: &mut String, shape: &mut Shape, context: &FmtContext) {
//    if shape.standalone {
//        add_prefix(result, shape, context);
//        shape.add_offset(1); // the possible line trailing `;`
//    }
//}
//
//pub fn add_prefix(result: &mut String, shape: &Shape, context: &FmtContext) {
//    add_indent(result, shape, context);
//    //shape.offset = shape.indent_len(context.config);
//}
//
//pub fn add_prefix_for_comment(
//    node: &Node,
//    result: &mut String,
//    shape: &Shape,
//    context: &FmtContext,
//) {
//    let needs_indent = match node.prev_named_sibling() {
//        Some(prev) => node.start_position().row != prev.end_position().row,
//        None => true,
//    };
//
//    // the preceeding number of `\n` is already proceeded by the last node
//    if needs_indent {
//        add_indent(result, shape, context);
//    }
//}
//
//pub fn try_add_standalone_suffix(
//    node: &Node,
//    result: &mut String,
//    shape: &Shape,
//    source_code: &str,
//) {
//    if shape.standalone {
//        add_standalone_suffix(node, result, source_code);
//    }
//}
//
//pub fn add_standalone_suffix(node: &Node, result: &mut String, source_code: &str) {
//    result.push(';');
//    add_standalone_suffix_no_semicolumn(node, result, source_code);
//}
//
//pub fn try_add_standalone_suffix_no_semicolumn(
//    node: &Node,
//    result: &mut String,
//    shape: &Shape,
//    source_code: &str,
//) {
//    if shape.standalone {
//        add_standalone_suffix_no_semicolumn(node, result, source_code);
//    }
//}
//
//pub fn add_standalone_suffix_no_semicolumn(node: &Node, result: &mut String, source_code: &str) {
//    if let Some(next) = node.next_named_sibling() {
//        let is_next_comment = next.kind() == "line_comment" || next.kind() == "block_comment";
//        let same_line = node.end_position().row == next.start_position().row;
//
//        if is_next_comment && same_line {
//            result.push(' ');
//        } else {
//            let count_new_lines = newlines_to_add(node, source_code);
//            result.push_str(&"\n".repeat(count_new_lines));
//        }
//    }
//}
//
//pub fn add_indent(result: &mut String, shape: &Shape, context: &FmtContext) {
//    result.push_str(&shape.indent.as_string(&context.config));
//}
//
//fn newlines_to_add(node: &Node, source_code: &str) -> usize {
//    let index = node.end_byte();
//    if index >= source_code.len() {
//        return 0;
//    }
//
//    let remaining_code = &source_code[index..];
//    let mut newline_count = 0;
//    let mut found_non_whitespace = false;
//
//    for char in remaining_code.chars() {
//        match char {
//            '\n' => {
//                newline_count += 1;
//                if newline_count >= 2 {
//                    break;
//                }
//            }
//            ' ' | '\t' | '\r' => continue,
//            _ => {
//                found_non_whitespace = true;
//                break;
//            }
//        }
//    }
//
//    if found_non_whitespace && newline_count == 0 {
//        1
//    } else {
//        newline_count
//    }
//}
//
//pub fn rewrite<'a, 't, T>(n: &'a Node<'t>, shape: &mut Shape, context: &FmtContext) -> String
//where
//    T: FromNode<'a, 't> + Rewrite,
//{
//    let block = T::new(n);
//    block.rewrite(shape, context)
//}
//
//pub fn rewrite_shape<'a, 't, T>(
//    n: &'a Node<'t>,
//    shape: &mut Shape,
//    is_standalone: bool,
//    context: &FmtContext,
//) -> String
//where
//    T: FromNode<'a, 't> + Rewrite,
//{
//    let block = T::new(n);
//    let cloned = &mut shape.clone_with_standalone(is_standalone);
//    block.rewrite(cloned, context)
//}
//
//pub fn update_source_code_for_if_statement(node: &Node, source_code: &str) -> Option<String> {
//    let node_code = node.v(source_code);
//    let mut is_updated = false;
//    let mut new_node_code = String::from(node_code);
//
//    if node.c_by_n("consequence").kind() != "block" {
//        let consequence_code = node.c_by_n("consequence").v(source_code);
//        new_node_code =
//            new_node_code.replace(consequence_code, &format!("{{ {} }}", consequence_code));
//        is_updated = true;
//    }
//
//    if let Some(a) = node.try_c_by_n("alternative") {
//        if a.kind() != "block" && a.kind() != "if_statement" {
//            let alternative_code = a.v(source_code);
//            new_node_code =
//                new_node_code.replace(alternative_code, &format!("{{ {} }}", alternative_code));
//            is_updated = true;
//        }
//    }
//
//    if is_updated {
//        Some(new_node_code)
//    } else {
//        None
//    }
//}
//
//pub fn is_parent_where_clause(node: &Node<'_>) -> bool {
//    if let Some(p) = node.parent() {
//        if p.kind() == "where_clause" {
//            return true;
//        }
//    }
//    false
//}
//
//pub fn split_and_rewrite_directly(node: &Node, shape: &mut Shape, context: &FmtContext) -> String {
//    match node.kind() {
//        "binary_expression" => BinaryExpression::new(node).rewrite_multi_line(shape, context),
//        _ => rewrite::<Expression>(node, shape, context),
//    }
//}
