use cfg::reg::Register;

#[derive(Debug)]
pub struct InputInstruction {
    pub inst: String,
    pub args: Vec<InputArg>,
}

impl From<&str> for InputInstruction {
    fn from(value: &str) -> Self {
        let value = value.trim();

        let (ident, args) = value.split_at(value.find(|c| c == ' ').unwrap());
        let args = args
            .split(',')
            .map(|a| a.trim())
            .filter(|a| !a.is_empty())
            .map(|a| {
                if let Some(reg) = a.strip_prefix('%') {
                    if reg.starts_with('[') && reg.ends_with(']') {
                        let reg = reg.trim_start_matches('[').trim_end_matches(']');
                        InputArg::Index(reg.to_string())
                    } else {
                        InputArg::Register(Register::from(reg))
                    }
                } else if a.starts_with('$') {
                    InputArg::Immediate(util::parse_num(a))
                } else if a.starts_with('[') && a.ends_with(']') {
                    let a = a.trim_start_matches('[').trim_end_matches(']');
                    InputArg::Index(a.to_string())
                } else {
                    panic!("Bad argument, {a}")
                }
            })
            .collect::<Vec<_>>();

        Self {
            inst: ident.to_string(),
            args,
        }
    }
}

#[derive(Debug)]
pub enum InputArg {
    Register(Register),
    Immediate(u64),
    Index(String),
}

impl ToString for InputArg {
    fn to_string(&self) -> String {
        match self {
            InputArg::Immediate(n) => format!("${n}D",),
            InputArg::Index(r) => format!("[{}]", r.to_string()),
            InputArg::Register(r) => format!("%{}", r.to_string()),
        }
    }
}
