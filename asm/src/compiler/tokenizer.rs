use std::iter::{Enumerate, Peekable};
use std::path::PathBuf;
use std::str::Chars;
use std::sync::Arc;

use super::Compiler;
use anyhow::{bail, Context, Result};

const UNDERSCORE: char = '_';
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

#[derive(Debug)]
pub(crate) struct TokenMeta {
    pub(crate) token: Token,
    pub(crate) path: Arc<PathBuf>,
    pub(crate) line: usize,
    pub(crate) col: usize,
}
macro_rules! isable {
    ($v:vis enum $n:ident {
        $($member:ident($is:ident $(, $inner:ty, $x:pat)?),)*
    }) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        $v enum $n {
            $($member $( ($inner) )? ,)*
        }

        #[allow(dead_code)]
        impl $n {
            $(pub fn $is(&self) -> bool {
                match &self {
                    $n::$member$(($x))? => true,
                    _ => false
                }
            })*
        }
    }
}

isable! {
    pub enum Token {
        Word(is_word, String, _),
        String(is_str, String, _),
        Number(is_num, i128, _),
        Comma(is_comma),
        Colon(is_colon),
        MustacheOpen(is_mustache_open),
        MustacheClose(is_mustache_close),
        BracketOpen(is_brack_open),
        BracketClose(is_bracket_close),
        ParenOpen(is_paren_open),
        ParenClose(is_paren_close),
        Equal(is_equal),
        LeftShift(is_left_shift),
        RightShift(is_right_shift),
        Add(is_add),
        Sub(is_sub),
        Ampersand(is_ampersand),
        Pipe(is_pipe),
        Mul(is_mul),
        Div(is_div),
        Percent(is_percent),
        Dollar(is_dollar),
        Space(is_space),
        NewLine(is_newline),
        Directive(is_directive),
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Self::Word(v) => v.to_string(),
            Self::String(v) => v.to_string(),
            Self::Number(v) => v.to_string(),
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
    pub(crate) fn tokenize(file: String, path: Arc<PathBuf>) -> Result<Vec<TokenMeta>> {
        let mut chars = file.chars().enumerate().peekable();
        let mut lines = 0;
        let mut last_line = 0;
        let mut tokens = vec![];

        while let Some((i, ch)) = chars.next() {
            let line = lines;
            let col = i - last_line;
            let token = tokenize_next(&mut chars, i, ch, &mut lines, &mut last_line)
                .context(format!("Error at {path:?}:{}:{}", line + 1, col + 1))?;

            tokens.push(TokenMeta {
                token,
                path: path.clone(),
                line,
                col,
            });
        }

        Ok(tokens)
    }
}

fn tokenize_next(
    chars: &mut Peekable<Enumerate<Chars>>,
    i: usize,
    ch: char,
    lines: &mut usize,
    last_line: &mut usize,
) -> Result<Token> {
    let next = match ch {
        ANGLE_CLOSE => match chars.next() {
            Some((_, ANGLE_CLOSE)) => Token::RightShift,
            x => bail!("Expected `>` after `>` got: {x:?}"),
        },
        ANGLE_OPEN => match chars.next() {
            Some((_, ANGLE_OPEN)) => Token::LeftShift,
            x => bail!("Expected `<` after `<` got: {x:?}"),
        },
        SPACE => {
            while Some(SPACE) == chars.peek().map(|t| t.1) {
                chars.next();
            }
            Token::Space
        }
        'A'..='Z' | 'a'..='z' | UNDERSCORE | PERIOD => {
            let mut s = String::new();
            s.push(ch);

            while let Some('A'..='Z' | 'a'..='z' | '0'..='9' | UNDERSCORE | PERIOD) =
                chars.peek().map(|t| t.1)
            {
                s.push(chars.next().unwrap().1);
            }
            Token::Word(s)
        }
        '0'..='9' => {
            let mut str = String::new();
            str.push(ch);

            while let Some('A'..='Z' | 'a'..='z' | '0'..='9' | UNDERSCORE) =
                chars.peek().map(|t| t.1)
            {
                let next = chars.next().map(|t| t.1).unwrap();
                if next != UNDERSCORE {
                    str.push(next);
                }
            }

            let num = {
                if let Some(b) = str.strip_prefix("0b") {
                    i128::from_str_radix(b, 2)
                } else if let Some(h) = str.strip_prefix("0x") {
                    i128::from_str_radix(h, 16)
                } else {
                    str.parse::<i128>()
                }
            };

            match num {
                Ok(n) => Token::Number(n),
                Err(_) => bail!("Invalid number: {str:#?}"),
            }
        }

        NEW_LINE => {
            *last_line = i;
            *lines += 1;
            Token::NewLine
        }
        COMMA => Token::Comma,
        COLON => Token::Colon,
        SEMI_COLON => {
            while chars.peek().map(|t| t.1) != Some(NEW_LINE) {
                chars.next();
            }
            Token::Space
        }
        DOUBLE_QUOTE => {
            let mut s = String::new();
            while let Some(x) = chars.peek().map(|t| t.1) {
                if x == DOUBLE_QUOTE {
                    chars.next();
                    break;
                }
                s.push(x);
                chars.next();
            }
            Token::String(s)
        }
        MUSTACHE_OPEN => Token::MustacheOpen,
        MUSTACHE_CLOSE => Token::MustacheClose,
        BRACKET_OPEN => Token::BracketOpen,
        BRACKET_CLOSE => Token::BracketClose,
        PAREN_OPEN => Token::ParenOpen,
        PAREN_CLOSE => Token::ParenClose,
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
        oth => bail!(format!("Unexpected: {oth:#?}")),
    };

    Ok(next)
}
