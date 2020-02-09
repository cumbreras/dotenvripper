extern crate walkdir;

use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn main() -> std::io::Result<()> {
    fs::create_dir("../dotenv-backup")?;

    for entry in WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        if f_name.ends_with(".env") {
            let new_path = Path::new(entry.path());
            println!("{}", new_path.display());

            println!("{:?}", entry.path());
            fs::copy(
                entry.path(),
                format!(
                    "../dotenv-backup/{}",
                    new_path.parent().unwrap().to_str().unwrap()
                ),
            )?;
        }
    }

    Ok(())
}
