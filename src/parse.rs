#[derive(Debug)]
pub enum Token {
    Right(usize),
    Left(usize),
    Increment(usize),
    Decrement(usize),
    Output,
    Input,
    LoopStart,
    LoopEnd,
}

impl Token {
    fn from_char(c: char, last_seen_count: usize) -> Option<Token> {
        match c {
            '>' => Some(Token::Right(last_seen_count)),
            '<' => Some(Token::Left(last_seen_count)),
            '+' => Some(Token::Increment(last_seen_count)),
            '-' => Some(Token::Decrement(last_seen_count)),
            '.' => Some(Token::Output),
            ',' => Some(Token::Input),
            '[' => Some(Token::LoopStart),
            ']' => Some(Token::LoopEnd),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub tokens: Vec<Token>,
}

impl From<&str> for Program {
    fn from(value: &str) -> Self {
        let mut tokens = Vec::new();

        let mut last_seen: Option<char> = None;
        let mut last_seen_count = 0;
        for current in value.chars() {
            if let Some(last) = last_seen {
                if current == last {
                    last_seen_count += 1;
                } else {
                    if let Some(tok) = Token::from_char(last, last_seen_count) {
                        tokens.push(tok);
                    }
                    last_seen = Some(current);
                    last_seen_count = 1;
                }
            } else {
                last_seen = Some(current);
                last_seen_count = 1;
            }
        }
        if let Some(last) = last_seen {
            if let Some(tok) = Token::from_char(last, last_seen_count) {
                tokens.push(tok);
            }
        }

        Program { tokens }
    }
}
