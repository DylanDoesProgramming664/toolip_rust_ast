mod lexer;
mod parser;

fn sum(a: usize, b: usize) -> usize {
    return a + b;
}

fn main() {
    let (one, two): (usize, usize) = (1, 2);

    let _result = sum(one, two);
}
