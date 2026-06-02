use std::error::Error;
use std::path::{Path, PathBuf};

use homotopycontinuationdriver::{
    Polynomial, TrackConfig, TrackedRoot, factor_polynomial, write_core_vectors,
    write_homotopy_step_vectors,
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
    let kind = args.first().map(String::as_str).unwrap_or("cubic_demo");
    if kind != "cubic_demo" && kind != "homotopy_step" {
        return Err(format!("unknown vector kind: {kind}").into());
    }

    let mut output = default_vector_path(kind)?;
    let mut index = 1;

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

    match kind {
        "cubic_demo" => write_core_vectors(&output)?,
        "homotopy_step" => write_homotopy_step_vectors(&output)?,
        _ => unreachable!("validated vector kind"),
    }
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

fn default_vector_path(kind: &str) -> Result<PathBuf, Box<dyn Error>> {
    let filename = match kind {
        "cubic_demo" => "homotopy_core_vectors.mem",
        "homotopy_step" => "homotopy_step_vectors.mem",
        _ => return Err(format!("unknown vector kind: {kind}").into()),
    };
    let repo_relative = Path::new("Verilog/sim/vectors").join(filename);
    if Path::new("Verilog/source").exists() {
        Ok(repo_relative)
    } else {
        Ok(Path::new("../Verilog/sim/vectors").join(filename))
    }
}

fn print_usage() {
    println!("Usage:");
    println!("  cargo run -- factor <c0> <c1> ... <cd>");
    println!("  cargo run -- vectors cubic_demo [--output <path>]");
    println!("  cargo run -- vectors homotopy_step [--output <path>]");
    println!();
    println!("Coefficients are in increasing degree order: c0 + c1*z + ... + cd*z^d.");
}
