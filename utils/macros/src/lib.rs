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

#[macro_export]
macro_rules! define_banks {
    ($idv:vis enum $id:ident, $v:vis struct $name:ident { $( $member:ident($val:literal) if $feature:literal,)* }) => {
        #[allow(non_snake_case)]
        #[derive(Default)]
        $v struct $name {
            $( #[cfg(feature = $feature)] $member: Bank, )*
        }

        encodable! {
            $idv enum $id {
                else UNKNOWN,
                Builtin(0x00),
                $( $member($val), )*
            }
        }

        impl $id {
            pub fn check(id: impl TryInto<Self> + Debug + Clone) -> Result<Self> {
                let i = id.clone();
                match id.try_into() {
                    Ok(Self::Builtin) => Ok(Self::Builtin),
                    $(
                        #[cfg(feature = $feature)]
                        Ok(Self::$member) => Ok(Self::$member),
                    )*
                    Ok(oth) => bail!("Memory bank: {oth:#?} is not connected"),
                    _ => bail!("Memory bank: {i:#?} is not defined"),
                }
            }
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut d = f.debug_struct("{$name}");

                d.field("Builtin", &0);

                $(
                    #[cfg(feature = $feature)]
                    d.field("$member", &($id::$member as u8));
                )*

                d.finish()
            }
        }


        impl<'b> $name {
            pub fn get(&'b self, id: impl TryInto<$id> + Debug + Clone) -> Result<Option<&'b Bank>> {
                let i = id.clone();
                Ok(match id.try_into() {
                    Ok($id::Builtin) => None,
                    $(
                        #[cfg(feature = $feature)]
                        Ok($id::$member) => Some(&self.$member),
                    )*
                    Ok(oth) => bail!("Memory bank: {oth:#?} is not connected"),
                    _ => bail!("Memory bank: {i:#?} is not defined"),
                })
            }

            pub fn get_mut(&'b mut self, id: impl TryInto<$id> + Debug + Clone) -> Result<Option<&'b mut Bank>> {
                let i = id.clone();
                Ok(match id.try_into() {
                    Ok($id::Builtin) => None,

                    $(
                        #[cfg(feature = $feature)]
                        Ok($id::$member) => Some(&mut self.$member),
                    )*

                    Ok(oth) => bail!("Memory bank: {oth:#?} is not connected"),
                    _ => bail!("Memory bank: {i:#?} is not defined"),
                })
            }
        }
    };
}
