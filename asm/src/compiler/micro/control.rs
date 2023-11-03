use std::fmt::{Debug, Display};

use anyhow::bail;

use super::{
    AddressBusWriter, AluSignal, DataBusReader, DataBusWriter, MicroSignal, ProgramCounterSignal,
    StackPointerSignal,
};

/// ```text
/// [0,        1,        2       ]
/// [ABCDEFGH, IJKKKLLM, NOPPPQRS]
/// ```
/// - `A`: Read into `K` from databus
/// - `B`: Read into `F` from databus
/// - `C`: Read into `OP` from databus
/// - `D`: Read into `IO` from databus
/// - `E`: Write into RAM the value in databus
/// - `F`: Read into `Lhs` from databus
/// - `G`: Read into `Rhs` from databus
/// - `H`: Allow selected device to read from databus
///      - (device connected to the port of the value in `IO`)
/// - `I`: Read into register has the ID of the value in `Lhs` from databus
/// - `J`: Write to databus whatever register has the ID of the value in `Rhs`
/// - `K`: Databus writer ID:
///     - `Device`
///     - `K`
///     - `Alu Flags`
///     - `Alu`
///     - `Memory`
///     - `Io`
///     - `Rhs`
///     - `Operation`
/// - `L`: Addressbus writer ID:
///     - Program Counter
///     - Stack Pointer
///     - `X` (low) + `Y` (high)
///     - `Lhs` (low) + `Rhs` (high)
/// - `M`: `PC JNZ` (read into `PC` from address bus if alu `ZeroFlag` is not 1)
/// - `N`: `PC JMP` (read into `PC` from address bus)
/// - `O`: Increment `PC` (at the end of the clock tick)
/// - `P`: ALU Operation:
///     - `Add`
///     - `Sub`
///     - `And`
///     - `Or`
///     - `Nor`
///     - `Cmp`
/// - `Q`: Increment Stack Pointer
/// - `R`: Decrement Stack Pointer
/// - `S`: Cycle Complete
///     - (tells the Control Unit this is the last micro instruction of the cycle)
pub struct RawControlSignal(pub [u8; 3]);

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub struct ControlSignal {
    alu_op: Option<AluSignal>,
    databus_writer: Option<DataBusWriter>,
    addressbus_writer: Option<AddressBusWriter>,
    databus_write_select: bool,
    databus_read_select: bool,
    pc_jnz: bool,
    pc_jmp: bool,
    pc_inc: bool,
    sp_inc: bool,
    sp_dec: bool,
    dr_k: bool,
    dr_f: bool,
    dr_io: bool,
    dr_op: bool,
    dr_mem: bool,
    dr_lhs: bool,
    dr_rhs: bool,
    dr_dev: bool,
}

impl Display for RawControlSignal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:08b} {:08b} {:08b}",
            self.0[0], self.0[1], self.0[2]
        ))
    }
}

impl Debug for RawControlSignal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl From<RawControlSignal> for u32 {
    fn from(val: RawControlSignal) -> Self {
        let mut r = 0u32;
        r |= val.0[0] as u32;
        r |= (val.0[1] as u32) << 8;
        r |= (val.0[2] as u32) << 16;
        r
    }
}

impl TryFrom<&Vec<MicroSignal>> for ControlSignal {
    type Error = anyhow::Error;
    fn try_from(value: &Vec<MicroSignal>) -> std::result::Result<Self, Self::Error> {
        let mut control = ControlSignal::default();

        macro_rules! set {
            ($field:ident, $sig:literal, $val:ident, Some $(|| $ex:expr)?) => {{
                if control.$field.is_some() $(|| $ex)? {
                    bail!("Cannot set {:#?} twice", $sig);
                }
                control.$field = Some(*$val);
            }};
            ($field:ident, $sig:literal, true $(|| $ex:expr)?) => {{
                if control.$field $(|| $ex)? {
                    bail!("Cannot set {:#?} twice", $sig);
                }
                control.$field = true;
            }};
        }

        for signal in value {
            match signal {
                MicroSignal::AddressBusWrite(aw) => set!(addressbus_writer, "aw", aw, Some),
                MicroSignal::Alu(alu_op) => set!(alu_op, "alu", alu_op, Some),
                MicroSignal::DataBusRead(dr) => match dr {
                    DataBusReader::Device => set!(dr_dev, "dr dev", true),
                    DataBusReader::Flags => set!(dr_f, "dr f", true),
                    DataBusReader::Io => set!(dr_io, "dr io", true),
                    DataBusReader::Lhs => set!(dr_lhs, "dr lhs", true),
                    DataBusReader::Rhs => set!(dr_rhs, "dr rhs", true),
                    DataBusReader::Memory => set!(dr_mem, "dr mem", true),
                    DataBusReader::MemoryBank => set!(dr_k, "dr k", true),
                    DataBusReader::Sel => set!(databus_read_select, "dr sel", true),
                },
                MicroSignal::DataBusWrite(dw) => {
                    if matches!(dw, DataBusWriter::Sel) {
                        set!(
                            databus_write_select,
                            "dw",
                            true || control.databus_writer.is_some()
                        );
                    } else {
                        set!(
                            databus_writer,
                            "dw",
                            dw,
                            Some || control.databus_write_select
                        );
                    }
                }
                MicroSignal::ProgramCounter(pc) => match pc {
                    ProgramCounterSignal::Increment => set!(pc_inc, "pc inc", true),
                    ProgramCounterSignal::Jump => set!(pc_jmp, "pc jmp", true),
                    ProgramCounterSignal::JumpNotZero => set!(pc_jnz, "pc jnz", true),
                },
                MicroSignal::StackPointer(sp) => match sp {
                    StackPointerSignal::Decrement => set!(sp_dec, "sp dec", true),
                    StackPointerSignal::Increment => set!(sp_inc, "sp inc", true),
                },
            }
        }

        Ok(control)
    }
}

impl From<ControlSignal> for RawControlSignal {
    fn from(value: ControlSignal) -> Self {
        let mut bits = [0; 3];
        if value.dr_k {
            flag(&mut bits[0], 7);
        }
        if value.dr_f {
            flag(&mut bits[0], 6);
        }
        if value.dr_op {
            flag(&mut bits[0], 5);
        }
        if value.dr_io {
            flag(&mut bits[0], 4);
        }
        if value.dr_mem {
            flag(&mut bits[0], 3);
        }
        if value.dr_lhs {
            flag(&mut bits[0], 2);
        }
        if value.dr_rhs {
            flag(&mut bits[0], 1);
        }
        if value.dr_dev {
            flag(&mut bits[0], 0);
        }
        if value.databus_read_select {
            flag(&mut bits[1], 7);
        }
        if value.databus_write_select {
            flag(&mut bits[1], 6);
        }
        if let Some(dw) = value.databus_writer {
            bits[1] |= ((dw as u8) & 0b111) << 3;
        }
        if let Some(aw) = value.addressbus_writer {
            bits[1] |= ((aw as u8) & 0b11) << 1;
        }
        if value.pc_jnz {
            flag(&mut bits[1], 0);
        }
        if value.pc_jmp {
            flag(&mut bits[2], 7);
        }
        if value.pc_inc {
            flag(&mut bits[2], 6);
        }
        if let Some(op) = value.alu_op {
            bits[2] |= ((op as u8) & 0b111) << 3;
        }
        if value.sp_inc {
            flag(&mut bits[2], 2)
        }
        if value.sp_dec {
            flag(&mut bits[2], 1)
        }

        Self(bits)
    }
}

fn flag(byte: &mut u8, idx: u8) {
    *byte |= 1 << idx;
}
