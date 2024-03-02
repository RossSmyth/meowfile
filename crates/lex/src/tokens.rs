use diagnostics::Span;
use logos::Logos;

#[derive(Clone, Copy, Default, Debug)]
pub struct Token {
	pub kind: TokenKind,
	pub span: Span,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, Logos)]
pub enum TokenKind {
    #[regex(r"(\p{XID_Start}\p{XID_Continue}*)|(_\p{XID_Continue}+)", priority = 2)]
    Ident,
    #[token("rule")]
    Rule,
    #[token("[")]
    RBracket,
    #[token("]")]
    LBracket,
    #[token(",")]
    Comma,
    #[token("export")]
    Export,
    #[token("let")]
    Let,
    #[token("=")]
    Equal,
    #[token("&")]
    Execute,
    #[regex(r#""(\.|[^"\\])*""#)]
    StringLit,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("+")]
    Plus,
    #[regex(r"\#\![0-9a-zA-Z _\.\-:][^\n]")]
    Shebang,
    #[token("pub")]
    Pub,
    #[token("#")]
    Pound,
    #[token("!")]
    Exclamation,
    #[regex("[ \t\n\r]+")]
	Whitespace,
    #[regex("//[^\n]*")]
	Comment,
    EOF,
    #[default]
    Error
}