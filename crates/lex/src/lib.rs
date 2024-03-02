//! Lexer for meowfiles.

use diagnostics::Span;
use tokens::{Token, TokenKind};

mod tokens;

/// Lexer object for each token.
#[derive(Debug)]
pub struct Lexer<'s> {
	inner: logos::Lexer<'s, TokenKind>,
}

impl<'s> Lexer<'s> {
    /// Constructs a new lexer for a source file
	pub fn new(source: &'s str) -> Self {
		Self {
			inner: logos::Lexer::new(source),
		}
	}

    /// Get the source the lexer is tokenizing
	pub fn source(&self) -> &'s str { self.inner.source() }
}

impl Lexer<'_> {

    /// Calculate the next token in the source 
	pub fn next(&mut self) -> Token {
		let token = self.inner.next();
		let span = self.inner.span();
		match token {
			Some(token) => Token {
				kind: token.unwrap_or(TokenKind::Error),
				span: Span {
					start: span.start as _,
					end: span.end as _,
				},
			},
			None => Token {
				kind: TokenKind::EOF,
				span: self.eof_span(),
			},
		}
	}

    /// Get the eof
	pub fn eof_span(&self) -> Span {
		Span {
			start: self.source().len() as u32 - 1,
			end: self.source().len() as _,
		}
	}
}

#[cfg(test)]
mod test {
    use super::*;

    const FILE: &str = include_str!("../../../meowfile");

    #[test]
    fn lex() {
        let mut lex = Lexer::new(FILE);

        let mut tok = lex.next();
        
        while !matches!(tok.kind, TokenKind::EOF) {
            assert_ne!(tok.kind, TokenKind::Error);
            tok = lex.next();
        }
    }
}
