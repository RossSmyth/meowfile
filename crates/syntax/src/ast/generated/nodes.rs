//! Generated by `sourcegen_ast`, do not edit by hand.

#![allow(non_snake_case)]
use crate::{
    ast::{self, support, AstChildren, AstNode},
    SyntaxKind::{self, *},
    SyntaxNode, SyntaxToken, T,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name {
    pub(crate) syntax: SyntaxNode,
}
impl Name {
    pub fn ident_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![ident]) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rule {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasAttrs for Rule {}
impl ast::HasName for Rule {}
impl ast::HasVisibility for Rule {}
impl Rule {
    pub fn rule_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![rule]) }
    pub fn body(&self) -> Option<BlockExpr> { support::child(&self.syntax) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LetExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasName for LetExpr {}
impl LetExpr {
    pub fn export_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![export]) }
    pub fn let_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![let]) }
    pub fn eq_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![=]) }
    pub fn init(&self) -> Option<Expr> { support::child(&self.syntax) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attr {
    pub(crate) syntax: SyntaxNode,
}
impl Attr {
    pub fn pound_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![#]) }
    pub fn excl_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![!]) }
    pub fn l_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['[']) }
    pub fn meta(&self) -> Option<Meta> { support::child(&self.syntax) }
    pub fn r_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![']']) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Visibility {
    pub(crate) syntax: SyntaxNode,
}
impl Visibility {
    pub fn pub_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![pub]) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasAttrs for BlockExpr {}
