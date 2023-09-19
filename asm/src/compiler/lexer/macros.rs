macro_rules! while_peek {
    ($self:ident, $n:ident, $do:block) => {
        while let Some($n) = $self.tokens.peek() {
            let $n = $n.token.clone();
            $do;
            $self.tokens.next();
        }
    };
}

macro_rules! while_next {
    ($self:ident, $n:ident, $do:block) => {
        while let Some($n) = $self.tokens.next() {
            $do
        }
    };
}

macro_rules! ignore {
    ($self:ident, $p:pat) => {
        while_peek!($self, n, {
            match n {
                $p => {}
                _ => break,
            }
        });
    };
}

macro_rules! next {
    ($self:ident) => {
        match $self.tokens.next() {
            Some(t) => t,
            _ => bail!("Expected token but found none"),
        }
    };
}
macro_rules! expect {
    ($self:ident, $msg:expr, match $is:ident $( | $oneof:ident)*) => {
        {
            let next = next!($self);
            if next.token.$is() {
                next.token
            }
            $(else if next.token.$oneof() { next.token })*
            else { Err(anyhow!($msg).context(format!("File: {:?}:{}:{}", next.path, next.line + 1, next.col + 1)))? }
        }
    };
    ($self:ident, $msg:expr, $t:ident($inner:ident)) => {
        {
            let next = next!($self);
            match next.token {
            Token::$t($inner) => $inner,
            _ => Err(anyhow!($msg).context(format!("File: {:?}:{}:{}", next.path, next.line + 1, next.col + 1)))?
        }}
    };
}

macro_rules! while_next_in {
    ($in:ident, $n:ident, $do:block) => {
        while let Some($n) = $in.next() {
            $do
        }
    };
}

macro_rules! next_in {
    ($in:ident) => {
        match $in.next() {
            Some(t) => t,
            _ => bail!("Expected token but found none"),
        }
    };
}
macro_rules! expect_in {
    ($in:ident, $msg:expr, match $is:ident $( | $oneof:ident)*) => {
        {
            let next = next_in!($in);
            if next.token.$is() {
                next.token
            }
            $(else if next.token.$oneof() { next.token })*
            else { Err(anyhow!($msg).context(format!("File: {:?}:{}:{}", next.path, next.line + 1, next.col + 1)))? }
        }
    };
    ($in:ident, $msg:expr, $t:ident($inner:ident)) => {
        {
            let next = next_in!($in);
            match next.token {
            Token::$t($inner) => $inner,
            _ => Err(anyhow!($msg).context(format!("File: {:?}:{}:{}", next.path, next.line + 1, next.col + 1)))?
        }}
    };
}
