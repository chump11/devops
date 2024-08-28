use std::{env, fs, io::{BufReader, Read}, path::Path};

fn walk_path(path: &Path) {
    for entry in fs::read_dir(path).unwrap() {
        let unwrapped_entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                println!("Error: {}", err);
                continue;
            }
        };
        println!("Entry path: {}", unwrapped_entry.path().display());
        let path = unwrapped_entry.path();
        if path.is_dir() {
            walk_path(&path);
        } else {
            let file = fs::File::open(&path).unwrap();
            let mut buffer = [0; 1024];
            let mut reader = BufReader::new(file);
            let bytes_read = reader.read(&mut buffer).unwrap();
            if(bytes_read > 0) {
                println!("First 10 bytes: {:?}", &buffer[0..bytes_read]);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <path>", args[0]);
        return;
    }

    let path = Path::new(&args[1]);

    if !path.exists() {
        println!("Path {} does not exist", path.display());
        return;
    }

    walk_path(path);
}