macro_rules! ignore_space {
    ($tkns:expr) => {
        if $tkns.peek() == Some(&Token::Space) {
            $tkns.next();
        }
    };
}

macro_rules! ignore_line_space {
    ($tkns:expr) => {
        while $tkns.peek() == Some(&Token::Space) || $tkns.peek() == Some(&Token::NewLine) {
            $tkns.next();
        }
    };
}

macro_rules! err {
    ($err:expr $(, $arg:expr )*) => {
        Err(format!($err $(, $arg)*))
    }
}

macro_rules! defnext {
    ($self:ident, $name:ident, $t:ident$(($a:ident))?) => {
        macro_rules! $name {
            ($err:expr) => {{
                let w = match $self.tokens.next() {
                    Some(Token::$t $(($a))?) => ($($a)?),
                    _ => Err(format!($err))?,
                };
                w
            }};
        }
    };
}
