// Code generated; DO NOT EDIT.

use num_derive::FromPrimitive;

#[derive(Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum Javascript {
    End = 0,
    Identifier = 1,
    HashBangLine = 2,
    Export = 3,
    STAR = 4,
    Default = 5,
    LBRACE = 6,
    COMMA = 7,
    RBRACE = 8,
    As = 9,
    Import2 = 10,
    From = 11,
    Var = 12,
    Let = 13,
    Const = 14,
    Else = 15,
    If = 16,
    Switch = 17,
    For = 18,
    LPAREN = 19,
    RPAREN = 20,
    Await = 21,
    In = 22,
    Of = 23,
    While = 24,
    Do = 25,
    Try = 26,
    With = 27,
    Break = 28,
    Continue = 29,
    Debugger = 30,
    Return = 31,
    Throw = 32,
    SEMI = 33,
    COLON = 34,
    Case = 35,
    Catch = 36,
    Finally = 37,
    Yield = 38,
    EQ = 39,
    LBRACK = 40,
    RBRACK = 41,
    LT = 42,
    GT = 43,
    SLASH = 44,
    JsxText = 45,
    Identifier2 = 46,
    DOT = 47,
    Class2 = 48,
    Extends = 49,
    Async = 50,
    Function2 = 51,
    EQGT = 52,
    QMARKDOT = 53,
    New = 54,
    PLUSEQ = 55,
    DASHEQ = 56,
    STAREQ = 57,
    SLASHEQ = 58,
    PERCENTEQ = 59,
    CARETEQ = 60,
    AMPEQ = 61,
    PIPEEQ = 62,
    GTGTEQ = 63,
    GTGTGTEQ = 64,
    LTLTEQ = 65,
    STARSTAREQ = 66,
    AMPAMPEQ = 67,
    PIPEPIPEEQ = 68,
    QMARKQMARKEQ = 69,
    DOTDOTDOT = 70,
    AMPAMP = 71,
    PIPEPIPE = 72,
    GTGT = 73,
    GTGTGT = 74,
    LTLT = 75,
    AMP = 76,
    CARET = 77,
    PIPE = 78,
    PLUS = 79,
    DASH = 80,
    PERCENT = 81,
    STARSTAR = 82,
    LTEQ = 83,
    EQEQ = 84,
    EQEQEQ = 85,
    BANGEQ = 86,
    BANGEQEQ = 87,
    GTEQ = 88,
    QMARKQMARK = 89,
    Instanceof = 90,
    BANG = 91,
    TILDE = 92,
    Typeof = 93,
    Void = 94,
    Delete = 95,
    PLUSPLUS = 96,
    DASHDASH = 97,
    DQUOTE = 98,
    SQUOTE = 99,
    StringFragment = 100,
    StringFragment2 = 101,
    EscapeSequence = 102,
    Comment = 103,
    BQUOTE = 104,
    DOLLARLBRACE = 105,
    SLASH2 = 106,
    RegexPattern = 107,
    RegexFlags = 108,
    Number = 109,
    PrivatePropertyIdentifier = 110,
    Target = 111,
    This = 112,
    Super = 113,
    True = 114,
    False = 115,
    Null = 116,
    Undefined = 117,
    AT = 118,
    Static = 119,
    Get = 120,
    Set = 121,
    AutomaticSemicolon = 122,
    TemplateChars = 123,
    QMARK = 124,
    Program = 125,
    ExportStatement = 126,
    ExportClause = 127,
    ExportSpecifier = 128,
    Declaration = 129,
    Import = 130,
    ImportStatement = 131,
    ImportClause = 132,
    FromClause = 133,
    NamespaceImport = 134,
    NamedImports = 135,
    ExpressionStatement = 136,
    VariableDeclaration = 137,
    LexicalDeclaration = 138,
    VariableDeclarator = 139,
    StatementBlock = 140,
    ElseClause = 141,
    IfStatement = 142,
    SwitchStatement = 143,
    ForStatement = 144,
    ForInStatement = 145,
    ForHeader = 146,
    WhileStatement = 147,
    DoStatement = 148,
    TryStatement = 149,
    WithStatement = 150,
    BreakStatement = 151,
    ContinueStatement = 152,
    DebuggerStatement = 153,
    ReturnStatement = 154,
    ThrowStatement = 155,
    EmptyStatement = 156,
    LabeledStatement = 157,
    SwitchBody = 158,
    SwitchCase = 159,
    SwitchDefault = 160,
    CatchClause = 161,
    FinallyClause = 162,
    ParenthesizedExpression = 163,
    Expression = 164,
    PrimaryExpression = 165,
    YieldExpression = 166,
    Object = 167,
    ObjectPattern = 168,
    AssignmentPattern = 169,
    ObjectAssignmentPattern = 170,
    Array = 171,
    ArrayPattern = 172,
    JsxElement = 173,
    JsxFragment = 174,
    JsxExpression = 175,
    JsxOpeningElement = 176,
    NestedIdentifier = 177,
    JsxNamespaceName = 178,
    JsxClosingElement = 179,
    JsxSelfClosingElement = 180,
    JsxAttribute = 181,
    Class = 182,
    ClassDeclaration = 183,
    ClassHeritage = 184,
    Function = 185,
    FunctionDeclaration = 186,
    GeneratorFunction = 187,
    GeneratorFunctionDeclaration = 188,
    ArrowFunction = 189,
    CallExpression = 190,
    NewExpression = 191,
    AwaitExpression = 192,
    MemberExpression = 193,
    SubscriptExpression = 194,
    AssignmentExpression = 195,
    AugmentedAssignmentLhs = 196,
    AugmentedAssignmentExpression = 197,
    Initializer = 198,
    DestructuringPattern = 199,
    SpreadElement = 200,
    TernaryExpression = 201,
    BinaryExpression = 202,
    UnaryExpression = 203,
    UpdateExpression = 204,
    SequenceExpression = 205,
    String = 206,
    TemplateString = 207,
    TemplateSubstitution = 208,
    Regex = 209,
    MetaProperty = 210,
    Arguments = 211,
    Decorator = 212,
    MemberExpression2 = 213,
    CallExpression2 = 214,
    ClassBody = 215,
    FieldDefinition = 216,
    FormalParameters = 217,
    Pattern = 218,
    RestPattern = 219,
    MethodDefinition = 220,
    Pair = 221,
    PairPattern = 222,
    PropertyName = 223,
    ComputedPropertyName = 224,
    ProgramRepeat1 = 225,
    ExportStatementRepeat1 = 226,
    ExportClauseRepeat1 = 227,
    NamedImportsRepeat1 = 228,
    VariableDeclarationRepeat1 = 229,
    SwitchBodyRepeat1 = 230,
    ObjectRepeat1 = 231,
    ObjectPatternRepeat1 = 232,
    ArrayRepeat1 = 233,
    ArrayPatternRepeat1 = 234,
    JsxElementRepeat1 = 235,
    JsxOpeningElementRepeat1 = 236,
    StringRepeat1 = 237,
    StringRepeat2 = 238,
    TemplateStringRepeat1 = 239,
    ClassBodyRepeat1 = 240,
    FormalParametersRepeat1 = 241,
    ImportSpecifier = 242,
    NamespaceExport = 243,
    PropertyIdentifier = 244,
    ShorthandPropertyIdentifier = 245,
    ShorthandPropertyIdentifierPattern = 246,
    StatementIdentifier = 247,
    Error = 248,
}

