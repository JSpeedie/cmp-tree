use std::fs::DirEntry;
use std::fs::FileType;
use std::path::PathBuf;


#[derive(Debug,PartialEq,Eq)]
pub enum FileCmp {
    Mismatch,
    Match,
}


#[derive(Debug,PartialEq,Eq)]
pub enum SimpleFileType {
    RegFile,
    Directory,
    SoftLink,
}


impl SimpleFileType {
    /* {{{ */
    /// Attempts to construct a `SimpleFileType` from a `std::fs::FileType`.
    ///
    /// #### Parameters:
    /// * `fs_filetype` the `std::fs::FileType` we wish to map to a `SimpleFileType`.
    /// #### Return:
    /// * a `SimpleFileType` representing one of the file types this library supports on success,
    ///     and on error, a unit type (`()`).
    pub fn try_from(fs_filetype: &FileType) -> Result<SimpleFileType, ()> {
        if fs_filetype.is_file() {
            return Ok(SimpleFileType::RegFile);
        } else if fs_filetype.is_dir() {
            return Ok(SimpleFileType::Directory);
        } else if fs_filetype.is_symlink() {
            return Ok(SimpleFileType::SoftLink);
        } else {
            return Err(());
        }
    }
    /* }}} */
}


pub struct SimpleFile {
    pub f_name: PathBuf,
    pub d_entry: DirEntry,
}
