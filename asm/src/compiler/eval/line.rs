use regex::Regex;

use super::input::InputInstruction;

lazy_static! {
    static ref LABEL: Regex = Regex::new(r"^(\w+):$").unwrap();
}

pub fn line(line: &str, line_num: usize) -> Option<Line> {
    let line = line.trim();

    if line.is_empty() {
        return None;
    }

    if LABEL.is_match(line) {
        let label_name = line.trim_end_matches(':').to_string();
        return Some(Line::Label((label_name.clone(), line_num as u16)));
    }

    Some(Line::InputInstruction(InputInstruction::from(line)))
}

#[derive(Debug)]
pub enum Line {
    InputInstruction(InputInstruction),
    Label((String, u16)),
}
