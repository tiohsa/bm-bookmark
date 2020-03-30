use crate::bookmark::Manager;
use failure::{format_err, Error};
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
            Err(format_err!("{} ({}) is invalid directory path", name, path))
        }
    }

    pub fn show_list() -> Result<(), Error> {
        let manager = Manager::new();
        let bookmarks = manager.read_bookmarks()?;
        for bookmark in bookmarks {
            println!("{}={}", &bookmark.name, &bookmark.path);
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
            Err(format_err!("{} ({}) is invalid directory path", name, path))
        }
    }

    pub fn remove_bookmark(name: &str) -> Result<(), Error> {
        let manager = Manager::new();
        manager.remove_bookmark(name)?;
        Ok(())
    }
}
