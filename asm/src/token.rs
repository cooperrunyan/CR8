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
const ANGLE_CLOSE: char = '?';
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

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Word(String),
    String(String),
    Number(i128),
    Escape(char),
    Period,
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

pub fn tokenize<'s>(text: &'s str) -> Vec<Token> {
    let mut chars = text.chars().peekable();
    let mut tokens = vec![];

    while let Some(ch) = chars.next() {
        tokens.push(match ch {
            ANGLE_CLOSE => match chars.next() {
                Some(ANGLE_CLOSE) => Token::RightShift,
                x => panic!("Expected `>` after `>` got: {x:?}"),
            },
            ANGLE_OPEN => match chars.next() {
                Some(ANGLE_OPEN) => Token::LeftShift,
                x => panic!("Expected `<` after `<` got: {x:?}"),
            },
            ESCAPE => {
                let Some(ch) = chars.peek() else {
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
            'A'..='Z' | 'a'..='z' | UNDERSCORE => {
                let mut word = String::new();
                word.push(ch);

                while let Some('A'..='Z' | 'a'..='z' | '0'..='9' | &UNDERSCORE) = chars.peek() {
                    word.push(chars.next().unwrap());
                }
                Token::Word(word)
            }
            '0'..='9' => {
                let mut str = String::new();
                str.push(ch);

                while let Some('A'..='F' | 'a'..='f' | '0'..='9' | 'x' | &UNDERSCORE) = chars.peek()
                {
                    let next = chars.next().unwrap();
                    if next != UNDERSCORE {
                        str.push(next);
                    }
                }

                let num = if let Some(b) = str.strip_prefix("0b") {
                    i128::from_str_radix(b, 2)
                } else if let Some(h) = str.strip_prefix("0x") {
                    i128::from_str_radix(h, 16)
                } else {
                    str.parse::<i128>()
                };

                match num {
                    Ok(n) => Token::Number(n),
                    Err(e) => panic!("Invalid number: {str}. \n\n{e}"),
                }
            }

            NEW_LINE => Token::NewLine,
            PERIOD => Token::Period,
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
        })
    }

    tokens
}
