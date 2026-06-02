use std::error::Error;
use std::path::{Path, PathBuf};

use homotopycontinuationdriver::{
    Polynomial, TrackConfig, TrackedRoot, factor_polynomial, write_core_vectors,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.first().map(String::as_str) {
        None => factor_command(&["1".into(), "2".into(), "3".into(), "4".into()]),
        Some("factor") => factor_command(&args[1..]),
        Some("vectors") => vectors_command(&args[1..]),
        Some("help") | Some("--help") | Some("-h") => {
            print_usage();
            Ok(())
        }
        Some(command) => Err(format!("unknown command: {command}").into()),
    }
}

fn factor_command(args: &[String]) -> Result<(), Box<dyn Error>> {
    if args.is_empty() {
        return Err("provide coefficients in increasing degree order".into());
    }

    let coefficients = args
        .iter()
        .map(|arg| arg.parse::<f64>())
        .collect::<Result<Vec<_>, _>>()?;
    let polynomial = Polynomial::new(coefficients);
    let roots = factor_polynomial(polynomial.clone(), TrackConfig::default())?;

    println!("f(z) = {polynomial}");
    print_roots(&roots);
    Ok(())
}

fn vectors_command(args: &[String]) -> Result<(), Box<dyn Error>> {
    let mut output = default_vector_path();
    let mut index = 0;

    if args.get(index).map(String::as_str) == Some("cubic_demo") {
        index += 1;
    }

    while index < args.len() {
        match args[index].as_str() {
            "--output" | "-o" => {
                index += 1;
                output = PathBuf::from(args.get(index).ok_or("--output requires a path argument")?);
            }
            other => return Err(format!("unknown vectors argument: {other}").into()),
        }
        index += 1;
    }

    write_core_vectors(&output)?;
    println!("wrote {}", output.display());
    Ok(())
}

fn print_roots(roots: &[TrackedRoot]) {
    for root in roots {
        println!(
            "path {}: start {} -> root {} | residual {:.3e} | Newton iterations {}",
            root.path_index,
            root.start_point,
            root.root,
            root.residual,
            root.total_newton_iterations
        );
    }
}

fn default_vector_path() -> PathBuf {
    let repo_relative = Path::new("Verilog/sim/vectors/homotopy_core_vectors.mem");
    if Path::new("Verilog/source").exists() {
        repo_relative.to_path_buf()
    } else {
        PathBuf::from("../Verilog/sim/vectors/homotopy_core_vectors.mem")
    }
}

fn print_usage() {
    println!("Usage:");
    println!("  cargo run -- factor <c0> <c1> ... <cd>");
    println!("  cargo run -- vectors cubic_demo [--output <path>]");
    println!();
    println!("Coefficients are in increasing degree order: c0 + c1*z + ... + cd*z^d.");
}
