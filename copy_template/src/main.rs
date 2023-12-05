use std::fs;
use std::io;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use clap::Parser;

/// Create a copy of the template day
#[derive(Parser, Debug)]
struct Args {
    /// Template source folder
    #[arg(short, long, default_value = "../day_template/")]
    source: String,

    /// Destination folder (takes precedence over --year)
    #[arg(short, long)] //, default_value = "../year_2015/"
    target: Option<String>,

    /// Year of the problem
    #[arg(short, long, default_value = "2015")]
    year: u16,    

    /// Day of the problem, omit to automatically find
    #[arg(short, long, default_value = "0")]
    day: u8,
}

fn find_latest_day(target: &Path) -> u8 {
    fs::read_dir(target)
        .ok()
        .map(|entries| {
            entries
                .flatten()
                .filter_map(|entry| {
                    let name = entry.file_name().to_string_lossy().to_string();
                    name.strip_prefix("day_")
                        .and_then(|num_str| num_str.parse::<u8>().ok())
                })
                .max()
                .unwrap_or(0)
        })
        .unwrap_or(0)
}

/// Recursively copy a directory to another directory
fn copy_directory(source: &Path, target: &Path, day: &u8, add_day_suffix: bool) -> Result<PathBuf, std::io::Error> {
    if source.is_dir() {
        let source_folder = source.file_name().unwrap();
        let target_folder: PathBuf;

        if add_day_suffix {    
            target_folder = target.join(format!(
                "{}_{:02}",
                &source_folder.to_string_lossy().split('_').next().unwrap(),
                day
            ));
        } else {
            target_folder = target.join(source_folder);
        }

        if target_folder.exists() {
            panic!("Destination folder already exists. Aborting to prevent overwriting.");
        }

        fs::create_dir_all(&target_folder)?;

        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let source_path = entry.path();
            let file_name = source_path.file_name().unwrap();
            let target_path = target_folder.join(file_name);

            if entry.file_type()?.is_dir() {
                copy_directory(&source_path, &target_folder, &day, false)?;
            } else {
                fs::copy(&source_path, &target_path)?;
            }
        }

        if add_day_suffix {
            println!("Copy {source_folder:?} to {target_folder:?}");
        }

        Ok(target_folder)
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Source path is not a directory",
        ))
    }

}

/// Update files after copying the directory
fn update_files(root_folder: &PathBuf, day: &u8) -> io::Result<()> {
    let cargo_toml_path = root_folder.join("Cargo.toml");

    // Update Cargo.toml
    update_file_content(&cargo_toml_path, &"day_template", &format!("day_{:02}", day))?;
    update_file_content(&cargo_toml_path, &"../utilities", &"../../../utilities")?;

    Ok(())
}

/// Update file content by replacing old_content with new_content
fn update_file_content(file_path: &Path, old_content: &str, new_content: &str) -> io::Result<()> {
    let mut file = fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let updated_contents = contents.replace(old_content, new_content);
    let mut updated_file = fs::File::create(file_path)?;
    updated_file.write_all(updated_contents.as_bytes())?;

    println!("Update {old_content:?} to {new_content:?} in {file_path:?}");

    Ok(())
}

fn main() {
    let args = Args::parse();

    let source = PathBuf::from(args.source);
    let target = PathBuf::from(match args.target {
        Some(t) => t,
        None => format!("../solutions/year_{}/", args.year),
    });
    let day;

    if args.day == 0 {
        day = find_latest_day(&target) + 1;
    } else {
        day = args.day;
    }

    if let Ok(root_folder) = copy_directory(&source, &target, &day, true) {
        if let Err(err) = update_files(&root_folder, &day) {
            eprintln!("Error occurred while updating files: {:?}", err);
            return;
        }
    } else {
        eprintln!("Error occurred while copying directory");
        return;
    }

    println!("Files copied and updated successfully!");
}