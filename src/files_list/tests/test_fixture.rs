use std::path::{Path, PathBuf};

use tempfile::TempDir;

// Thanks Andrew Radev!
// https://andrewra.dev/2019/03/01/testing-in-rust-temporary-files/
pub struct Fixture {
    path: PathBuf,
    source: PathBuf,
    _tempdir: TempDir,
}

impl Fixture {
    fn blank(fixture_filename: &str) -> Self {
        let root = env!("CARGO_MANIFEST_DIR");
        let mut source = PathBuf::from(root);
        source.push("tests/fixtures");
        source.push(&fixture_filename);

        let tempdir =
            tempfile::tempdir().expect("Failed to initialize a temporary directory for a fixture");
        let mut path = PathBuf::from(&tempdir.path());
        path.push(&fixture_filename);

        Fixture {
            path,
            source,
            _tempdir: tempdir,
        }
    }
    pub fn copy(fixture_filename: &str) -> Self {
        let fixture = Fixture::blank(fixture_filename);
        std::fs::copy(&fixture.source, &fixture.path)
            .expect("Failed to copy a fixture file to the temporary directory");
        fixture
    }
}

impl std::ops::Deref for Fixture {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        self.path.deref()
    }
}
