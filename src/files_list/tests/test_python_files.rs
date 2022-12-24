use std::fs::{create_dir, remove_dir, remove_file, write, File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::path::Component::Normal;
use std::path::{Component, Path, PathBuf};

use crate::error::Error;

pub struct PythonFiles<'a> {
    input_path: &'a Path,
    expected_path: &'a Path,
}

impl<'a> PythonFiles<'a> {
    pub fn new(input_path: &'a Path, expected_path: &'a Path) -> Self {
        PythonFiles {
            input_path,
            expected_path,
        }
    }

    pub fn touch_files_and_update_files_lists(&self) -> Result<(), Error> {
        // For BufReader::lines
        use std::io::BufRead;
        // for OsStr.as_bytes
        use std::os::unix::ffi::OsStrExt;

        let files: Vec<String>;
        let temp_root = match Path::new(self.expected_path).parent() {
            Some(temp_root_path) => temp_root_path.to_path_buf(),
            None => PathBuf::from("/"),
        };
        // Read the files to be touched
        {
            let file = File::open(self.expected_path)?;
            let reader = BufReader::new(&file);
            files = reader.lines().map(|line| line.unwrap()).collect();
        }
        // Touch the files
        {
            for file in files.iter() {
                if file.starts_with('#') {
                    continue;
                }
                let file_path = temp_root.join(file);
                let path_components = file_path.components();
                let mut current: Option<Component> = None;
                let mut next: Option<Component> = None;
                let mut path_from_parts = PathBuf::from("/");
                for component in path_components {
                    if next.is_some() {
                        current = next;
                    }
                    next = Some(component);
                    if let Some(Normal(path_part)) = current {
                        path_from_parts.push(path_part);
                        if path_from_parts.is_file() {
                            remove_file(&path_from_parts)?;
                        }
                        if !path_from_parts.is_dir() {
                            create_dir(&path_from_parts)?;
                        }
                    } else {
                        // ???
                    }
                    /*
                    if current.is_some() && next.is_some() {
                        create_dir(current)
                    }
                    */
                }
                if let Some(Normal(path_part)) = next {
                    path_from_parts.push(path_part);
                    if path_from_parts.is_dir() {
                        remove_dir(&path_from_parts)?;
                    }
                    if !path_from_parts.is_file() {
                        write(path_from_parts, "")?;
                    }
                }

                // Write this path_from_parts to the
                /*
                if let Some(mut parent) = file_path.parent() {
                    while !parent.exists() && parent.starts_with(&temp_root) {
                        if let Some(mut next_parent) = parent.parent() {
                            parent = next_parent;
                        }
                    }
                    if parent.is_file() && parent.starts_with(&temp_root) {
                        // replace file with directory
                        remove_file(parent).expect(format!("Failed to delete file {:?}", parent).as_str());
                        create_dir(parent).expect(format!("Failed to create directory {:?}", parent).as_str());
                    }
                }
                */
            }
        }
        // Update the expected files list
        {
            let file = OpenOptions::new().write(true).open(self.expected_path)?;
            let mut writer = BufWriter::new(&file);
            let mut write_count: usize;
            for file_line in files {
                if !file_line.starts_with('#') {
                    let prefix = temp_root.as_os_str().as_bytes();
                    write_count = writer
                        .write(&prefix[1..])
                        .expect("Failed to write file prefix in test");
                    if write_count < prefix.len() - 1 {
                        panic!("Failed to write file prefix in test");
                    }
                    // Write a slash
                    write_count = writer
                        .write(&[0x2f])
                        .expect("Failed to write slash in test");
                    if write_count < 1 {
                        panic!("Failed to write slash in test");
                    }
                }
                let bytes = file_line.as_bytes();
                write_count = writer
                    .write(&bytes)
                    .expect("Failed to write all of the bytes in test");
                if write_count < bytes.len() {
                    panic!("Failed to write all of the bytes in test");
                }
                write_count = writer
                    .write(&[0x0a])
                    .expect("Failed to write newline in test");
                if write_count < 1 {
                    panic!("Failed to write newline in test");
                }
            }
            writer.flush().unwrap();
        }
        let input_files: Vec<String>;
        // Read and update the input files list
        {
            let file = File::open(self.input_path)?;
            let reader = BufReader::new(&file);
            input_files = reader.lines().map(|line| line.unwrap()).collect();
        }
        // Update the input files list
        {
            let file = OpenOptions::new().write(true).open(self.input_path)?;
            let mut writer = BufWriter::new(&file);
            let mut write_count: usize;
            for file_line in input_files {
                if !file_line.starts_with('#') {
                    let prefix = temp_root.as_os_str().as_bytes();
                    write_count = writer
                        .write(&prefix[1..])
                        .expect("Failed to write file prefix in test");
                    if write_count < prefix.len() - 1 {
                        panic!("Failed to write file prefix in test");
                    }
                    // Write a slash
                    write_count = writer
                        .write(&[0x2f])
                        .expect("Failed to write slash in test");
                    if write_count < 1 {
                        panic!("Failed to write slash in test");
                    }
                }
                let bytes = file_line.as_bytes();
                write_count = writer
                    .write(&bytes)
                    .expect("Failed to write all of the bytes in test");
                if write_count < bytes.len() {
                    panic!("Failed to write all of the bytes in test");
                }
                write_count = writer
                    .write(&[0x0a])
                    .expect("Failed to write newline in test");
                if write_count < 1 {
                    panic!("Failed to write newline in test");
                }
            }
            writer.flush().unwrap();
        }
        Ok(())
    }
}
