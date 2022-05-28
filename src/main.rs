use std::{
    env::args,
    error::Error,
    fmt::{Debug, Display},
    fs,
    str::FromStr,
};

pub fn opcode_parser<Code, Parse>(opcode: Code, value: Code) -> Parse
where
    Code: Display,
    Parse: FromStr,
    <Parse as FromStr>::Err: Debug,
{
    format!("{}{}", opcode, value).parse::<Parse>().unwrap()
}

#[derive(Debug, Clone)]
pub enum Op {
    MSTORE,
    MLOAD,
    CREATE,
    EXTCODECOPY,
    PUSH1,
    POP,
    DUP1,
    SWAP1,
    VALUE(String),
}

impl Op {
    pub fn from_str(str: &'static str) -> Result<Self, Box<dyn Error>> {
        match str {
            "MSTORE" | "mstore" => Ok(Self::MSTORE),
            "MLOAD" | "mload" => Ok(Self::MLOAD),
            "CREATE" | "create" => Ok(Self::CREATE),
            "EXTCODECOPY" | "extcodecopy" => Ok(Self::EXTCODECOPY),
            "PUSH1" | "push1" => Ok(Self::PUSH1),
            "POP" | "pop" => Ok(Self::POP),
            "DUP1" | "dup1" => Ok(Self::DUP1),
            "SWAP1" | "swap1" => Ok(Self::SWAP1),

            _ => Ok(Self::VALUE(str.to_string())),
        }
    }

    fn as_opcode(&self) -> u32 {
        match self {
            Self::MSTORE => 52,
            Self::MLOAD => 51,
            Self::CREATE => 0xF0,
            Self::EXTCODECOPY => 0x3C,
            Self::PUSH1 => 60,
            Self::POP => 50,
            Self::DUP1 => 80,
            Self::SWAP1 => 90,

            _ => 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct LexError {
    line: Option<usize>,
    description: &'static str,
}

impl LexError {
    pub fn new(description: &'static str) -> Self {
        Self {
            line: None,
            description,
        }
    }

    /// Set line that Error happend
    fn line(&mut self, l: usize) -> Self {
        self.line = Some(l);

        *self
    }
}

impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.line {
            Some(line) => write!(
                f,
                "Error when lexing on line {} with description {}",
                line, self.description
            ),

            None => write!(f, "Error when lexing with description {}", self.description),
        }
    }
}

impl Error for LexError {
    fn description(&self) -> &str {
        self.description
    }
}

#[derive(Debug, Clone)]
struct Lexer {
    /// Result
    result: Vec<Op>,
}

impl Lexer {
    pub fn new() -> Self {
        Self { result: vec![] }
    }

    /// Start lexing
    fn lex(&mut self, source: &'static str) -> Result<(), Box<dyn Error>> {
        let mut lines = source.lines();

        while let Some(line) = lines.next() {
            // split by word
            let mut splited = line.split_whitespace();

            while let Some(word) = splited.next() {
                match Op::from_str(word) {
                    Ok(op) => self.result.push(op),

                    Err(error) => panic!("{}", error),
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Parser {
    cursor: usize,
    tokens: Vec<Op>,
}

impl Iterator for Parser {
    type Item = Op;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.tokens.get(self.cursor) {
            Some(token) => Some(token.clone()),
            None => None,
        };

        self.cursor += 1;

        result
    }
}

impl Parser {
    pub fn new(tokens: Vec<Op>) -> Self {
        Self { cursor: 0, tokens }
    }

    fn get_token(&self, index: usize) -> Option<&Op> {
        self.tokens.get(index)
    }

    fn parse(&mut self) -> Result<Vec<u32>, Box<dyn Error>> {
        let mut result: Vec<u32> = vec![];

        while let Some(token) = self.next() {
            match token {
                Op::PUSH1 => {
                    let next_token = match self.get_token(self.cursor) {
                        Some(val) => val,
                        None => panic!("PANICED"),
                    };

                    let value = match next_token {
                        Op::VALUE(val) => val.trim_start_matches("0x"),

                        _ => panic!("PANIC {:?}", next_token),
                    };

                    let code = opcode_parser(Op::PUSH1.as_opcode().to_string(), value.to_string());

                    result.push(code);

                    self.next();
                }

                _ => {
                    result.push(token.as_opcode());
                    self.next();
                }
            }
        }

        Ok(result)
    }
}

fn main() {
    let file_addr = args().nth(1).unwrap();
    let source = Box::new(fs::read_to_string(file_addr).unwrap());

    let mut lexer = Lexer::new();
    lexer.lex(Box::leak(source)).unwrap();

    println!("{:?}", lexer);

    let mut parser = Parser::new(lexer.result);
    let result = parser.parse().unwrap();

    let joined: Vec<String> = result.iter().map(|n| n.to_string()).collect();

    println!("{}", joined.join("").to_string());
}
