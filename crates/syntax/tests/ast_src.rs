//! Defines input for code generation process.

pub(crate) struct KindsSrc<'a> {
    pub(crate) punct: &'a [(&'a str, &'a str)],
    pub(crate) keywords: &'a [&'a str],
    pub(crate) contextual_keywords: &'a [&'a str],
    pub(crate) literals: &'a [&'a str],
    pub(crate) tokens: &'a [&'a str],
    pub(crate) nodes: &'a [&'a str],
}

pub(crate) const KINDS_SRC: KindsSrc<'_> = KindsSrc {
    punct: &[
        (",", "COMMA"),
        ("(", "L_PAREN"),
        (")", "R_PAREN"),
        ("{", "L_CURLY"),
        ("}", "R_CURLY"),
        ("[", "L_BRACK"),
        ("]", "R_BRACK"),
        ("#", "POUND"),
        ("&", "AMP"),
        ("+", "PLUS"),
        ("=", "EQ"),
        ("!", "EXCLA"),
    ],
    keywords: &[
        "let",
        "export",
        "pub",
        "cfg",
        "rule",
    ],
    contextual_keywords: &[
    ],
    // These are already upper snake case.
    literals: &[
        "STRING",
    ],
    tokens: &[
        "ERROR",
        "IDENT",
        "WHITESPACE",
        "COMMENT",
        "SHEBANG",
    ],
    nodes: &[
        "SOURCE_FILE",
        "RULE",
        // atoms
        "ARRAY_EXPR",
        "BLOCK_EXPR",
        "STMT_LIST",
        "LET_EXPR",
        // unary
        "SHELL_EXPR",
        "ATTR",
        "LITERAL",
        "VISIBILITY",
        "NAME",
        "LET_ELSE",
    ],
};

#[derive(Default, Debug)]
pub(crate) struct AstSrc {
    pub(crate) tokens: Vec<String>,
    pub(crate) nodes: Vec<AstNodeSrc>,
    pub(crate) enums: Vec<AstEnumSrc>,
}

#[derive(Debug)]
pub(crate) struct AstNodeSrc {
    pub(crate) doc: Vec<String>,
    pub(crate) name: String,
    pub(crate) traits: Vec<String>,
    pub(crate) fields: Vec<Field>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Field {
    Token(String),
    Node { name: String, ty: String, cardinality: Cardinality },
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Cardinality {
    Optional,
    Many,
}

#[derive(Debug)]
pub(crate) struct AstEnumSrc {
    pub(crate) doc: Vec<String>,
    pub(crate) name: String,
    pub(crate) traits: Vec<String>,
    pub(crate) variants: Vec<String>,
}