impl From<Javascript> for &'static str {
    #[inline(always)]
    fn from(tok: Javascript) -> Self {
        match tok {
            Javascript::End => "end",
            Javascript::Identifier => "identifier",
            Javascript::HashBangLine => "hash_bang_line",
            Javascript::Export => "export",
            Javascript::STAR => "*",
            Javascript::Default => "default",
            Javascript::LBRACE => "{",
            Javascript::COMMA => ",",
            Javascript::RBRACE => "}",
            Javascript::As => "as",
            Javascript::Import2 => "import",
            Javascript::From => "from",
            Javascript::Var => "var",
            Javascript::Let => "let",
            Javascript::Const => "const",
            Javascript::Else => "else",
            Javascript::If => "if",
            Javascript::Switch => "switch",
            Javascript::For => "for",
            Javascript::LPAREN => "(",
            Javascript::RPAREN => ")",
            Javascript::Await => "await",
            Javascript::In => "in",
            Javascript::Of => "of",
            Javascript::While => "while",
            Javascript::Do => "do",
            Javascript::Try => "try",
            Javascript::With => "with",
            Javascript::Break => "break",
            Javascript::Continue => "continue",
            Javascript::Debugger => "debugger",
            Javascript::Return => "return",
            Javascript::Throw => "throw",
            Javascript::SEMI => ";",
            Javascript::COLON => ":",
            Javascript::Case => "case",
            Javascript::Catch => "catch",
            Javascript::Finally => "finally",
            Javascript::Yield => "yield",
            Javascript::EQ => "=",
            Javascript::LBRACK => "[",
            Javascript::RBRACK => "]",
            Javascript::LT => "<",
            Javascript::GT => ">",
            Javascript::SLASH => "/",
            Javascript::JsxText => "jsx_text",
            Javascript::Identifier2 => "identifier",
            Javascript::DOT => ".",
            Javascript::Class2 => "class",
            Javascript::Extends => "extends",
            Javascript::Async => "async",
            Javascript::Function2 => "function",
            Javascript::EQGT => "=>",
            Javascript::QMARKDOT => "?.",
            Javascript::New => "new",
            Javascript::PLUSEQ => "+=",
            Javascript::DASHEQ => "-=",
            Javascript::STAREQ => "*=",
            Javascript::SLASHEQ => "/=",
            Javascript::PERCENTEQ => "%=",
            Javascript::CARETEQ => "^=",
            Javascript::AMPEQ => "&=",
            Javascript::PIPEEQ => "|=",
            Javascript::GTGTEQ => ">>=",
            Javascript::GTGTGTEQ => ">>>=",
            Javascript::LTLTEQ => "<<=",
            Javascript::STARSTAREQ => "**=",
            Javascript::AMPAMPEQ => "&&=",
            Javascript::PIPEPIPEEQ => "||=",
            Javascript::QMARKQMARKEQ => "??=",
            Javascript::DOTDOTDOT => "...",
            Javascript::AMPAMP => "&&",
            Javascript::PIPEPIPE => "||",
            Javascript::GTGT => ">>",
            Javascript::GTGTGT => ">>>",
            Javascript::LTLT => "<<",
            Javascript::AMP => "&",
            Javascript::CARET => "^",
            Javascript::PIPE => "|",
            Javascript::PLUS => "+",
            Javascript::DASH => "-",
            Javascript::PERCENT => "%",
            Javascript::STARSTAR => "**",
            Javascript::LTEQ => "<=",
            Javascript::EQEQ => "==",
            Javascript::EQEQEQ => "===",
            Javascript::BANGEQ => "!=",
            Javascript::BANGEQEQ => "!==",
            Javascript::GTEQ => ">=",
            Javascript::QMARKQMARK => "??",
            Javascript::Instanceof => "instanceof",
            Javascript::BANG => "!",
            Javascript::TILDE => "~",
            Javascript::Typeof => "typeof",
            Javascript::Void => "void",
            Javascript::Delete => "delete",
            Javascript::PLUSPLUS => "++",
            Javascript::DASHDASH => "--",
            Javascript::DQUOTE => "\"",
            Javascript::SQUOTE => "'",
            Javascript::StringFragment => "string_fragment",
            Javascript::StringFragment2 => "string_fragment",
            Javascript::EscapeSequence => "escape_sequence",
            Javascript::Comment => "comment",
            Javascript::BQUOTE => "`",
            Javascript::DOLLARLBRACE => "${",
            Javascript::SLASH2 => "/",
            Javascript::RegexPattern => "regex_pattern",
            Javascript::RegexFlags => "regex_flags",
            Javascript::Number => "number",
            Javascript::PrivatePropertyIdentifier => "private_property_identifier",
            Javascript::Target => "target",
            Javascript::This => "this",
            Javascript::Super => "super",
            Javascript::True => "true",
            Javascript::False => "false",
            Javascript::Null => "null",
            Javascript::Undefined => "undefined",
            Javascript::AT => "@",
            Javascript::Static => "static",
            Javascript::Get => "get",
            Javascript::Set => "set",
            Javascript::AutomaticSemicolon => "_automatic_semicolon",
            Javascript::TemplateChars => "_template_chars",
            Javascript::QMARK => "?",
            Javascript::Program => "program",
            Javascript::ExportStatement => "export_statement",
            Javascript::ExportClause => "export_clause",
            Javascript::ExportSpecifier => "export_specifier",
            Javascript::Declaration => "declaration",
            Javascript::Import => "import",
            Javascript::ImportStatement => "import_statement",
            Javascript::ImportClause => "import_clause",
            Javascript::FromClause => "_from_clause",
            Javascript::NamespaceImport => "namespace_import",
            Javascript::NamedImports => "named_imports",
            Javascript::ExpressionStatement => "expression_statement",
            Javascript::VariableDeclaration => "variable_declaration",
            Javascript::LexicalDeclaration => "lexical_declaration",
            Javascript::VariableDeclarator => "variable_declarator",
            Javascript::StatementBlock => "statement_block",
            Javascript::ElseClause => "else_clause",
            Javascript::IfStatement => "if_statement",
            Javascript::SwitchStatement => "switch_statement",
            Javascript::ForStatement => "for_statement",
            Javascript::ForInStatement => "for_in_statement",
            Javascript::ForHeader => "_for_header",
            Javascript::WhileStatement => "while_statement",
            Javascript::DoStatement => "do_statement",
            Javascript::TryStatement => "try_statement",
            Javascript::WithStatement => "with_statement",
            Javascript::BreakStatement => "break_statement",
            Javascript::ContinueStatement => "continue_statement",
            Javascript::DebuggerStatement => "debugger_statement",
            Javascript::ReturnStatement => "return_statement",
            Javascript::ThrowStatement => "throw_statement",
            Javascript::EmptyStatement => "empty_statement",
            Javascript::LabeledStatement => "labeled_statement",
            Javascript::SwitchBody => "switch_body",
            Javascript::SwitchCase => "switch_case",
            Javascript::SwitchDefault => "switch_default",
            Javascript::CatchClause => "catch_clause",
            Javascript::FinallyClause => "finally_clause",
            Javascript::ParenthesizedExpression => "parenthesized_expression",
            Javascript::Expression => "expression",
            Javascript::PrimaryExpression => "primary_expression",
            Javascript::YieldExpression => "yield_expression",
            Javascript::Object => "object",
            Javascript::ObjectPattern => "object_pattern",
            Javascript::AssignmentPattern => "assignment_pattern",
            Javascript::ObjectAssignmentPattern => "object_assignment_pattern",
            Javascript::Array => "array",
            Javascript::ArrayPattern => "array_pattern",
            Javascript::JsxElement => "jsx_element",
            Javascript::JsxFragment => "jsx_fragment",
            Javascript::JsxExpression => "jsx_expression",
            Javascript::JsxOpeningElement => "jsx_opening_element",
            Javascript::NestedIdentifier => "nested_identifier",
            Javascript::JsxNamespaceName => "jsx_namespace_name",
            Javascript::JsxClosingElement => "jsx_closing_element",
            Javascript::JsxSelfClosingElement => "jsx_self_closing_element",
            Javascript::JsxAttribute => "jsx_attribute",
            Javascript::Class => "class",
            Javascript::ClassDeclaration => "class_declaration",
            Javascript::ClassHeritage => "class_heritage",
            Javascript::Function => "function",
            Javascript::FunctionDeclaration => "function_declaration",
            Javascript::GeneratorFunction => "generator_function",
            Javascript::GeneratorFunctionDeclaration => "generator_function_declaration",
            Javascript::ArrowFunction => "arrow_function",
            Javascript::CallExpression => "call_expression",
            Javascript::NewExpression => "new_expression",
            Javascript::AwaitExpression => "await_expression",
            Javascript::MemberExpression => "member_expression",
            Javascript::SubscriptExpression => "subscript_expression",
            Javascript::AssignmentExpression => "assignment_expression",
            Javascript::AugmentedAssignmentLhs => "_augmented_assignment_lhs",
            Javascript::AugmentedAssignmentExpression => "augmented_assignment_expression",
            Javascript::Initializer => "_initializer",
            Javascript::DestructuringPattern => "_destructuring_pattern",
            Javascript::SpreadElement => "spread_element",
            Javascript::TernaryExpression => "ternary_expression",
            Javascript::BinaryExpression => "binary_expression",
            Javascript::UnaryExpression => "unary_expression",
            Javascript::UpdateExpression => "update_expression",
            Javascript::SequenceExpression => "sequence_expression",
            Javascript::String => "string",
            Javascript::TemplateString => "template_string",
            Javascript::TemplateSubstitution => "template_substitution",
            Javascript::Regex => "regex",
            Javascript::MetaProperty => "meta_property",
            Javascript::Arguments => "arguments",
            Javascript::Decorator => "decorator",
            Javascript::MemberExpression2 => "member_expression",
            Javascript::CallExpression2 => "call_expression",
            Javascript::ClassBody => "class_body",
            Javascript::FieldDefinition => "field_definition",
            Javascript::FormalParameters => "formal_parameters",
            Javascript::Pattern => "pattern",
            Javascript::RestPattern => "rest_pattern",
            Javascript::MethodDefinition => "method_definition",
            Javascript::Pair => "pair",
            Javascript::PairPattern => "pair_pattern",
            Javascript::PropertyName => "_property_name",
            Javascript::ComputedPropertyName => "computed_property_name",
            Javascript::ProgramRepeat1 => "program_repeat1",
            Javascript::ExportStatementRepeat1 => "export_statement_repeat1",
            Javascript::ExportClauseRepeat1 => "export_clause_repeat1",
            Javascript::NamedImportsRepeat1 => "named_imports_repeat1",
            Javascript::VariableDeclarationRepeat1 => "variable_declaration_repeat1",
            Javascript::SwitchBodyRepeat1 => "switch_body_repeat1",
            Javascript::ObjectRepeat1 => "object_repeat1",
            Javascript::ObjectPatternRepeat1 => "object_pattern_repeat1",
            Javascript::ArrayRepeat1 => "array_repeat1",
            Javascript::ArrayPatternRepeat1 => "array_pattern_repeat1",
            Javascript::JsxElementRepeat1 => "jsx_element_repeat1",
            Javascript::JsxOpeningElementRepeat1 => "jsx_opening_element_repeat1",
            Javascript::StringRepeat1 => "string_repeat1",
            Javascript::StringRepeat2 => "string_repeat2",
            Javascript::TemplateStringRepeat1 => "template_string_repeat1",
            Javascript::ClassBodyRepeat1 => "class_body_repeat1",
            Javascript::FormalParametersRepeat1 => "formal_parameters_repeat1",
            Javascript::ImportSpecifier => "import_specifier",
            Javascript::NamespaceExport => "namespace_export",
            Javascript::PropertyIdentifier => "property_identifier",
            Javascript::ShorthandPropertyIdentifier => "shorthand_property_identifier",
            Javascript::ShorthandPropertyIdentifierPattern => {
                "shorthand_property_identifier_pattern"
            }
            Javascript::StatementIdentifier => "statement_identifier",
            Javascript::Error => "ERROR",
        }
    }
}

impl From<u16> for Javascript {
    #[inline(always)]
    fn from(x: u16) -> Self {
        num::FromPrimitive::from_u16(x).unwrap_or(Self::Error)
    }
}

// Javascript == u16
impl PartialEq<u16> for Javascript {
    #[inline(always)]
    fn eq(&self, x: &u16) -> bool {
        *self == Javascript::from(*x)
    }
}

// u16 == Javascript
impl PartialEq<Javascript> for u16 {
    #[inline(always)]
    fn eq(&self, x: &Javascript) -> bool {
        *x == *self
    }
}
