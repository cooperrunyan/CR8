#[macro_export]
macro_rules! encodable {
    ($v:vis enum $n:ident {
        $($member:ident($val:literal, $str:literal),)*
    }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        $v enum $n {
            $($member,)*
        }

        impl TryFrom<u8> for $n {
            type Error = ();
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    $( $val => Ok($n::$member), )*
                    _ => Err(()),
                }
            }
        }

        impl TryFrom<&str> for $n {
            type Error = ();
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match value {
                    $( $str => Ok($n::$member), )*
                    _ => Err(()),
                }
            }
        }
    };
    ($v:vis enum $n:ident {
        $($member:ident($val:literal),)*
    }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        $v enum $n {
            $($member,)*
        }

        impl TryFrom<u8> for $n {
            type Error = ();
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    $( $val => Ok($n::$member), )*
                    _ => Err(()),
                }
            }
        }
    };

    ($v:vis enum $n:ident {
        else $def:ident,
        $($member:ident($val:literal),)*
    }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        $v enum $n {
            $def,
            $($member,)*
        }

        impl From<u8> for $n {
            fn from(value: u8) -> Self {
                match value {
                    $( $val => $n::$member, )*
                    _ => $n::$def,
                }
            }
        }
    };

    ($v:vis enum $n:ident {
        else $def:ident,
        $($member:ident($val:literal, $str:literal),)*
    }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        $v enum $n {
            $def,
            $($member,)*
        }

        impl From<u8> for $n {
            fn from(value: u8) -> Self {
                match value {
                    $( $val => $n::$member, )*
                    _ => $n::$def,
                }
            }
        }

        impl From<&str> for $n {
            fn from(value: &str) -> Self {
                match value {
                    $( $str => $n::$member, )*
                    _ => $n::$def,
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_into_as {
    (for $for:ident, as $a:ty, impl $($t:ty),+) => {
        $(impl TryFrom<$t> for $for {
            type Error = ();
            fn try_from(value: $t) -> Result<Self, Self::Error> {
                Self::try_from(value as $a)
            }
        })*
    }
}
