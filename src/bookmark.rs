extern crate serde;
use anyhow::{bail, Error};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

const BOOKMARK_PATH: &str = ".cache/bm-bookmark";

pub struct Manager {
    file_path: String,
}

impl Manager {
    pub fn new() -> Self {
        let file_path = format!("{}/{}", env::var("HOME").unwrap(), BOOKMARK_PATH);
        Self { file_path }
    }

    pub fn get_bookmark(&self, name: &str) -> Result<String, Error> {
        if let Some(bookmark) = self.read_bookmarks()?.iter().find(|v| v.name == name) {
            Ok(bookmark.path.clone())
        } else {
            bail!("{} is not found.", name)
        }
    }

    pub fn add_bookmark(&self, name: &str, path: &str) -> Result<(), Error> {
        let directory_path = Path::new(path);
        let abs_path = directory_path.canonicalize().unwrap();
        let bookmark = Bookmark::new(name, abs_path.to_str().unwrap());
        let mut bookmarks = self.read_bookmarks()?;
        if let Some(same_bookmark) = bookmarks.iter().find(|v| v.name == bookmark.name) {
            bail!(
                "{} is already registered. ({} = {})",
                same_bookmark.name,
                same_bookmark.name,
                same_bookmark.path
            )
        } else {
            bookmarks.push(bookmark);
            self.write_bookmarks(&bookmarks)?;
            Ok(())
        }
    }

    pub fn remove_bookmark(&self, name: &str) -> Result<(), Error> {
        let mut bookmarks = self.read_bookmarks()?;
        if let Some(index) = bookmarks.iter().position(|v| v.name == name) {
            bookmarks.remove(index);
            self.write_bookmarks(&bookmarks)?;
            Ok(())
        } else {
            bail!("{} is not found.", name)
        }
    }

    pub fn read_bookmarks(&self) -> Result<Vec<Bookmark>, Error> {
        if std::path::Path::new(&self.file_path).exists() {
            let f = File::open(&self.file_path)?;
            let reader = BufReader::new(f);
            let bookmarks: Vec<Bookmark> = serde_json::from_reader(reader)?;
            Ok(bookmarks)
        } else {
            Ok(vec![])
        }
    }

    fn write_bookmarks(&self, bookmarks: &Vec<Bookmark>) -> Result<(), Error> {
        let file_path = Path::new(&self.file_path);
        let parent = file_path.parent().unwrap();
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
        let f = File::create(&self.file_path)?;
        let writer = BufWriter::new(f);
        serde_json::to_writer(writer, bookmarks)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Bookmark {
    pub name: String,
    pub path: String,
}

impl Bookmark {
    pub fn new(name: &str, path: &str) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
        }
    }
}

mod tests {
    use super::*;

    struct TestDataManager {
        file_path: String,
    }

    impl TestDataManager {
        #[allow(dead_code)]
        fn new(file_path: &str) -> Self {
            Self {
                file_path: file_path.to_string(),
            }
        }

        #[allow(dead_code)]
        fn read(&self) -> Result<String, Error> {
            let f = File::open(&self.file_path).unwrap();
            let mut reader = BufReader::new(f);
            let mut data = String::new();
            reader.read_to_string(&mut data).unwrap();
            Ok(data)
        }

        #[allow(dead_code)]
        fn write(&self, data: &str) -> Result<(), Error> {
            let f = File::create(&self.file_path).unwrap();
            let mut writer = BufWriter::new(f);
            writer.write(data.as_bytes())?;
            Ok(())
        }
    }

    impl Drop for TestDataManager {
        fn drop(&mut self) {
            if std::path::Path::new(&self.file_path).exists() {
                std::fs::remove_file(&self.file_path).unwrap();
            }
        }
    }

    #[test]
    fn test_read_bookmarks_not_found_file() {
        let test_bookmarks_path = "/tmp/test_read_bookmarks_not_found.json";

        let manager = Manager {
            file_path: test_bookmarks_path.to_string(),
        };
        let bookmarks = manager.read_bookmarks().unwrap();
        assert!(bookmarks.is_empty());
    }

    #[test]
    fn test_read_bookmarks() {
        let test_json = r#"[{"name":"test1","path":"/tmp/test1"},{"name":"test2","path":"/tmp/test2"},{"name":"test3","path":"/tmp/test3"}]"#;

        let test_bookmarks_path = "/tmp/test_read_bookmarks.json";
        let test_data_manager = TestDataManager::new(test_bookmarks_path);
        test_data_manager.write(&test_json).unwrap();

        let manager = Manager {
            file_path: test_bookmarks_path.to_string(),
        };
        let bookmarks = manager.read_bookmarks().unwrap();
        assert_eq!(bookmarks[0].name, "test1");
        assert_eq!(bookmarks[0].path, "/tmp/test1");
        assert_eq!(bookmarks[1].name, "test2");
        assert_eq!(bookmarks[1].path, "/tmp/test2");
        assert_eq!(bookmarks[2].name, "test3");
        assert_eq!(bookmarks[2].path, "/tmp/test3");
    }

    #[test]
    fn test_write_bookmarks() {
        let test_json = r#"[{"name":"test1","path":"/tmp/test1"},{"name":"test2","path":"/tmp/test2"},{"name":"test3","path":"/tmp/test3"}]"#;

        let test_bookmarks_path = "/tmp/test_write_bookmarks.json";
        let test_data_manager = TestDataManager::new(test_bookmarks_path);

        let manager = Manager {
            file_path: test_bookmarks_path.to_string(),
        };
        let bookmarks = vec![
            Bookmark::new("test1", "/tmp/test1"),
            Bookmark::new("test2", "/tmp/test2"),
            Bookmark::new("test3", "/tmp/test3"),
        ];
        manager.write_bookmarks(&bookmarks).unwrap();

        let json = test_data_manager.read().unwrap();
        assert_eq!(&json, &test_json);
    }

