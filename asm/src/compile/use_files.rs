use std::{
    fs,
    path::{Path, PathBuf},
};

use regex::Regex;

lazy_static! {
    pub static ref USE_RE: Regex = Regex::new(r#"@use\s*"(?<path>.+)""#).unwrap();
}

pub fn include_import_statements(file: &str, path: PathBuf, sourced: &mut Vec<PathBuf>) -> String {
    let mut inserts: Vec<(usize, usize, String)> = vec![];

    for mat in USE_RE.find_iter(file) {
        let start = mat.start();
        let end = mat.end();

        let cap = USE_RE.captures(mat.as_str()).unwrap();
        let mut relative_import_path = String::new();
        cap.expand("$path", &mut relative_import_path);

        #[allow(unused_assignments)]
        let mut pathbuf = Path::new("").to_path_buf();

        if relative_import_path.starts_with('/') {
            pathbuf = Path::new(&relative_import_path).to_path_buf();
        } else {
            pathbuf = path.join(Path::new(&format!("../{relative_import_path}")));
        }

        let insert_path = fs::canonicalize(pathbuf).unwrap();

        if !insert_path.exists() {
            panic!(
                "Error at file: {}:\n  Attempted to @use nonexistent file: {}",
                path.display(),
                insert_path.display()
            );
        }

        if sourced.contains(&insert_path) {
            panic!(
                "Attempting circular import at {}, importing {}",
                path.display(),
                insert_path.display()
            );
        }

        let imported_file = match fs::read_to_string(&insert_path) {
            Ok(f) => f,
            Err(e) => panic!("{e}"),
        };

        sourced.push(insert_path.clone());
        let full = include_import_statements(&imported_file, insert_path, sourced);

        inserts.push((start, end, full));
    }

    if inserts.is_empty() {
        return file.to_string();
    }

    let mut new_file = String::new();
    let mut last_end = 0;

    for (start, end, insert_file) in inserts {
        new_file.push_str(&file[last_end..start]);
        new_file.push_str(&insert_file);
        last_end = end;
    }

    new_file.push_str(&file[last_end..]);

    new_file
}
