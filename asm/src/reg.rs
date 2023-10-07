use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Register {
    A,
    B,
    C,
    D,
    Z,
    L,
    H,
    F,
    PCL,
    PCH,
    SPL,
    SPH,
    MB,
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Register as R;
        let str = match self {
            R::A => "a",
            R::B => "b",
            R::C => "c",
            R::D => "d",
            R::Z => "z",
            R::L => "l",
            R::H => "h",
            R::F => "f",
            R::PCL => "pcl",
            R::PCH => "pch",
            R::SPL => "spl",
            R::SPH => "sph",
            R::MB => "mb",
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
            "z" => R::Z,
            "l" => R::L,
            "h" => R::H,
            "f" => R::F,
            "pcl" => R::PCL,
            "pch" => R::PCH,
            "spl" => R::SPL,
            "sph" => R::SPH,
            "mb" => R::MB,
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
            4 => R::Z,
            5 => R::L,
            6 => R::H,
            7 => R::F,
            8 => R::PCL,
            9 => R::PCH,
            10 => R::SPL,
            11 => R::SPH,
            12 => R::MB,
            _ => Err(())?,
        })
    }
}
