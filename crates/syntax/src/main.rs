use ungrammar::{Grammar, Node, Rule};

fn main() {
    let grammar: Grammar = include_str!("../meow.ungram").parse().unwrap();

    for tok in grammar.tokens() {
        dbg!(
        &grammar[tok]
        );
    }
}
