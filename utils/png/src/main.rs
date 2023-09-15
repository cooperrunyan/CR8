use std::env::args;
use std::fs;

fn main() {
    let args = args().collect::<Vec<_>>();
    let name = args.get(1).expect("Expected a name");
    let input = args.get(2).expect("Expected input file");
    let output = args.get(3).expect("Expected output file");

    let img = image::open(input).expect("Failed to open input image");

    let img = img.to_rgb8();

    let mut asm = String::new();
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
        asm.push_str(&format!("{:#010b}, ", byte));
    }

    fs::write(output, format!("#mem {len} {name} [ {asm} ]")).unwrap();
}
