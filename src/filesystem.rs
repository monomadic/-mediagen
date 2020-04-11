#![allow(dead_code)]

// general purpose filesystem utility functions.

use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

/// Recursively walk a directory
///
/// # Examples
///
/// ```no_run
/// let mut entries = fs::walk_dir(".")?
///     .map(|res| res.map(|e| e.path()))
///     .collect::<Result<Vec<_>, io::Error>>()?;
/// ```
///
pub(crate) fn walk_dir<P: AsRef<Path>>(
    dir: P,
    cb: &dyn Fn(&DirEntry)
) -> io::Result<()> {
    if dir.as_ref().is_dir() {
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
