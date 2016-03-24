mod regex;
mod tonfa;
mod farule;
mod nfadesign;
mod nfarulebook;
mod nfa;
mod helper;

use regex::{Regex};
use tonfa::{ToNFA};

pub fn main() {
    let mut pattern = Regex::repeat(Regex::choose(Regex::concatenate(Regex::literal('a'), Regex::literal('b')), Regex::literal('a')));
    println!("{}", pattern);

    let mut nfadesign = Regex::empty().to_nfa_design();
    println!("{}", nfadesign.accept(""));
    println!("{}", nfadesign.accept("a"));
    nfadesign = Regex::literal('a').to_nfa_design();
    println!("{}", nfadesign.accept(""));
    println!("{}", nfadesign.accept("a"));
    println!("{}", nfadesign.accept("b"));
    println!("{}", Regex::empty().matches("a"));
    println!("{}", Regex::literal('a').matches("a"));

    println!("Concatenate");
    pattern = Regex::concatenate(Regex::literal('a'), Regex::literal('b'));
    println!("'{}' accept? a: {}, ab: {}, abc: {}", pattern, pattern.matches("a"), pattern.matches("ab"), pattern.matches("abc"));

    pattern = Regex::choose(Regex::literal('a'), Regex::literal('b'));
    println!("'{}' accept? a: {}, b: {}, c: {}", pattern, pattern.matches("a"), pattern.matches("b"), pattern.matches("c"));
}
