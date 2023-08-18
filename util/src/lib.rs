pub fn parse_num<T>(str: &str) -> T
where
    T: From<u64>,
{
    let str = str.trim().trim_start_matches("$");

    if str.ends_with('B') {
        let str = str.trim_end_matches('B');
        let val: u64 =
            u64::from_str_radix(str, 2).expect(&format!("Failed to parse {} as binary", str));
        val.into()
    } else if str.ends_with('H') {
        let str = str.trim_end_matches('H');
        let val: u64 =
            u64::from_str_radix(str, 16).expect(&format!("Failed to parse {} as hex", str));
        val.into()
    } else if str.ends_with('D') {
        let str = str.trim_end_matches('D');
        let val: u64 = match str.trim().parse() {
            Ok(v) => v,
            Err(_) => panic!("Failed to parse {} as a number", str),
        };
        val.into()
    } else {
        panic!("End literal numbers with D H or B to signify their base")
    }
}
