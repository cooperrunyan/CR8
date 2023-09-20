use anyhow::{bail, Result};
use std::fmt::Debug;

use super::{BANK_LEN, BANK_MASK, RAM_MASK};

crate::define_banks! {
    pub enum BankId,
    pub struct BankCollection {
        Vram(0x01) if "gfx",
    }
}

pub(super) fn mask(idx: usize) -> usize {
    idx & RAM_MASK
}

pub(super) fn smallmask(idx: usize) -> usize {
    idx & BANK_MASK
}

#[derive(Debug)]
pub struct Bank([u8; BANK_LEN]);

impl Default for Bank {
    fn default() -> Self {
        Self([0; BANK_LEN])
    }
}

impl Bank {
    pub fn get(&self, idx: usize) -> Option<u8> {
        self.0.get(smallmask(idx)).map(|b| *b)
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut u8> {
        self.0.get_mut(smallmask(idx))
    }

    pub fn set(&mut self, idx: usize, val: u8) -> Result<()> {
        let byte = self.0.get_mut(smallmask(idx)).unwrap();
        *byte = val;
        Ok(())
    }
}

#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! define_banks {
    ($idv:vis enum $id:ident, $v:vis struct $name:ident { $( $member:ident($val:literal) if $feature:literal,)* }) => {
        #[allow(non_snake_case)]
        #[derive(Default)]
        $v struct $name {
            $( #[cfg(feature = $feature)] pub(super) $member: Bank, )*
        }

        #[derive(Debug, Clone, Copy, Eq, PartialEq)]
        $idv enum $id {
            Builtin,
            $( $member, )*
        }

        impl TryInto<u8> for $id {
            type Error = ();
            #[allow(unreachable_patterns)]
            fn try_into(self) -> Result<u8, Self::Error> {
                Ok(match self {
                    Self::Builtin => 0x00,
                     $( Self::$member => $val, )*
                    _ => Err(())?
                })
            }
        }

        impl TryFrom<u8> for $id {
            type Error = ();
            fn try_from(val: u8) -> Result<Self, Self::Error> {
                Ok(match val {
                    0x00 => Self::Builtin,
                     $( $val => Self::$member, )*
                    _ => Err(())?
                })
            }
        }

        impl $id {
            pub fn check(id: impl TryInto<Self> + Debug + Clone) -> Result<Self> {
                let i = id.clone();
                #[allow(unreachable_patterns)]
                match id.try_into() {
                    Ok(Self::Builtin) => Ok(Self::Builtin),
                    $(
                        #[cfg(feature = $feature)]
                        Ok(Self::$member) => Ok(Self::$member),
                    )*
                    Ok(_) => bail!("Memory bank: {i:#?} is not connected"),
                    _ => bail!("Memory bank: {i:#?} is not defined"),
                }
            }
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "      Builtin: 0x00\n")?;

                $(
                    #[cfg(feature = $feature)]
                    write!(f, "      {:?}: {:#04x}\n", $id::$member, TryInto::<u8>::try_into($id::$member).unwrap())?;
                )*

                write!(f, "")
            }
        }


        impl<'b> $name {
            pub fn get(&'b self, id: impl TryInto<$id> + Debug + Clone) -> Result<Option<&'b Bank>> {
                let i = id.clone();
                #[allow(unreachable_patterns)]
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
                #[allow(unreachable_patterns)]
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
}
