//! Lexer for meowfiles.

use diagnostics::Span;
use token::{Token, TokenKind};

/// The tokens the lexer produces
pub mod token;

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
				kind: token.unwrap_or(T!(err)),
				span: Span {
					start: span.start as _,
					end: span.end as _,
				},
			},
			None => Token {
				kind: T!(eof),
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
    fn verify_span_is_continuous() {
		let mut lexer = Lexer::new(FILE);
		
		let mut next = lexer.next();
		let mut last_end = 0;
	
		while !matches!(next.kind, T![eof]) {
			let span = next.span;
			assert_eq!(last_end, span.start);
			last_end = span.end;
			next = lexer.next();
		}
	}
}
