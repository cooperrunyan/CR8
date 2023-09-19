use std::env::args;
use std::fs;

fn main() {
    let args = args().collect::<Vec<_>>();
    let input = args.get(1).expect("Expected input file");
    let dindx = input.rfind(".").expect("Expected a '.' in file path");
    let slindx = input.rfind("/").map(|i| i + 1).unwrap_or(0);

    let name = &input[slindx..dindx].to_ascii_uppercase();

    let img = image::open(input).expect("Failed to open input image");
    let w = img.width() * 8;
    let h = img.height() * 8;

    let img = img.to_rgb8();

    let mut bytes = String::new();
    let mut len = 0;

    for x in img.chunks_exact(24).into_iter() {
        len += 1;
        let mut byte = 0;
        for j in 0..8 {
            let r = x[j * 3] as u16;
            let g = x[(j * 3) + 1] as u16;
            let b = x[(j * 3) + 2] as u16;
            if (r + g + b) / 3 > 128 {
                byte |= 1 << (7 - j);
            };
        }
        bytes.push_str(&format!("{:#04X},", byte));
        if len % 32 == 0 {
            bytes.push_str("\n    ");
        }
    }

    let def = format!(
        r#"#[static({name}_W: {w:#06X})]
#[static({name}_WL: {:#04X})]
#[static({name}_WH: {:#04X})]
#[static({name}_H: {h:#06X})]
#[static({name}_HL: {:#04X})]
#[static({name}_HH: {:#04X})]
#[static({name}_SZ: {len:#06X})]
#[static({name}_SZH: {:#04X})]
#[static({name}_SZL: {:#04X})]"#,
        (w >> 8) as u8,
        w as u8,
        (h >> 8) as u8,
        h as u8,
        (len >> 8) as u8,
        len as u8,
    );

    println!("{def}");

    fs::write(
        format!("./{name}.asm"),
        format!(
            r#"; Generated with "utils/png"

{def}

#[mem( {name} )] {{
    {bytes}
}}"#
        ),
    )
    .unwrap();
}
