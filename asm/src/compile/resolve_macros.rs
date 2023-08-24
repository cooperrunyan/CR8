use std::collections::{HashMap, VecDeque};

use super::scan::Macro;

pub fn resolve_macros(section: &str, macros: &HashMap<String, Macro>) -> String {
    let mut resolved = String::new();

    let mut queue = section
        .lines()
        .map(|l| l.to_string())
        .collect::<VecDeque<String>>();

    while !queue.is_empty() {
        let next = match queue.pop_front() {
            None => break,
            Some(n) => n,
        };

        let (ident, inputted_args) = next.split_once(' ').unwrap_or((next.as_str(), ""));
        let mac = match macros.get(ident) {
            None => {
                resolved.push_str(next.as_str());
                resolved.push('\n');
                continue;
            }
            Some(m) => m,
        };

        let inputted_args = inputted_args
            .split(',')
            .map(|a| a.trim())
            .filter(|a| !a.is_empty());

        let mut filled = mac.content.clone();

        let mut adj = 0;

        for (i, input) in inputted_args.enumerate() {
            let i = i - adj;
            if input.starts_with('[') && input.ends_with(']') {
                let mac_arg_name_l = match mac.args.get(i) {
                    None => panic!("Bad macro call: {next}"),
                    Some(a) => a,
                };
                let mac_arg_name_h = match mac.args.get(i + 1) {
                    None => panic!("Bad macro call: {next}"),
                    Some(a) => a,
                };
                filled = filled.replace(mac_arg_name_l, &format!("{}::0", input));
                filled = filled.replace(mac_arg_name_h, &format!("{}::1", input));
                adj += 1;
            } else {
                let mac_arg_name = match mac.args.get(i) {
                    None => panic!("Bad macro call: {next}"),
                    Some(a) => a,
                };
                filled = filled.replace(mac_arg_name, input);
            }
        }

        for new_line in filled.lines().rev() {
            queue.push_front(new_line.to_string());
        }
    }

    resolved
}
