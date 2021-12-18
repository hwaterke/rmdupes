use std::collections::HashMap;
use std::path::PathBuf;
use structopt::StructOpt;
use walkdir::WalkDir;

/// Find and remove duplicate files.
#[derive(StructOpt, Debug)]
struct Cli {
    /// Activate verbose mode
    #[structopt(short, long)]
    verbose: bool,

    /// Activate dry-run mode
    #[structopt(short, long)]
    dry_run: bool,

    /// List of directories to search
    #[structopt(parse(from_os_str), required = true)]
    directories: Vec<PathBuf>,

    /// List of reference directories to search (no files will ever be deleted there)
    #[structopt(
        short = "r",
        long = "reference",
        parse(from_os_str),
        number_of_values = 1
    )]
    reference_directories: Vec<PathBuf>,
}

fn main() {
    let args = Cli::from_args();

    println!("{:?}", args);

    let mut files_by_size: HashMap<u64, Vec<walkdir::DirEntry>> = HashMap::new();
    let mut files_by_hash: HashMap<u64, Vec<walkdir::DirEntry>> = HashMap::new();

    for dir in args.directories {
        for walk_entry in WalkDir::new(dir) {
            match walk_entry {
                Ok(entry) => {
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_file() {
                            println!(
                                "{} - {} - {}",
                                entry.path().display(),
                                metadata.is_dir(),
                                metadata.len()
                            );
                            let file_size = metadata.len();

                            files_by_size.entry(file_size).or_insert(Vec::new());
                            files_by_size.get_mut(&file_size).unwrap().push(entry);
                        }
                    }
                }
                Err(e) => println!("error processing entry: {:?}", e),
            }
        }
    }

    for (key, val) in files_by_size.iter() {
        if val.len() > 1 {
            println!(
                "Finding duplicates in group of {} files of size {}",
                val.len(),
                key
            );

            for e in val {
                println!("{:?}", e)
            }
        }
    }

    // Go through all the files.
    // Group them by size
    // Then for each group, group them by similar hash
    // Then for each group, if length > 2 then deletes the ones not under the reserved directories
}
