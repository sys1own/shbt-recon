//! `shbt-recon` binary — native sub-CLI for the workspace (the Python
//! orchestrator `python/shbt_recon/cli/main.py` is the primary interface).

use std::path::PathBuf;

fn main() -> std::process::ExitCode {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("verify") => {
            let gates = shbt_recon_cli::verify_matrix();
            println!("{}", shbt_recon_cli::gates_to_json(&gates));
            let failed = gates.iter().filter(|g| !g.passed).count();
            if failed > 0 {
                eprintln!("[verify] {failed} gate(s) failed");
                return std::process::ExitCode::FAILURE;
            }
            std::process::ExitCode::SUCCESS
        }
        Some("export-eda") => {
            let dir = args
                .next()
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("eda_outputs"));
            match shbt_recon_cli::export_eda(&dir) {
                Ok((gds, step)) => {
                    println!("[export-eda] {} {}", gds.display(), step.display());
                    std::process::ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("[export-eda] {e}");
                    std::process::ExitCode::FAILURE
                }
            }
        }
        _ => {
            eprintln!("usage: shbt-recon <verify|export-eda [dir]>");
            std::process::ExitCode::FAILURE
        }
    }
}
