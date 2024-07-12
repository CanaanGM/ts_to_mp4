use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use walkdir::WalkDir;

fn convert_ts_to_mp4(input_path: &Path) {
    let output_path = input_path.with_extension("mp4");

    println!("Input path: {:?}", input_path);
    println!("Output path: {:?}", output_path);

    let input_path_str = format!("{}", input_path.to_str().unwrap().replace("\\", "/"));
    let output_path_str = format!("{}", output_path.to_str().unwrap().replace("\\", "/"));

    let status = Command::new("ffmpeg")
        .arg("-hwaccel")
        .arg("cuda")
        .arg("-i")
        .arg(&input_path_str)
        .arg("-codec")
        .arg("copy")
        .arg(&output_path_str)
        .status()
        .expect("Failed to execute ffmpeg");
        // .arg("-i")
        // .arg(&input_path_str)
        // .arg("-c:v")
        // .arg("libx264")
        // .arg(&output_path_str)
        // .status()
        // .expect("Failed to execute ffmpeg");


    if status.success() {
        println!("Successfully converted: {}", input_path.display());
        if let Err(err) = fs::remove_file(input_path) {
            eprintln!("Error removing original .ts file: {}", err);
        }
    } else {
        eprintln!("Conversion failed for: {}", input_path.display());
    }
}

fn convert_in_directory(directory: &Path, recursive: bool) {
    let walker = WalkDir::new(directory)
        .follow_links(true)
        .min_depth(1)
        .max_depth(if recursive { std::usize::MAX } else { 1 })
        .into_iter();

    for entry in walker.filter_map(|e| e.ok()) {
        let path = entry.path();
        let extension = path.extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_default();

        if entry.file_type().is_file() && extension == "ts" {
            convert_ts_to_mp4(path);
        } else {
            println!("Skipped: {}", path.display());
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <directory> [-r]", args[0]);
        std::process::exit(1);
    }

    let directory = &args[1];
    let recursive = args.contains(&"-r".to_string());

    let directory_path = Path::new(directory);
    if !directory_path.exists() || !directory_path.is_dir() {
        eprintln!("Error: {} is not a valid directory", directory);
        std::process::exit(1);
    }

    convert_in_directory(directory_path, recursive);
}
