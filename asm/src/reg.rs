use std::fmt::Display;

/// Single-byte data gets stored into a [Register].
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Register {
    /// %a
    A,
    /// %b
    B,
    /// %c
    C,
    /// %d
    D,
    /// %x
    X,
    /// %y
    Y,
    /// %z
    Z,
    /// %f
    F,
    /// %k
    K,
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Register as R;
        let str = match self {
            R::A => "a",
            R::B => "b",
            R::C => "c",
            R::D => "d",
            R::X => "x",
            R::Y => "y",
            R::Z => "z",
            R::F => "f",
            R::K => "k",
        };
        f.write_str(str)
    }
}

impl TryFrom<&str> for Register {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Register as R;
        Ok(match value {
            "a" => R::A,
            "b" => R::B,
            "c" => R::C,
            "d" => R::D,
            "x" => R::X,
            "y" => R::Y,
            "z" => R::Z,
            "f" => R::F,
            "k" => R::K,
            _ => Err(())?,
        })
    }
}

impl TryFrom<u8> for Register {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Register as R;
        Ok(match value {
            0 => R::A,
            1 => R::B,
            2 => R::C,
            3 => R::D,
            4 => R::X,
            5 => R::Y,
            6 => R::Z,
            7 => R::F,
            8 => R::K,
            _ => Err(())?,
        })
    }
}