    #[test]
    fn test_add_bookmark_to_empty_file() {
        let test_bookmarks_path = "/tmp/test_add_bookmark_to_empty_file.json";
        let test_data_manager = TestDataManager::new(test_bookmarks_path);

        let name = "test4";
        let path = "/tmp";
        let manager = Manager {
            file_path: test_bookmarks_path.to_string(),
        };
        manager.add_bookmark(name, path).unwrap();

        let json = test_data_manager.read().unwrap();
        let bookmarks: Vec<Bookmark> = serde_json::from_str(&json).unwrap();
        assert_eq!(&bookmarks[0].name, &name);
        assert_eq!(&bookmarks[0].path, &path);
    }

    #[test]
    fn test_add_bookmark_to_not_empty_file() {
        let test_json = r#"[{"name":"test1","path":"/tmp/test1"},{"name":"test2","path":"/tmp/test2"},{"name":"test3","path":"/tmp/test3"}]"#;

        let test_bookmarks_path = "/tmp/test_add_bookmark_to_not_empty_file.json";
        let test_data_manager = TestDataManager::new(test_bookmarks_path);
        test_data_manager.write(&test_json).unwrap();

        let name = "test4";
        let path = "/tmp";
        let manager = Manager {
            file_path: test_bookmarks_path.to_string(),
        };
        manager.add_bookmark(name, path).unwrap();

        let json = test_data_manager.read().unwrap();
        let bookmarks: Vec<Bookmark> = serde_json::from_str(&json).unwrap();
        assert_eq!(&bookmarks[3].name, &name);
        assert_eq!(&bookmarks[3].path, &path);
    }

    #[test]
    #[should_panic(expected = "test3 is already registered. (test3 = /tmp/test3)")]
    fn test_add_bookmark_already_registered() {
        let test_json = r#"[{"name":"test1","path":"/tmp/test1"},{"name":"test2","path":"/tmp/test2"},{"name":"test3","path":"/tmp/test3"}]"#;

        let test_bookmarks_path = "/tmp/test_add_bookmark_already_registered.json";
        let test_data_manager = TestDataManager::new(test_bookmarks_path);
        test_data_manager.write(&test_json).unwrap();

        let name = "test3";
        let path = "/tmp";
        let manager = Manager {
            file_path: test_bookmarks_path.to_string(),
        };
        manager.add_bookmark(name, path).unwrap();
    }

    #[test]
    fn test_remove_bookmark() {
        let test_json = r#"[{"name":"test1","path":"/tmp/test1"},{"name":"test2","path":"/tmp/test2"},{"name":"test3","path":"/tmp/test3"}]"#;

        let test_bookmarks_path = "/tmp/test_remove_bookmark.json";
        let test_data_manager = TestDataManager::new(test_bookmarks_path);
        test_data_manager.write(&test_json).unwrap();

        let name = "test2";
        let manager = Manager {
            file_path: test_bookmarks_path.to_string(),
        };
        manager.remove_bookmark(name).unwrap();
        let json = test_data_manager.read().unwrap();
        let expected =
            r#"[{"name":"test1","path":"/tmp/test1"},{"name":"test3","path":"/tmp/test3"}]"#;
        assert_eq!(&json, &expected);
    }

    #[test]
    #[should_panic(expected = "test4 is not found.")]
    fn test_remove_bookmark_not_found() {
        let test_json = r#"[{"name":"test1","path":"/tmp/test1"},{"name":"test2","path":"/tmp/test2"},{"name":"test3","path":"/tmp/test3"}]"#;

        let test_bookmarks_path = "/tmp/test_remove_bookmark_not_found.json";
        let test_data_manager = TestDataManager::new(test_bookmarks_path);
        test_data_manager.write(&test_json).unwrap();

        let name = "test4";
        let manager = Manager {
            file_path: test_bookmarks_path.to_string(),
        };
        manager.remove_bookmark(name).unwrap();
    }

    #[test]
    fn test_get_bookmark() {
        let test_json = r#"[{"name":"test1","path":"/tmp/test1"},{"name":"test2","path":"/tmp/test2"},{"name":"test3","path":"/tmp/test3"}]"#;

        let test_bookmarks_path = "/tmp/test_get_bookmark.json";
        let test_data_manager = TestDataManager::new(test_bookmarks_path);
        test_data_manager.write(&test_json).unwrap();

        let name = "test3";
        let manager = Manager {
            file_path: test_bookmarks_path.to_string(),
        };
        let bookmark = manager.get_bookmark(name).unwrap();
        assert_eq!(bookmark, "/tmp/test3");
    }

    #[test]
    #[should_panic(expected = "test4 is not found.")]
    fn test_get_bookmark_is_not_found() {
        let test_json = r#"[{"name":"test1","path":"/tmp/test1"},{"name":"test2","path":"/tmp/test2"},{"name":"test3","path":"/tmp/test3"}]"#;

        let test_bookmarks_path = "/tmp/test_get_bookmark_is_not_found.json";
        let test_data_manager = TestDataManager::new(test_bookmarks_path);
        test_data_manager.write(&test_json).unwrap();

        let name = "test4";
        let manager = Manager {
            file_path: test_bookmarks_path.to_string(),
        };
        manager.get_bookmark(name).unwrap();
    }
}
