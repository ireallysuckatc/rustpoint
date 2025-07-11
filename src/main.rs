use serde_json::{Value, from_reader, to_writer_pretty};
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Write};
use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    mode: String,
}

fn main() {
    let args = Args::parse();

    match args.mode.as_str() {
        "add" => add_path(),
        "use" => use_path(),
        "delete" => delete_path(),
        _ => {
            eprintln!("Invalid mode. Use 'add','use', 'delete'.");
            std::process::exit(1);
        }
    }  
}

fn add_path() {
    let mut path = String::new();

    println!("Please enter the path you want to store.");
    io::stdin().read_line(&mut path).expect("Failed to read line");

    let path = path.trim().to_string();


    if !Path::new(&path).exists() {
        eprintln!("The provided path does not exist.");
        return;
    }

    let mut paths: Vec<String> = if let Ok(file) = File::open("path.json") {
        let reader = BufReader::new(file);
        match from_reader(reader) {
            Ok(Value::Array(arr)) => arr.into_iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect(),
            _ => Vec::new(),
        }
    } else {
        Vec::new()
    };

    if paths.contains(&path) {
        println!("Path already exists in path.json.");
        return;
    }

    paths.push(path);

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("path.json")
        .expect("Unable to open file");

    let mut writer = BufWriter::new(file);
    to_writer_pretty(&mut writer, &paths).expect("Unable to write data");
    writer.flush().expect("Unable to flush writer");

    println!("Path added to path.json successfully.");
}

fn use_path() {
    let paths: Vec<String> = if let Ok(file) = File::open("path.json") {
        let reader = BufReader::new(file);
        match from_reader(reader) {
            Ok(Value::Array(arr)) => arr.into_iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect(),
            _ => Vec::new(),
        }
    } else {
        Vec::new()
    };

    if paths.is_empty() {
        eprintln!("No paths found. Add a path first.");
        std::process::exit(1);
    }

    eprintln!("Select a path to go to:");
    for (i, path) in paths.iter().enumerate() {
        eprintln!("{}. {}", i + 1, path);
    }

    eprintln!("Enter the number of the path you want to select:");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let selection: usize = match input.trim().parse() {
        Ok(num) if num > 0 && num <= paths.len() => num,
        _ => {
            eprintln!("Invalid selection.");
            std::process::exit(1);
        }
    };

    println!("{}", paths[selection - 1]);  
}

fn delete_path() {
    let mut path = String::new();

    println!("Please enter the path you want to delete.");
    io::stdin().read_line(&mut path).expect("Failed to read line");
    let path = path.trim().to_string();

    if path.is_empty() {
        eprintln!("No path provided.");
        return;
    }

    let mut paths: Vec<String> = if let Ok(file) = File::open("path.json") {
        let reader = BufReader::new(file);
        match from_reader(reader) {
            Ok(Value::Array(arr)) => arr.into_iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect(),
            _ => Vec::new(),
        }
    } else {
        Vec::new()
    };

    let initial_len = paths.len();
    paths.retain(|p| p != &path);

    if paths.len() == initial_len {
        eprintln!("Path not found in path.json.");
        return;
    }

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("path.json")
        .expect("Unable to open file");

    let mut writer = BufWriter::new(file);

    to_writer_pretty(&mut writer, &paths).expect("Unable to write data");
    writer.flush().expect("Unable to flush writer");

    println!("Path deleted from path.json successfully.");
}