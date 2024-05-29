use std::io::{self, BufRead};

#[derive(Debug)]
enum TokenType {
    // Single char tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,
    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier,
    String,
    Number,
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

#[derive(Debug)]
enum Literal {
    LitInt(i32),
    LitStr(String),
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: i32,
}

struct Scanner {
    x: String,
}

impl Scanner {
    fn scan_tokens() -> Vec<TokenType> {
        Vec::new()
    }
}

fn add_token(token_type: TokenType) {}

fn scan_token(ch: char) -> () {
    use TokenType as TT;
    let next: char = 'x';
    let token: TokenType;
    token = match ch {
        '(' => TT::LeftParen,
        ')' => TT::RightParen,
        '{' => TT::LeftBrace,
        '}' => TT::RightBrace,
        ',' => TT::Comma,
        '.' => TT::Dot,
        '-' => TT::Minus,
        '+' => TT::Plus,
        ';' => TT::SemiColon,
        '*' => TT::Star,
        '!' => {
            if next == '=' {
                TT::BangEqual
            } else {
                TT::Bang
            }
        }
        '=' => {
            if next == '=' {
                TT::EqualEqual
            } else {
                TT::Equal
            }
        }
        '<' => {
            if next == '=' {
                TT::LessEqual
            } else {
                TokenType::Less
            }
        }
        '>' => {
            if next == '=' {
                TT::GreaterEqual
            } else {
                TokenType::Greater
            }
        }
        _ => unimplemented!(),
    };
    add_token(token);
}

fn run(source: String) {
    let scanner = Scanner { x: source };
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{}", token)
    }
}

fn run_prompt() {
    println!(" ");
    print!("> ");
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    run(line);
}

fn main() -> io::Result<()> {
    let arg = std::env::args().nth(1);

    if let Some(arg) = arg {
        run(arg)
    } else {
        run_prompt()
    }

    Ok(())
}
