use humansize::{format_size, DECIMAL};

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let str_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    let (file, output) = match str_args.as_slice() {
        [file] => (file, file),
        [file, "-o", output] => (file, output),
        ["-o", output, file] => (file, output),
        _ => {
            println!("Please provide a path to an ELF file as argument.");
            println!("Usage: [program] file");
            println!("Usage: [program] file -o output");
            return;
        }
    };

    let contents = std::fs::read(file).unwrap();

    let mut elf = elf_repack::read::read_elf_file(&contents).unwrap();

    let stripped = elf_repack::strip_zeroes(&mut elf);

    let repacked = elf_repack::write::create_elf_file(&elf);

    std::fs::write(output, &repacked).unwrap();

    let abs_reduction = contents.len() - repacked.len();
    let rel_size = repacked.len() as f64 / contents.len() as f64;
    // reduction in percentage
    let rel_reduction = ((1.0 - rel_size) * 100.0).round() as u8;

    println!("Summary");
    println!(
        "  File size before: {}",
        format_size(contents.len(), DECIMAL)
    );
    println!(
        "  File size after:  {}",
        format_size(repacked.len(), DECIMAL)
    );

    if stripped > 0 {
        println!(
            "  Trailing zeroes stripped: {}",
            format_size(stripped, DECIMAL)
        );
    }

    println!();
    println!(
        "  Reduction:        {} ({}%)",
        format_size(abs_reduction, DECIMAL),
        rel_reduction
    );
}
