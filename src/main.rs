use glob::glob;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::{env, fs, io};

fn download_binaryen(version: &str) -> Result<(), io::Error> {
    let download_url = format!(
        "https://github.com/WebAssembly/binaryen/releases/download/version_{}/binaryen-version_{}-x86_64-linux.tar.gz",
        version, version
    );
    println!("Downloading binaryen from: {}", download_url);

    let response = ureq::get(&download_url)
        .call()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let mut reader = response.into_reader();
    let decoder = flate2::read::GzDecoder::new(&mut reader);
    let mut archive = tar::Archive::new(decoder);

    // Extract only the wasm-opt binary
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        if let Some(filename) = path.file_name() {
            if filename == "wasm-opt" {
                entry.unpack("/binaryen/bin/wasm-opt")?;
                fs::set_permissions("/binaryen/bin/wasm-opt", fs::Permissions::from_mode(0o755))?;
                return Ok(());
            }
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "wasm-opt binary not found in archive",
    ))
}

fn main() -> Result<(), io::Error> {
    // Github passes empty strings for not defined optional parameters.
    // We have multiple optional params, so we need to keep their order.
    let mut args: Vec<Option<String>> = env::args()
        .map(|arg| if !arg.is_empty() { Some(arg) } else { None })
        .collect();
    if args.iter().filter(|arg| arg.is_some()).count() < 2 {
        panic!("You need to at least define a file to optimize");
    }
    args.remove(0);
    println!("Got arguments: {:?}", args);

    args.reverse();

    // Get binaryen version
    let binaryen_version = args
        .get(0)
        .and_then(|arg| arg.clone())
        .unwrap_or_else(|| "122".to_string());

    if !std::path::Path::new("/binaryen/bin/wasm-opt").exists() {
        download_binaryen(&binaryen_version)?;
    }

    let glob_input = format!(
        "/github/workspace/{}",
        args.pop().unwrap().unwrap().trim_start_matches('/')
    );
    println!("Searching for file matching '{glob_input}'");
    let file_paths = glob(&glob_input).expect("Failed to read glob pattern");
    let output = args.pop().unwrap();
    let optimize_all = args.pop().unwrap().unwrap_or("false".to_owned()) == "true";
    let options = args.pop().unwrap().unwrap_or("-Os".to_owned());

    for path in file_paths {
        let input = path
            .expect("Failed to read path")
            .to_str()
            .expect("Path should be string")
            .to_owned();
        println!("Optimizing '{input}'");

        // Log input file size
        let input_size = fs::metadata(&input).map(|m| m.len()).unwrap_or_else(|_| 0);
        println!("Input file size: {} bytes", input_size);

        let output = if let Some(output) = output.clone() {
            format!("/github/workspace/{}", output.trim_start_matches('/'))
        } else {
            input.clone()
        };
        println!("Writing optimized wasm file to '{output}'");
        println!("Executing 'wasm-opt {input} -o {output} {options}'");
        let cmd_output = Command::new("wasm-opt")
            .args([input, "-o".to_owned(), output.clone()])
            .args(shell_words::split(&options).expect("Failed to parse options"))
            .output()
            .expect("failed to execute command");

        // Write output regardless of success/failure
        io::stdout()
            .write_all(&cmd_output.stdout)
            .expect("failed to write to stdout");
        io::stderr()
            .write_all(&cmd_output.stderr)
            .expect("failed to write to stderr");

        // Check if the command was successful
        if !cmd_output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("wasm-opt failed with status: {}", cmd_output.status),
            ));
        }

        // Log output file size and reduction percentage
        let output_size = fs::metadata(&output).map(|m| m.len()).unwrap_or_else(|_| 0);
        let size_reduction = if input_size > 0 {
            ((input_size as f64 - output_size as f64) / input_size as f64 * 100.0).round()
        } else {
            0.0
        };
        println!(
            "Output file size: {} bytes ({}% reduction)",
            output_size, size_reduction
        );

        if !optimize_all {
            break;
        }
    }

    Ok(())
}
