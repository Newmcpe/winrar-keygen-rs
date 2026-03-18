use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 || args.len() > 4 {
        eprintln!("Usage: winrar-keygen <username> <license-type> [output-file]");
        eprintln!("Example: winrar-keygen \"GitHub\" \"Single PC usage license\" rarreg.key");
        std::process::exit(1);
    }

    let output = winrar_keygen::keygen::generate_license_text(&args[1], &args[2]);

    if args.len() == 4 {
        std::fs::write(&args[3], &output).unwrap();
        eprintln!("Saved to {}", args[3]);
    }

    std::io::stdout().lock().write_all(output.as_bytes()).unwrap();
}
