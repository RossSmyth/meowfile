/// A span of source code.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Span {
	pub start: u32,
	pub end: u32,
}