use super::directive::import;

pub fn with_builtins(file: &str) -> String {
    let builtin_file = include_bytes!("./builtin.asm").to_vec();
    let builtin_file =
        String::from_utf8(builtin_file).expect("Built in builtin file bad formatting");

    let builtin = import::include(&builtin_file);

    format!("{builtin}\n{file}")
}
