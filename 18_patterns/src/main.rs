// let PATTERN = EXPRESSION;


// Patterns come in two forms: refutable and irrefutable. Patterns that will match for any possible value passed are irrefutable
// let statements, and for loops can only accept irrefutable patterns
mod syntax;

fn main() {
    syntax::match_guards();
}
