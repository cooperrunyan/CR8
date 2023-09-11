use super::Compiler;

const UNDERSCORE: char = '_';
const ESCAPE: char = '\\';
const PERIOD: char = '.';
const COMMA: char = ',';
const COLON: char = ':';
const SEMI_COLON: char = ';';
const MUSTACHE_OPEN: char = '{';
const MUSTACHE_CLOSE: char = '}';
const BRACKET_OPEN: char = '[';
const BRACKET_CLOSE: char = ']';
const PAREN_OPEN: char = '(';
const PAREN_CLOSE: char = ')';
const DOUBLE_QUOTE: char = '"';
const EQUAL: char = '=';
const ANGLE_OPEN: char = '<';
const ANGLE_CLOSE: char = '>';
const ADD: char = '+';
const SUB: char = '-';
const AMPERSAND: char = '&';
const PIPE: char = '|';
const MUL: char = '*';
const DIV: char = '/';
const PERCENT: char = '%';
const DOLLAR: char = '$';
const SPACE: char = ' ';
const NEW_LINE: char = '\n';
const DIRECTIVE: char = '#';

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Token {
    Word(String),
    String(String),
    Number(i128),
    Escape(char),
    Comma,
    Colon,
    MustacheOpen,
    MustacheClose,
    BracketOpen,
    BracketClose,
    ParenOpen,
    ParenClose,
    Equal,
    LeftShift,
    RightShift,
    Add,
    Sub,
    Ampersand,
    Pipe,
    Mul,
    Div,
    Percent,
    Dollar,
    Space,
    NewLine,
    Directive,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Self::Word(v) => v.to_string(),
            Self::String(v) => v.to_string(),
            Self::Number(v) => v.to_string(),
            Self::Escape(v) => v.to_string(),
            Self::Comma => COMMA.to_string(),
            Self::Colon => COLON.to_string(),
            Self::MustacheOpen => MUSTACHE_OPEN.to_string(),
            Self::MustacheClose => MUSTACHE_CLOSE.to_string(),
            Self::BracketOpen => BRACKET_OPEN.to_string(),
            Self::BracketClose => BRACKET_CLOSE.to_string(),
            Self::ParenOpen => PAREN_OPEN.to_string(),
            Self::ParenClose => PAREN_CLOSE.to_string(),
            Self::Equal => EQUAL.to_string(),
            Self::LeftShift => "<<".to_string(),
            Self::RightShift => ">>".to_string(),
            Self::Add => ADD.to_string(),
            Self::Sub => SUB.to_string(),
            Self::Ampersand => AMPERSAND.to_string(),
            Self::Pipe => PIPE.to_string(),
            Self::Mul => MUL.to_string(),
            Self::Div => DIV.to_string(),
            Self::Percent => PERCENT.to_string(),
            Self::Dollar => DOLLAR.to_string(),
            Self::Space => SPACE.to_string(),
            Self::NewLine => NEW_LINE.to_string(),
            Self::Directive => DIRECTIVE.to_string(),
        }
    }
}

impl Compiler {
    pub(crate) fn tokenize<'s>(text: &'s str, file: &'s str) -> Vec<Token> {
        let mut chars = text.chars().peekable();
        let mut tokens = vec![];

