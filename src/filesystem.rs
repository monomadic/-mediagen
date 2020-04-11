#![allow(dead_code)]

// general purpose filesystem utility functions.

use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

/// recursively walk directory
pub(crate) fn walk_dir(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_dir(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
