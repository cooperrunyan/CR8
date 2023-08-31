#[macro_export]
macro_rules! err {
    ($err:expr $(, $args:expr)*) => {
        Err(format!($err $(, $args )* ))
    };
}

#[macro_export]
macro_rules! next {
            ($tkns:ident, $tk:ident$(($v:ident))? $(, $err:literal)?) => {
                match $tkns.next() {
                    Some(Token::$tk$(($v))?) => {$($v)?},
                    x => {
                        #[allow(unused_mut, unused_assignments)]
                        let mut e = Err(format!("Invalid syntax at: {:#?}", x));
                        $( e = Err(format!($err)); )?
                        e
                    }?,
                }
            };
        }

#[macro_export]
macro_rules! expect_any {
    ($tkns:ident) => {
        match $tkns.next() {
            Some(t) => t,
            None => Err(format!("Expected symbol, found EOL"))?,
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
