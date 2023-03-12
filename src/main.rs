use std::{
    env::args,
    error::Error,
    fmt::{Debug, Display},
    fs,
};

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
    STOP,
    ADD,
    MUL,
    SUB,
    DIV,
    SDIV,
    MOD,
    SMOD,
    ADDMOD,
    MULMOD,
    EXP,
    SIGNEXTEND,
    GT,
    SLT,
    SGT,
    EQ,
    ISZERO,
    AND,
    OR,
    XOR,
    NOT,
    BYTE,
    SHL,
    SHR,
    SAR,
    SHA3,
    ADDRESS,
    BALANCE,
    CALLVALUE,
    RETURN,
}

impl Op {
    pub fn from_str(str: &'static str) -> Result<(Self, &str), Box<dyn Error>> {
        match str {
            "MSTORE" | "mstore" => Ok((Self::MSTORE, "52")),
            "MLOAD" | "mload" => Ok((Self::MLOAD, "51")),
            "CREATE" | "create" => Ok((Self::CREATE, "f0")),
            "EXTCODECOPY" | "extcodecopy" => Ok((Self::EXTCODECOPY, "3c")),
            "PUSH1" | "push1" => Ok((Self::PUSH1, "60")),
            "POP" | "pop" => Ok((Self::POP, "50")),
            "DUP1" | "dup1" => Ok((Self::DUP1, "80")),
            "SWAP1" | "swap1" => Ok((Self::SWAP1, "90")),
            "STOP" | "stop" => Ok((Self::STOP, "00")),
            "ADD" | "add" => Ok((Self::ADD, "01")),
            "MUL" | "mul" => Ok((Self::MUL, "02")),
            "SUB" | "sub" => Ok((Self::SUB, "03")),
            "DIV" | "div" => Ok((Self::DIV, "04")),
            "SDIV" | "sdiv" => Ok((Self::SDIV, "05")),
            "MOD" | "mod" => Ok((Self::MOD, "06")),
            "SMOD" | "smod" => Ok((Self::SMOD, "07")),
            "ADDMOD" | "addmod" => Ok((Self::ADDMOD, "08")),
            "MULMOD" | "mulmod" => Ok((Self::MULMOD, "09")),
            "EXP" | "exp" => Ok((Self::EXP, "0a")),
            "SIGNEXTEND" | "signextend" => Ok((Self::SIGNEXTEND, "0b")),
            "GT" | "gt" => Ok((Self::GT, "11")),
            "SLT" | "slt" => Ok((Self::SLT, "12")),
            "SGT" | "sgt" => Ok((Self::SGT, "13")),
            "EQ" | "eq" => Ok((Self::EQ, "14")),
            "CALLVALUE" | "callvalue" => Ok((Self::CALLVALUE, "34")),
            "RETURN" | "return" => Ok((Self::RETURN, "f3")),

            _ => Ok((Self::VALUE(str.to_string()), "0")),
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
struct Lexer<'a> {
    /// Result
    result: Vec<(Op, &'a str)>,
}

impl<'a> Lexer<'a> {
    pub fn new() -> Self {
        Self { result: vec![] }
    }

    /// Start lexing
    fn lex(&mut self, source: &'static str) -> Result<(), Box<dyn Error>> {
        let mut splited = source.split_whitespace();

        while let Some(word) = splited.next() {
            match Op::from_str(word) {
                Ok(op) => self.result.push(op),

                Err(error) => panic!("{}", error),
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Parser<'a> {
    cursor: usize,
    tokens: &'a Vec<(Op, &'a str)>,
}

impl<'a> Iterator for Parser<'a> {
    type Item = (Op, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.tokens.get(self.cursor) {
            Some(token) => Some(token.clone()),
            None => None,
        };

        self.cursor += 1;

        result
    }
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<(Op, &'a str)>) -> Self {
        Self { cursor: 0, tokens }
    }

    fn parse(&mut self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut result: Vec<String> = vec![];

        while let Some(token) = self.next() {
            match token {
                (Op::VALUE(value), "0") => result.push(value.trim_start_matches("0x").to_string()),

                _ => {
                    result.push(token.1.to_string());
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

    let mut parser = Parser::new(&lexer.result);
    let result = parser.parse().unwrap();

    println!("{}", result.join("").to_string());
}
