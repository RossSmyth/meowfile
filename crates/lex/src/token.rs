use diagnostics::Span;
use logos::Logos;

/// Token & span
#[derive(Clone, Copy, Default, Debug)]
pub struct Token {
	/// The token itself
	pub kind: TokenKind,

	/// The location in the file.
	pub span: Span,
}

#[allow(missing_docs)]
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
    Ampersand,
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
	// Everything in 
	// https://github.com/rust-lang/regex/blob/10fe722a3fcfdc17068b21f3262189cc52227bb5/regex-syntax/src/unicode_tables/property_bool.rs#L9909-L9920
	// except for newlines linefeed
    #[regex(r"[\p{White_Space}&&[^\n]]+")]
	Whitespace,
	#[token("\n")]
	Newline,
    #[regex("//[^\n]*")]
	Comment,
    EOF,
    #[default]
    Error
}

/// Get a [`TokenKind`] by something like the token iteslf. 
#[macro_export]
macro_rules! T {
	(string) => {
		$crate::token::TokenKind::StringLit
	};
    (rule) => {
        $crate::token::TokenKind::Rule
    };
	(ident) => {
		$crate::token::TokenKind::Ident
	};
    (#!) => {
        $crate::token::TokenKind::Shebang
    };
	('(') => {
		$crate::token::TokenKind::LParen
	};
	(')') => {
		$crate::token::TokenKind::RParen
	};
	('{') => {
		$crate::token::TokenKind::LBrace
	};
	('}') => {
		$crate::token::TokenKind::RBrace
	};
	('[') => {
		$crate::token::TokenKind::LBracket
	};
	(']') => {
		$crate::token::TokenKind::RBracket
	};
    (#) => {
        $crate::token::TokenKind::Pound
    };
	(,) => {
		$crate::token::TokenKind::Comma
	};
	(=) => {
		$crate::token::TokenKind::Eq
	};
	(+) => {
		$crate::token::TokenKind::Plus
	};
	(-) => {
		$crate::token::TokenKind::Minus
	};
	(&) => {
		$crate::token::TokenKind::Execute
	};
	(err) => {
		$crate::token::TokenKind::Error
	};
	(ws) => {
		$crate::token::TokenKind::Whitespace
	};
	(comment) => {
		$crate::token::TokenKind::Comment
	};
	(let) => {
		$crate::token::TokenKind::LetKw
	};
	(pub) => {
		$crate::token::TokenKind::PubKw
	};
	(eof) => {
		$crate::token::TokenKind::EOF
	};
}