        while let Some(ch) = chars.next() {
            let next = match ch {
                ANGLE_CLOSE => match chars.next() {
                    Some(ANGLE_CLOSE) => Token::RightShift,
                    x => panic!("Expected `>` after `>` got: {x:?}"),
                },
                ANGLE_OPEN => match chars.next() {
                    Some(ANGLE_OPEN) => Token::LeftShift,
                    x => panic!("Expected `<` after `<` got: {x:?}"),
                },
                ESCAPE => {
                    let Some(_) = chars.peek() else {
                        panic!("Expected a character after `\\`")
                    };
                    Token::Escape(chars.next().unwrap())
                }
                SPACE => {
                    while Some(&SPACE) == chars.peek() {
                        chars.next();
                    }
                    Token::Space
                }
                'A'..='Z' | 'a'..='z' | UNDERSCORE | PERIOD => {
                    let mut word = String::new();
                    word.push(ch);

                    while let Some('A'..='Z' | 'a'..='z' | '0'..='9' | &UNDERSCORE | &PERIOD) =
                        chars.peek()
                    {
                        word.push(chars.next().unwrap());
                    }
                    Token::Word(word)
                }
                '0'..='9' => {
                    let mut str = String::new();
                    str.push(ch);

                    while let Some('A'..='Z' | 'a'..='z' | '0'..='9' | &UNDERSCORE) = chars.peek() {
                        let next = chars.next().unwrap();
                        if next != UNDERSCORE {
                            str.push(next);
                        }
                    }

                    let num = if let Some(b) = str.strip_prefix("0b") {
                        if b.len() > 16 {
                            let mut split: Vec<String> = vec![];
                            let mut skip = 0;
                            for (i, ch) in b.chars().enumerate() {
                                if skip > 0 {
                                    skip -= 1;
                                    continue;
                                }
                                let mut byt = String::new();
                                byt.push(ch);
                                byt.push(b.chars().nth(i + 1).unwrap_or(' '));
                                byt.push(b.chars().nth(i + 2).unwrap_or(' '));
                                byt.push(b.chars().nth(i + 3).unwrap_or(' '));
                                byt.push(b.chars().nth(i + 4).unwrap_or(' '));
                                byt.push(b.chars().nth(i + 5).unwrap_or(' '));
                                byt.push(b.chars().nth(i + 6).unwrap_or(' '));
                                byt.push(b.chars().nth(i + 7).unwrap_or(' '));
                                split.push(byt.trim().to_string());
                                skip = 7;
                            }
                            let split = split
                                .into_iter()
                                .map(|s| i128::from_str_radix(&s, 2).unwrap())
                                .collect::<Vec<_>>();
                            let (nums, last) = split.split_at(split.len() - 1);
                            for num in nums {
                                tokens.push(Token::Number(*num));
                                tokens.push(Token::Comma);
                            }
                            Ok(*last.get(0).unwrap())
                        } else {
                            i128::from_str_radix(b, 2)
                        }
                    } else if let Some(h) = str.strip_prefix("0x") {
                        i128::from_str_radix(h, 16)
                    } else {
                        str.parse::<i128>()
                    };

                    match num {
                        Ok(n) => Token::Number(n),
                        Err(e) => {
                            panic!("Error at {file:?}\nInvalid number: {str:#?} \n\n{e}")
                        }
                    }
                }

                NEW_LINE => Token::NewLine,
                COMMA => Token::Comma,
                COLON => Token::Colon,
                SEMI_COLON => {
                    while chars.peek() != Some(&NEW_LINE) {
                        chars.next();
                    }
                    Token::Space
                }
                MUSTACHE_OPEN => Token::MustacheOpen,
                MUSTACHE_CLOSE => Token::MustacheClose,
                BRACKET_OPEN => Token::BracketOpen,
                BRACKET_CLOSE => Token::BracketClose,
                PAREN_OPEN => Token::ParenOpen,
                PAREN_CLOSE => Token::ParenClose,
                DOUBLE_QUOTE => {
                    let mut str = String::new();
                    while let Some(ch) = chars.next() {
                        if ch == DOUBLE_QUOTE {
                            break;
                        }
                        str.push(ch)
                    }
                    Token::String(str)
                }
                EQUAL => Token::Equal,
                ADD => Token::Add,
                SUB => Token::Sub,
                AMPERSAND => Token::Ampersand,
                PIPE => Token::Pipe,
                MUL => Token::Mul,
                DIV => Token::Div,
                PERCENT => Token::Percent,
                DOLLAR => Token::Dollar,
                DIRECTIVE => Token::Directive,
                other => panic!("Unknown token: {other:#?}"),
            };

            tokens.push(next);
        }

        tokens
    }
}
