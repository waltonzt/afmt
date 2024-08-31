use crate::config::{Indent, Shape};
use crate::context::FmtContext;
use crate::utility::{get_indent_string, get_modifiers, get_parameters, get_value};
use crate::{define_struct, define_struct_and_enum};
use anyhow::{Context, Result};
use tree_sitter::Node;

pub trait Rewrite {
    fn rewrite(&self, context: &FmtContext, shape: &Shape) -> Option<String> {
        self.rewrite_result(context, shape).ok()
    }

    fn rewrite_result(&self, context: &FmtContext, shape: &Shape) -> Result<String>;
}

define_struct_and_enum!(
    true; ClassDeclaration => "class_declaration",
    true; FieldDeclaration => "field_declaration",
    true; MethodDeclaration => "method_declaration",
    false; ExpressionStatement => "expression_statement",
    true; SimpleStatement => "boolean" | "int",
    true; BinaryExpression => "binary_expression"
);

impl<'a, 'tree> ClassDeclaration<'a, 'tree> {}

impl<'a, 'tree> Rewrite for ClassDeclaration<'a, 'tree> {
    fn rewrite_result(&self, context: &FmtContext, shape: &Shape) -> Result<String> {
        let mut result = String::new();
        //result.push_str(&get_indent_string(&shape.indent));

        let modifier_nodes = get_modifiers(self.as_ast_node());
        let modifiers_doc = modifier_nodes
            .iter()
            .map(|n| get_value(n, context.source_code))
            .collect::<Vec<&str>>()
            .join(" ");

        result.push_str(&modifiers_doc);
        result.push(' ');

        let name_node = self
            .as_ast_node()
            .child_by_field_name("name")
            .context("mandatory name field missing")?;
        let name_node_value = get_value(&name_node, context.source_code);

        result.push_str(name_node_value);
        Ok(result)
    }
}

impl<'a, 'tree> Rewrite for MethodDeclaration<'a, 'tree> {
    fn rewrite_result(&self, context: &FmtContext, shape: &Shape) -> Result<String> {
        let mut result = String::new();
        //result.push_str(&get_indent_string(&shape.indent));

        let modifier_nodes = get_modifiers(self.as_ast_node());
        let modifiers_doc = modifier_nodes
            .iter()
            .map(|n| get_value(n, context.source_code))
            .collect::<Vec<&str>>()
            .join(" ");

        result.push_str(&modifiers_doc);
        result.push(' ');

        let type_node = self
            .as_ast_node()
            .child_by_field_name("type")
            .context("mandatory type field missing")?;
        let type_node_value = get_value(&type_node, context.source_code);
        result.push_str(type_node_value);
        result.push(' ');

        let name_node = self
            .as_ast_node()
            .child_by_field_name("name")
            .context("mandatory name field missing")?;
        let name_node_value = get_value(&name_node, context.source_code);
        result.push_str(name_node_value);

        result.push('(');
        let parameters_node = get_parameters(self.as_ast_node());
        let parameters_doc = parameters_node
            .iter()
            .map(|n| {
                let type_node = n.child_by_field_name("type").unwrap();
                let name_node = n.child_by_field_name("name").unwrap();
                let type_str = type_node
                    .utf8_text(context.source_code.as_bytes())
                    .ok()
                    .unwrap();
                let name_str = name_node
                    .utf8_text(context.source_code.as_bytes())
                    .ok()
                    .unwrap();
                let r = format!("{} {}", type_str, name_str);
                r
            })
            .collect::<Vec<String>>()
            .join(", ");

        result.push_str(&parameters_doc);
        result.push(')');

        Ok(result)
    }
}

impl<'a, 'tree> Rewrite for FieldDeclaration<'a, 'tree> {
    fn rewrite_result(&self, context: &FmtContext, shape: &Shape) -> Result<String> {
        let mut result = String::new();
        //result.push_str(&get_indent_string(&shape.indent));

        let modifier_nodes = get_modifiers(self.as_ast_node());
        let modifiers_doc = modifier_nodes
            .iter()
            .map(|n| {
                n.utf8_text(context.source_code.as_bytes())
                    .ok()
                    .unwrap_or_default()
            })
            .collect::<Vec<&str>>()
            .join(" ");

        result.push_str(&modifiers_doc);

        result.push(' ');

        let type_node = self
            .as_ast_node()
            .child_by_field_name("type")
            .context("mandatory type field missing")?;
        let type_node_value = get_value(&type_node, context.source_code);
        result.push_str(type_node_value);

        result.push(' ');

        let name_node = self
            .as_ast_node()
            .child_by_field_name("declarator")
            .context("mandatory declarator field missing")?
            .child_by_field_name("name")
            .context("mandatory name field missing")?;
        let name_node_value = get_value(&name_node, context.source_code);
        result.push_str(name_node_value);

        result.push(';');
        //let mut result = indent_lines(&result, shape);

        //println!("fieldD: result |{}|", result);
        Ok(result)
    }
}

impl<'a, 'tree> Rewrite for SimpleStatement<'a, 'tree> {
    fn rewrite_result(&self, context: &FmtContext, shape: &Shape) -> Result<String> {
        let mut result = String::new();
        //result.push_str(&get_indent_string(&shape.indent));

        let name_node_value = get_value(self.as_ast_node(), context.source_code);
        result.push_str(name_node_value);

        result.push(';');
        Ok(result)
    }
}

impl<'a, 'tree> Rewrite for BinaryExpression<'a, 'tree> {
    fn rewrite_result(&self, context: &FmtContext, shape: &Shape) -> Result<String> {
        let mut result = String::new();
        //result.push_str(&get_indent_string(&shape.indent));

        let name_node_value = get_value(self.as_ast_node(), context.source_code);
        result.push_str(name_node_value);

        result.push(';');
        Ok(result)
    }
}
