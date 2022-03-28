use std::fmt;
use std::fs::File;
use std::io::SeekFrom::Start;
use std::io::{Read, Seek};
use std::path::Path;

pub struct FileReader<'a> {
    filepath: &'a Path,
    file: File,
    size: u64,
}

impl<'a> FileReader<'a> {
    pub fn new(filepath: &'a Path) -> Self {
        let file =
            File::open(filepath).expect(format!("Failed to open file {:?}", filepath).as_str());
        let size = file
            .metadata()
            .expect(format!("Failed to get metadata for {:?}", filepath).as_str())
            .len();
        Self {
            filepath,
            file,
            size,
        }
    }
}

impl<'a> fmt::Debug for FileReader<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "{}", self.filepath.to_string_lossy()).unwrap();
        let mut my_file = &self.file;
        my_file.seek(Start(0)).unwrap();
        let buffer: &mut [u8] = &mut [0; 1024];
        loop {
            let bytes_read = my_file
                .read(buffer)
                .expect(format!("Read of {} failed.", self.filepath.to_string_lossy()).as_str());
            if bytes_read > 0 {
                write!(f, "{}", String::from_utf8_lossy(&buffer[0..bytes_read])).unwrap();
            } else {
                break;
            }
        }
        Ok(())
    }
}

impl<'a> PartialEq for FileReader<'_> {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }
        let mut my_file = &self.file;
        let mut other_file = &other.file;
        my_file.seek(Start(0)).unwrap();
        other_file.seek(Start(0)).unwrap();
        let my_buffer: &mut [u8] = &mut [0; 1024];
        let other_buffer: &mut [u8] = &mut [0; 1024];
        loop {
            let my_bytes_read = my_file
                .read(my_buffer)
                .expect(format!("Read of {} failed.", self.filepath.to_string_lossy()).as_str());
            let other_bytes_read = other_file
                .read(other_buffer)
                .expect(format!("Read of {} failed.", other.filepath.to_string_lossy()).as_str());
            if my_bytes_read == other_bytes_read {
                if my_buffer != other_buffer {
                    return false;
                }
            } else {
                return false;
            }
            if my_bytes_read == 0 {
                break;
            }
        }
        true
    }
}
