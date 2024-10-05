use crate::{rewrite::Rewrite, struct_def::*};
use phf::phf_map;
use tree_sitter::Node;

type PhfMap = phf::Map<&'static str, for<'a, 'tree> fn(&'a Node<'tree>) -> Box<dyn Rewrite + 'a>>;

pub static COMMON_MAP: PhfMap = phf_map! {
    "boolean" => |node| Box::new(SmallCaseValue::new(node)),
    "boolean_type" => |node| Box::new(SmallCaseValue::new(node)),
    "identifier" => |node| Box::new(Value::new(node)),
    "int" => |node| Box::new(Value::new(node)),
    "string_literal" => |node| Box::new(Value::new(node)),
    "type_identifier" => |node| Box::new(Value::new(node)),
    "null_literal" => |node| Box::new(Value::new(node)),
    "decimal_floating_point_literal" => |node| Box::new(Value::new(node)),
    // --------------- split
    "trigger_declaration" => |node| Box::new(TriggerDeclaration::new(node)),
    "argument_list" => |node| Box::new(ArgumentList::new(node)),
    "continue_statement" => |node| Box::new(ContinueStatement::new(node)),
    "break_statement" => |node| Box::new(BreakStatement::new(node)),
    "throw_statement" => |node| Box::new(ThrowStatement::new(node)),
    "interface_declaration" => |node| Box::new(InterfaceDeclaration::new(node)),
    "static_initializer" => |node| Box::new(StaticInitializer::new(node)),
    "annotation_argument_list" => |node| Box::new(AnnotationArgumentList::new(node)),
    "annotation_key_value" => |node| Box::new(AnnotationKeyValue::new(node)),
    "parenthesized_expression" => |node| Box::new(ParenthesizedExpression::new(node)),
    "array_access" => |node| Box::new(ArrayAccess::new(node)),
    "array_creation_expression" => |node| Box::new(ArrayCreationExpression::new(node)),
    "array_initializer" => |node| Box::new(ArrayInitializer::new(node)),
    "array_type" => |node| Box::new(ArrayType::new(node)),
    "assignment_expression" => |node| Box::new(AssignmentExpression::new(node)),
    "binary_expression" => |node| Box::new(BinaryExpression::new(node)),
    "block" => |node| Box::new(Block::new(node)),
    "cast_expression" => |node| Box::new(CastExpression::new(node)),
    "class_declaration" => |node| Box::new(ClassDeclaration::new(node)),
    "constructor_declaration" => |node| Box::new(ConstructorDeclaration::new(node)),
    "dimensions_expr" => |node| Box::new(DimensionsExpr::new(node)),
    "dml_expression" => |node| Box::new(DmlExpression::new(node)),
    "dml_security_mode" => |node| Box::new(DmlSecurityMode::new(node)),
    "dml_type" => |node| Box::new(DmlType::new(node)),
    "do_statement" => |node| Box::new(DoStatement::new(node)),
    "enhanced_for_statement" => |node| Box::new(EnhancedForStatement::new(node)),
    "enum_declaration" => |node| Box::new(EnumDeclaration::new(node)),
    "enum_constant" => |node| Box::new(EnumConstant::new(node)),
    "explicit_constructor_invocation" => |node| Box::new(ExplicitConstructorInvocation::new(node)),
    "expression_statement" => |node| Box::new(ExpressionStatement::new(node)),
    "field_access" => |node| Box::new(FieldAccess::new(node)),
    "field_declaration" => |node| Box::new(FieldDeclaration::new(node)),
    "for_statement" => |node| Box::new(ForStatement::new(node)),
    "generic_type" => |node| Box::new(GenericType::new(node)),
    "if_statement" => |node| Box::new(IfStatement::new(node)),
    "instanceof_expression" => |node| Box::new(InstanceOfExpression::new(node)),
    "line_comment" => |node| Box::new(LineComment::new(node)),
    "block_comment" => |node| Box::new(BlockComment::new(node)),
    "local_variable_declaration" => |node| Box::new(LocalVariableDeclaration::new(node)),
    "map_creation_expression" => |node| Box::new(MapCreationExpression::new(node)),
    "method_declaration" => |node| Box::new(MethodDeclaration::new(node)),
    "method_invocation" => |node| Box::new(MethodInvocation::new(node)),
    "object_creation_expression" => |node| Box::new(ObjectCreationExpression::new(node)),
    "return_statement" => |node| Box::new(ReturnStatement::new(node)),
    "run_as_statement" => |node| Box::new(RunAsStatement::new(node)),
    "scoped_type_identifier" => |node| Box::new(ScopedTypeIdentifier::new(node)),
    "query_expression" => |node| Box::new(QueryExpression::new(node)),
    "soql_query" => |node| Box::new(SoqlQuery::new(node)),
    "ternary_expression" => |node| Box::new(TernaryExpression::new(node)),
    "try_statement" => |node| Box::new(TryStatement::new(node)),
    "unary_expression" => |node| Box::new(UnaryExpression::new(node)),
    "update_expression" => |node| Box::new(UpdateExpression::new(node)),
    "while_statement" => |node| Box::new(WhileStatement::new(node)),
    "switch_expression" => |node| Box::new(SwitchExpression::new(node)),
    "switch_rule" => |node| Box::new(SwitchRule::new(node)),
    "class_literal" => |node| Box::new(ClassLiteral::new(node)),
};