impl BlockExpr {
    pub fn stmt_list(&self) -> Option<StmtList> { support::child(&self.syntax) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Stmt {
    pub(crate) syntax: SyntaxNode,
}
impl Stmt {
    pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
    pub fn newline_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![newline]) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ArrayExpr {
    pub fn l_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['[']) }
    pub fn array_list(&self) -> Option<ArrayList> { support::child(&self.syntax) }
    pub fn r_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![']']) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Literal {
    pub(crate) syntax: SyntaxNode,
}
impl Literal {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShellExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ShellExpr {
    pub fn amp_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![&]) }
    pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayList {
    pub(crate) syntax: SyntaxNode,
}
impl ArrayList {
    pub fn exprs(&self) -> AstChildren<Expr> { support::children(&self.syntax) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StmtList {
    pub(crate) syntax: SyntaxNode,
}
impl StmtList {
    pub fn l_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['{']) }
    pub fn statements(&self) -> AstChildren<Stmt> { support::children(&self.syntax) }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['}']) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinExpr {
    pub(crate) syntax: SyntaxNode,
}
impl BinExpr {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceFile {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasAttrs for SourceFile {}
impl ast::HasDocComments for SourceFile {}
impl SourceFile {
    pub fn shebang_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![shebang]) }
    pub fn item_kinds(&self) -> AstChildren<ItemKind> { support::children(&self.syntax) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Meta {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasName for Meta {}
impl Meta {
    pub fn eq_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![=]) }
    pub fn literal(&self) -> Option<Literal> { support::child(&self.syntax) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemKind {
    Rule(Rule),
    LetExpr(LetExpr),
}
impl ast::HasName for ItemKind {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    ArrayExpr(ArrayExpr),
    BlockExpr(BlockExpr),
    Literal(Literal),
    ShellExpr(ShellExpr),
    LetExpr(LetExpr),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnyHasAttrs {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasAttrs for AnyHasAttrs {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnyHasDocComments {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasDocComments for AnyHasDocComments {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnyHasName {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasName for AnyHasName {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnyHasVisibility {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasVisibility for AnyHasVisibility {}
impl AstNode for Name {
    fn can_cast(kind: SyntaxKind) -> bool { kind == NAME }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Rule {
    fn can_cast(kind: SyntaxKind) -> bool { kind == RULE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for LetExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == LET_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Attr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == ATTR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Visibility {
    fn can_cast(kind: SyntaxKind) -> bool { kind == VISIBILITY }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for BlockExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == BLOCK_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Stmt {
    fn can_cast(kind: SyntaxKind) -> bool { kind == STMT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ArrayExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == ARRAY_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Literal {
    fn can_cast(kind: SyntaxKind) -> bool { kind == LITERAL }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ShellExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SHELL_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ArrayList {
    fn can_cast(kind: SyntaxKind) -> bool { kind == ARRAY_LIST }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for StmtList {
    fn can_cast(kind: SyntaxKind) -> bool { kind == STMT_LIST }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for BinExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == BIN_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for SourceFile {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SOURCE_FILE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Meta {
    fn can_cast(kind: SyntaxKind) -> bool { kind == META }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl From<Rule> for ItemKind {
    fn from(node: Rule) -> ItemKind { ItemKind::Rule(node) }
}
impl From<LetExpr> for ItemKind {
    fn from(node: LetExpr) -> ItemKind { ItemKind::LetExpr(node) }
}
impl AstNode for ItemKind {
    fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, RULE | LET_EXPR) }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            RULE => ItemKind::Rule(Rule { syntax }),
            LET_EXPR => ItemKind::LetExpr(LetExpr { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            ItemKind::Rule(it) => &it.syntax,
            ItemKind::LetExpr(it) => &it.syntax,
        }
    }
}
impl From<ArrayExpr> for Expr {
    fn from(node: ArrayExpr) -> Expr { Expr::ArrayExpr(node) }
}
impl From<BlockExpr> for Expr {
    fn from(node: BlockExpr) -> Expr { Expr::BlockExpr(node) }
}
impl From<Literal> for Expr {
    fn from(node: Literal) -> Expr { Expr::Literal(node) }
}
impl From<ShellExpr> for Expr {
    fn from(node: ShellExpr) -> Expr { Expr::ShellExpr(node) }
}
impl From<LetExpr> for Expr {
    fn from(node: LetExpr) -> Expr { Expr::LetExpr(node) }
}
impl AstNode for Expr {
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            ARRAY_EXPR | BLOCK_EXPR | LITERAL | SHELL_EXPR | LET_EXPR
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            ARRAY_EXPR => Expr::ArrayExpr(ArrayExpr { syntax }),
            BLOCK_EXPR => Expr::BlockExpr(BlockExpr { syntax }),
            LITERAL => Expr::Literal(Literal { syntax }),
            SHELL_EXPR => Expr::ShellExpr(ShellExpr { syntax }),
            LET_EXPR => Expr::LetExpr(LetExpr { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Expr::ArrayExpr(it) => &it.syntax,
            Expr::BlockExpr(it) => &it.syntax,
            Expr::Literal(it) => &it.syntax,
            Expr::ShellExpr(it) => &it.syntax,
            Expr::LetExpr(it) => &it.syntax,
        }
    }
}
impl AnyHasAttrs {
    #[inline]
    pub fn new<T: ast::HasAttrs>(node: T) -> AnyHasAttrs {
        AnyHasAttrs {
            syntax: node.syntax().clone(),
        }
    }
}
impl AstNode for AnyHasAttrs {
    fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, RULE | BLOCK_EXPR | SOURCE_FILE) }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then_some(AnyHasAttrs { syntax })
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AnyHasDocComments {
    #[inline]
    pub fn new<T: ast::HasDocComments>(node: T) -> AnyHasDocComments {
        AnyHasDocComments {
            syntax: node.syntax().clone(),
        }
    }
}
impl AstNode for AnyHasDocComments {
    fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, SOURCE_FILE) }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then_some(AnyHasDocComments { syntax })
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AnyHasName {
    #[inline]
    pub fn new<T: ast::HasName>(node: T) -> AnyHasName {
        AnyHasName {
            syntax: node.syntax().clone(),
        }
    }
}
impl AstNode for AnyHasName {
    fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, RULE | LET_EXPR | META) }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then_some(AnyHasName { syntax })
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AnyHasVisibility {
    #[inline]
    pub fn new<T: ast::HasVisibility>(node: T) -> AnyHasVisibility {
        AnyHasVisibility {
            syntax: node.syntax().clone(),
        }
    }
}
impl AstNode for AnyHasVisibility {
    fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, RULE) }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then_some(AnyHasVisibility { syntax })
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Display for ItemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for LetExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Attr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for BlockExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ArrayExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ShellExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ArrayList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for StmtList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for BinExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SourceFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Meta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
