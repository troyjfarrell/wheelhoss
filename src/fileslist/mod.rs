//! FilesList
//!
//! `fileslist` helps build and maintain the sandstorm-files.list file

use std::collections::BTreeSet;
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::path::{self, Path, PathBuf};

use crate::error::Error;

const DOT_CPYTHON_DASH: &str = ".cpython-";
const PYC_EXTENSION: &str = ".pyc";
const PYCACHE_DIRECTORY: &str = "__pycache__";

pub struct FilesList {
    filepath: PathBuf,
    file: Option<File>,
    headers: Vec<String>,
    listed_files: BTreeSet<String>,
}

impl FilesList {
    /// Constructs a new `FilesList` at path `filepath`.
    pub fn new(filepath: &Path) -> Self {
        Self {
            filepath: filepath.to_path_buf(),
            file: None,
            headers: Vec::new(),
            listed_files: BTreeSet::new(),
        }
    }

    /// Adds Python source files to the fileslist file.
    ///
    /// `include_python_source_files` reads the fileslist file and identifies Python bytecode
    /// files.  If the corresponding Python source files are present on the filesystem and are not
    /// listed in the fileslist file, this function adds them to the fileslist file.
    ///
    /// Leading comments in the fileslist file will be preserved.  All other comments will be lost.
    pub fn include_python_source_files(&mut self) -> Result<BTreeSet<String>, Error> {
        self.ingest_file()?;
        let added_sources = self.add_missing_python_source_files()?;
        self.write_file()?;
        Ok(added_sources)
    }

    fn add_missing_python_source_files(&mut self) -> Result<BTreeSet<String>, Error> {
        let mut added_sources: BTreeSet<String> = BTreeSet::new();

        for line in self.listed_files.iter() {
            if line.contains(PYCACHE_DIRECTORY) && line.ends_with(PYC_EXTENSION) {
                let possible_sources = FilesList::suggest_python_sources_for(line.as_str())?;
                for possible_source in possible_sources {
                    let mut python_source = PathBuf::new();
                    python_source.push("/");
                    python_source.push(possible_source.clone());
                    if python_source.is_file() {
                        added_sources.insert(possible_source);
                    }
                }
            }
        }
        for line in added_sources.iter() {
            self.listed_files.insert((&line[1..]).to_string());
        }

        Ok(added_sources)
    }

    fn ingest_file(&mut self) -> Result<(), Error> {
        use std::io::BufRead;

        let mut in_headers: bool = true;

        self.open_and_lock_file()?;
        let mut file = self.file.take().expect("Unable to read the FilesList file");
        {
            let reader = BufReader::new(&mut file);
            for reader_line in reader.lines() {
                let line = reader_line?;
                if in_headers {
                    if line.starts_with('#') {
                        self.headers.push(line);
                    } else {
                        in_headers = false;
                        self.listed_files.insert(line);
                    }
                } else {
                    self.listed_files.insert(line);
                }
            }
        }
        self.file = Some(file);
        Ok(())
    }
    fn open_and_lock_file(&mut self) -> Result<(), Error> {
        use fs3::FileExt;

        if self.file.is_none() {
            let file = match OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(self.filepath.as_path())
            {
                Ok(file) => file,
                Err(err) => {
                    return Err(Error::FailedToOpenFile(
                        self.filepath.to_string_lossy().to_string(),
                        Some(err),
                    ));
                }
            };
            file.lock_exclusive()?;
            self.file = Some(file);
        }
        Ok(())
    }

    fn suggest_python_sources_for(pyc_path: &str) -> Result<Vec<String>, Error> {
        let mut result = Vec::<String>::new();
        let mut path = PathBuf::new();
        let mut suggestion = PathBuf::new();
        path.push(&path::MAIN_SEPARATOR.to_string());
        path.push(pyc_path);
        for path_part in path.iter() {
            if let Some(part) = path_part.to_str() {
                if part.contains(PYCACHE_DIRECTORY) {
                    continue;
                } else if part.contains(DOT_CPYTHON_DASH) && part.ends_with(PYC_EXTENSION) {
                    // This is going to need more work when this is used with Pypy or other
                    // interpreters.
                    if let Some((left, _)) = part.split_once(DOT_CPYTHON_DASH) {
                        suggestion.push(format!("{}.py", left));
                    } else {
                        return Err(Error::FailedToSplitFilename(part.to_string()));
                    }
                } else {
                    suggestion.push(part);
                }
            } else {
                return Err(Error::UnableToProcessNonUtf8Path(
                    path_part.to_string_lossy().into_owned(),
                ));
            }
        }
        result.push(suggestion.to_str().unwrap().to_string());
        Ok(result)
    }

    fn write_file(&mut self) -> Result<(), Error> {
        use std::io::SeekFrom::Start;
        use std::io::{Seek, Write};

        let mut file = self.file.take().expect("Unable to read the FilesList file");
        let mut write_count: usize;

        file.seek(Start(0))?;
        for header_line in self.headers.iter() {
            write_count = file.write(header_line.as_bytes())?;
            if write_count < header_line.len() {
                return Err(Error::FilesListWriteIncomplete(
                    self.filepath.to_string_lossy().to_string(),
                ));
            }
            write_count = file.write(&[0x0a])?;
            if write_count < 1 {
                return Err(Error::FilesListWriteIncomplete(
                    self.filepath.to_string_lossy().to_string(),
                ));
            }
        }
        for file_line in self.listed_files.iter() {
            write_count = file.write(file_line.as_bytes())?;
            if write_count < file_line.len() {
                return Err(Error::FilesListWriteIncomplete(
                    self.filepath.to_string_lossy().to_string(),
                ));
            }
            write_count = file.write(&[0x0a])?;
            if write_count < 1 {
                return Err(Error::FilesListWriteIncomplete(
                    self.filepath.to_string_lossy().to_string(),
                ));
            }
        }
        file.flush()?;
        self.file = Some(file);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    mod test_file_reader;
    mod test_fixture;
    mod test_python_files;

    use test_file_reader::FileReader;
    use test_fixture::Fixture;
    use test_python_files::PythonFiles;

    /// fileslist_include_python_source_files is a complex test for the include_python_source_files
    /// function.  It copies an "input" fileslist to a temporary directory, copies an "expected"
    /// fileslist, creates a file system tree to match the "expected" fileslist and updates both
    /// fileslists to point to the file system tree in the temporary directory.  After all this, it
    /// runs fileslist_include_python_source_files and verifies that the expected Python source
    /// files are included in the "input" fileslist.
    #[test]
    fn fileslist_include_python_source_files() {
        let fileslist_input_file = Fixture::copy("fileslist_include_python_source_files.input");
        let fileslist_expected_file =
            Fixture::copy("fileslist_include_python_source_files.expected");
        {
            let python_files = PythonFiles::new(&fileslist_input_file, &fileslist_expected_file);
            python_files.touch_files_and_update_fileslists().unwrap();
        }

        let mut fileslist = FilesList::new(&fileslist_input_file);
        fileslist.include_python_source_files().unwrap();

        assert_eq!(
            FileReader::new(&fileslist_expected_file),
            FileReader::new(&fileslist_input_file)
        )
    }
}
