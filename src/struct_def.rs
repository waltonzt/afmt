use crate::config::Config;
use crate::context::FmtContext;
use crate::def_struct;
use tree_sitter::Node;

pub trait FromNode<'a, 'tree> {
    fn new(node: &'a Node<'tree>) -> Self;
}

def_struct!(
    ClassDeclaration,
    FieldDeclaration,
    MethodDeclaration,
    EnumDeclaration,
    EnumConstant,
    EnumBody,
    Block,
    Statement,
    ExpressionStatement,
    DoStatement,
    WhileStatement,
    ForStatement,
    EnhancedForStatement,
    Value,
    SmallCaseValue,
    CapitalValue,
    SuperClass,
    Expression,
    ArrayAccess,
    PrimaryExpression,
    DmlExpression,
    DmlSecurityMode,
    DmlType,
    AssignmentExpression,
    LocalVariableDeclaration,
    VariableDeclarator,
    IfStatement,
    UpdateExpression,
    ParenthesizedExpression,
    Interfaces,
    LineComment,
    BlockComment,
    ReturnStatement,
    ArgumentList,
    TypeArguments,
    GenericType,
    ArrayInitializer,
    DimensionsExpr,
    ArrayType,
    MapInitializer,
    Annotation,
    AnnotationArgumentList,
    AnnotationKeyValue,
    Modifiers,
    ConstructorDeclaration,
    ConstructorBody,
    ExplicitConstructorInvocation,
    RunAsStatement,
    ScopedTypeIdentifier,
    ObjectCreationExpression,
    TryStatement,
    CatchClause,
    GroupByClause,
    HavingClause,
    HavingComparisonExpression,
    CatchFormalParameter,
    FinallyClause,
    ForClause,
    FieldAccess,
    InstanceOfExpression,
    CastExpression,
    TernaryExpression,
    MethodInvocation,
    AccessorList,
    AccessorDeclaration,
    QueryExpression,
    SoqlQuery,
    SoqlQueryBody,
    SoslQuery,
    SubQuery,
    SoslQueryBody,
    BinaryExpression,
    UnaryExpression,
    ArrayCreationExpression,
    MapCreationExpression,
    SelectClause,
    FromClause,
    StorageAlias,
    StorageIdentifier,
    WhereCluase,
    ComparisonExpression,
    OrExpression,
    NotExpression,
    FieldIdentifier,
    BoundApexExpression,
    LimitClause,
    OffsetClause,
    AllRowClause,
    AndExpression,
    FindClause,
    InClause,
    ReturningClause,
    UpdateClause,
    WithClause,
    OrderByClause,
    OrderExpression,
    Operator,
    OrderDirection,
    SobjectReturn,
    WithDivisionExpression,
    DateLiteral,
    DateLiteralWithParam,
    SwitchExpression,
    SwitchBlock,
    SwitchRule,
    SwitchLabel,
    StaticInitializer,
    InterfaceDeclaration,
    ThrowStatement,
    BreakStatement,
    ContinueStatement,
    TypeParameters,
    TypeParameter,
    TypeList,
    TriggerDeclaration,
    TriggerEvent,
    ClassLiteral,
    FunctionExpression,
    JavaType,
    JavaFieldAccess,
);
