mod token;
mod lexer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];

    let src = std::fs::read(path).expect("Failed to read file");

}