pub static EXP_MAP: PhfMap = phf_map! {
    "boolean" => |node| Box::new(SmallCaseValue::new(node)),
    "boolean_type" => |node| Box::new(SmallCaseValue::new(node)),
    "identifier" => |node| Box::new(Value::new(node)),
    "this" => |node| Box::new(Value::new(node)),
    "int" => |node| Box::new(Value::new(node)),
    "string_literal" => |node| Box::new(Value::new(node)),
    "type_identifier" => |node| Box::new(Value::new(node)),
    "null_literal" => |node| Box::new(SmallCaseValue::new(node)),
    "void_type" => |node| Box::new(SmallCaseValue::new(node)),
    "decimal_floating_point_literal" => |node| Box::new(SmallCaseValue::new(node)),
    // --------------- split
    "class_literal" => |node| Box::new(ClassLiteral::new(node)),
    "annotation_argument_list" => |node| Box::new(AnnotationArgumentList::new(node)),
    "annotation" => |node| Box::new(Annotation::new(node)),
    "array_access" => |node| Box::new(ArrayAccess::new(node)),
    "array_creation_expression" => |node| Box::new(ArrayCreationExpression::new(node)),
    "array_type" => |node| Box::new(ArrayType::new(node)),
    "assignment_expression" => |node| Box::new(AssignmentExpression::new(node)),
    "binary_expression" => |node| Box::new(BinaryExpression::new(node)),
    "cast_expression" => |node| Box::new(CastExpression::new(node)),
    "dimensions_expr" => |node| Box::new(DimensionsExpr::new(node)),
    "dml_expression" => |node| Box::new(DmlExpression::new(node)),
    "dml_security_mode" => |node| Box::new(DmlSecurityMode::new(node)),
    "dml_type" => |node| Box::new(DmlType::new(node)),
    "field_access" => |node| Box::new(FieldAccess::new(node)),
    "generic_type" => |node| Box::new(GenericType::new(node)),
    "instanceof_expression" => |node| Box::new(InstanceOfExpression::new(node)),
    "local_variable_declaration" => |node| Box::new(LocalVariableDeclaration::new(node)),
    "map_creation_expression" => |node| Box::new(MapCreationExpression::new(node)),
    "method_invocation" => |node| Box::new(MethodInvocation::new(node)),
    "object_creation_expression" => |node| Box::new(ObjectCreationExpression::new(node)),
    "parenthesized_expression" => |node| Box::new(ParenthesizedExpression::new(node)),
    "primary_expression" => |node| Box::new(PrimaryExpression::new(node)),
    "scoped_type_identifier" => |node| Box::new(ScopedTypeIdentifier::new(node)),
    "ternary_expression" => |node| Box::new(TernaryExpression::new(node)),
    "unary_expression" => |node| Box::new(UnaryExpression::new(node)),
    "update_expression" => |node| Box::new(UpdateExpression::new(node)),
    "query_expression" => |node| Box::new(QueryExpression::new(node)),
    "switch_block" => |node| Box::new(SwitchBlock::new(node)),
    "line_comment" => |node| Box::new(LineComment::new(node)),
    "block_comment" => |node| Box::new(BlockComment::new(node)),

    // --------------- split
    //"array_initializer" => |node| Box::new(ArrayInitializer::new(node)),
    //"assignment_operator" => |node| Box::new(Value::new(node)),
    //"block" => |node| Box::new(Block::new(node)),
    //"class_declaration" => |node| Box::new(ClassDeclaration::new(node)),
    //"constructor_declaration" => |node| Box::new(ConstructorDeclaration::new(node)),
    //"do_statement" => |node| Box::new(DoStatement::new(node)),
    //"enhanced_for_statement" => |node| Box::new(EnhancedForStatement::new(node)),
    //"enum_declaration" => |node| Box::new(EnumDeclaration::new(node)),
    //"explicit_constructor_invocation" => |node| Box::new(ExplicitConstructorInvocation::new(node)),
    //"expression_statement" => |node| Box::new(ExpressionStatement::new(node)),
    //"field_declaration" => |node| Box::new(FieldDeclaration::new(node)),
    //"for_statement" => |node| Box::new(ForStatement::new(node)),
    //"if_statement" => |node| Box::new(IfStatement::new(node)),
    //"local_variable_declaration" => |node| Box::new(LocalVariableDeclaration::new(node)),
    //"method_declaration" => |node| Box::new(MethodDeclaration::new(node)),
    //"return_statement" => |node| Box::new(ReturnStatement::new(node)),
    //"run_as_statement" => |node| Box::new(RunAsStatement::new(node)),
    //"try_statement" => |node| Box::new(TryStatement::new(node)),
    //"while_statement" => |node| Box::new(WhileStatement::new(node)),
};
