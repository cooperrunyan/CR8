#[macro_export]
macro_rules! next {
    ($tkns:ident, $tk:ident($v:ident)) => {
        match $tkns.next() {
            Some(Token::$tk($v)) => Some($v),
            _ => None,
        }
    };
    ($tkns:ident, $tk:ident) => {
        match $tkns.next() {
            Some(Token::$tk) => Some(Token::$tk),
            _ => None,
        }
    };
}

#[macro_export]
macro_rules! peek {
    ($tkns:ident, $tk:ident($v:ident)) => {
        match $tkns.peek() {
            Some(Token::$tk($v)) => Some($v),
            _ => None,
        }
    };
    ($tkns:ident, $tk:ident) => {
        match $tkns.peek() {
            Some(Token::$tk) => Some(Token::$tk),
            _ => None,
        }
    };
}

#[macro_export]
macro_rules! ignore_space {
    ($tkns:ident) => {
        if $tkns.peek() == Some(&Token::Space) {
            $tkns.next();
        }
    };
}

#[macro_export]
macro_rules! ignore_line_space {
    ($tkns:ident) => {
        while $tkns.peek() == Some(&Token::Space) || $tkns.peek() == Some(&Token::NewLine) {
            $tkns.next();
        }
    };
}
