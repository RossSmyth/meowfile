//! Lexer for meowfiles.

use std::ops::Range;

use diagnostics::Span;
use token::{Token, TokenKind};

/// The tokens the lexer produces
pub mod token;

/// Lexer object for each token.
#[derive(Debug)]
pub struct Lexer<'s> {
	inner: logos::Lexer<'s, TokenKind>,

	/// Debug field to verify all tokens are accounted for.
	#[cfg(debug_assertions)]
	last_end: u32,
}

impl<'s> Lexer<'s> {
    /// Constructs a new lexer for a source file
	pub fn new(source: &'s str) -> Self {
		Self {
			inner: logos::Lexer::new(source),
			#[cfg(debug_assertions)]
			last_end: 0,
		}
	}

    /// Get the source the lexer is tokenizing
	pub fn source(&self) -> &'s str { self.inner.source() }
}

impl Lexer<'_> {

    /// Calculate the next token in the source 
	pub fn next(&mut self) -> Token {
		let token = self.inner.next();
		let Range { start, end } = self.inner.span();

		let start = start.try_into().expect("Input too long to properly tokenize (exceeds a u32)");
		let end = end.try_into().expect("Input too long to properly tokenize (exceeds a u32)");

		assert_eq!(self.last_end, start as _, "Tokens not contiguous for token {:?} at start location {}", token, start);
		self.last_end = end as _;

		match token {
			Some(token) => Token {
				kind: token.unwrap_or(T!(err)),
				span: Span {
					start,
					end
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
	
		while !matches!(next.kind, T![eof]) {
			assert_ne!(next.kind, T![err]);
			next = lexer.next();
		}
	}
}
