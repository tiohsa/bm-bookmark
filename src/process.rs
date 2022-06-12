use crate::bookmark::Manager;
use anyhow::{bail, Error};
use std::path::Path;

pub struct Process {}

impl Process {
    fn is_valid_path(path: &Path) -> bool {
        path.exists() && path.is_dir()
    }

    pub fn change_directory(name: &str) -> Result<(), Error> {
        let manager = Manager::new();
        let path = manager.get_bookmark(name)?;
        let directory_path = Path::new(&path);
        if Process::is_valid_path(&directory_path) {
            print!("{}", &directory_path.to_str().unwrap());
            Ok(())
        } else {
            bail!("{} ({}) is invalid directory path", name, path)
        }
    }

    pub fn show_list() -> Result<(), Error> {
        let manager = Manager::new();
        let bookmarks = manager.read_bookmarks()?;
        if let Some(max) = bookmarks.iter().map(|v| v.name.len()).max() {
            for bookmark in bookmarks {
                println!("{:<width$} {}", &bookmark.name, &bookmark.path, width = max);
            }
        }
        Ok(())
    }

    pub fn add_bookmark(name: &str, path: &str) -> Result<(), Error> {
        let directory_path = Path::new(&path);
        if Process::is_valid_path(&directory_path) {
            let manager = Manager::new();
            manager.add_bookmark(name, path)?;
            Ok(())
        } else {
            bail!("{} ({}) is invalid directory path", name, path)
        }
    }

    pub fn remove_bookmark(name: &str) -> Result<(), Error> {
        let manager = Manager::new();
        manager.remove_bookmark(name)?;
        Ok(())
    }
}
