use std::io::stdin;

use anyhow::Result;
use log::{info, warn};

use super::DeviceSnapshot;

macro_rules! byte {
    ($name:expr, $val:expr) => {
        format!("{}:  {:#04x} | {:#3} | {:08b}", $name, $val, $val, $val)
    };
}

/// Communicates with the [crate::cr8::CR8].
/// When it receives a byte, it will parse it as a [SysCtrlSignal].
#[derive(Debug, Default)]
pub struct SysCtrl {
    pub state: u8,

    /// Whether or not to ignore breakpoints
    pub debug: bool,

    debugger: Option<Debugger>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SysCtrlSignal {
    /// Ping the controller
    Ping,

    /// Stop the clock
    Halt,

    /// Print [crate::cr8::CR8] state data
    Debug,

    /// Pause the clock until stdin receives a line
    Breakpoint,
}

impl TryFrom<u8> for SysCtrlSignal {
    type Error = ();
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        use SysCtrlSignal as SIG;
        Ok(match value {
            0x00 => SIG::Ping,
            0x01 => SIG::Halt,
            0x02 => SIG::Debug,
            0x03 => SIG::Breakpoint,
            _ => Err(())?,
        })
    }
}

impl SysCtrl {
    pub fn new(debug: bool) -> Self {
        Self {
            debug,
            ..Default::default()
        }
    }

    pub fn send(&self) -> Result<u8> {
        Ok(self.state)
    }

    pub fn receive(&mut self, byte: u8, dev: DeviceSnapshot) -> Result<()> {
        if let Some(ref mut debugger) = &mut self.debugger {
            if debugger.next(byte) {
                debugger.debug(dev);
                self.debugger = None;
            } else {
                return Ok(());
            }
        }

        use SysCtrlSignal as SIG;
        match SIG::try_from(byte) {
            Ok(s) => match s {
                SIG::Ping => info!("PONG"),
                SIG::Halt => self.state |= 0b00000001,
                SIG::Debug => {
                    self.debugger = Some(Debugger {
                        stage: DebugStage::Empty,
                        data: [0; 9],
                    })
                }
                SIG::Breakpoint => {
                    if !self.debug {
                        return Ok(());
                    }
                    let mut inp = String::new();
                    stdin().read_line(&mut inp)?;
                }
            },
            Err(_) => warn!("sysctrl recieved unknown {byte:#?} message"),
        };
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default)]
enum DebugStage {
    #[default]
    Empty,
    A,
    B,
    C,
    D,
    Z,
    F,
    L,
    H,
    MB,
}

impl DebugStage {
    fn inc(self) -> Option<Self> {
        use DebugStage as S;
        Some(match self {
            S::Empty => S::A,
            S::A => S::B,
            S::B => S::C,
            S::C => S::D,
            S::D => S::Z,
            S::Z => S::F,
            S::F => S::L,
            S::L => S::H,
            S::H => S::MB,
            S::MB => None?,
        })
    }
}

#[derive(Debug, Default)]
struct Debugger {
    stage: DebugStage,
    data: [u8; 9],
}

impl Debugger {
    fn next(&mut self, byte: u8) -> bool {
        if let Some(next) = self.stage.inc() {
            self.data[next as usize - 1] = byte;
            self.stage = next;
            false
        } else {
            true
        }
    }

    fn debug(&self, dev: DeviceSnapshot) {
        let mut snapshot = String::new();

        snapshot.push_str(&format!("    - sysctrl: {:#010b}", dev.sysctrl));

        #[cfg(feature = "keyboard")]
        snapshot.push_str(&format!("\n    - keyboard: {:#010b}", dev.keyboard));

        #[cfg(feature = "rng")]
        snapshot.push_str(&format!("\n    - rng: {:#010b}", dev.rng));

        let [a, b, c, d, z, f, l, h, mb] = self.data;

        info!(
            "\n{}",
            format!(
                r#"
================== State ==================

  {}
  {}
  {}
  {}
  {}
  {}
  {}
  {}
  {}

  Memory banks:
{:?}

  Devices:
{}

===========================================
"#,
                byte!("A", a),
                byte!("B", b),
                byte!("C", c),
                byte!("D", d),
                byte!("Z", z),
                byte!("F", f),
                byte!("L", l),
                byte!("H", h),
                byte!("MB", mb),
                "banks",
                snapshot
            )
        );
    }
}
