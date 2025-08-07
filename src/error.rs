#[derive(Debug)]
pub enum Error {
    // Lexing errors
    InvalidNumFormat,
    UnexpectedChar(char),
    UnsupportedOperator(&'static str),
    UnterminatedStr,

    // Parsing errors
}
