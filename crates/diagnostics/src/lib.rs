//! Diagnostic items.
//! To be expanded.

/// A span of source code.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Span {
	/// zero-indexed start
	pub start: u32,

	/// zero-indexed end
	pub end: u32,
}