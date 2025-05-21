use std::cmp::Ordering;
use std::fs::FileType;
use std::path::PathBuf;


#[derive(Debug,PartialEq,Eq,PartialOrd,Ord)]
pub enum FileCmp {
    /* (1) For Existence Comparisons */
    /* For when neither of the two files (understood in the broad sense) exist. */
    ExistenceNeitherFile,
    /* For when only the first of the two files (understood in the broad sense) exists. */
    ExistenceOnlyFirstFile,
    /* For when only the second of the two files (understood in the broad sense) exists. */
    ExistenceOnlySecondFile,
    /* (2) For File Type Comparisons */
    /* For when the two files (understood in the broad sense) mismatch in their type (e.g. one is a
    * directory, one is a regular file). */
    FileTypeTypeMismatch,
    /* (3) For Substance Comparisons */
    /* For when the two files mismatch in their content (i.e. they are not byte-for-byte
    * identical). */
    SubstanceRegFileContentMismatch,
    /* For when the two soft links mismatch in their link path */
    SubstanceSoftLinkLinkMismatch,
    /* (4) For Metadata Comparisons */
    MetadataModificationTimeMismatch,
    /* (5) For complete matches */
    /* For when the two files (understood in the broad sense) match don't mismatch in any of the
    * possible ways represented above */
    Match,
}


#[derive(Debug,PartialEq,Eq,Clone,PartialOrd,Ord)]
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


#[derive(Debug,PartialEq,Eq,PartialOrd)]
pub struct PartialFileComparison {
    pub file_cmp: FileCmp,
    pub first_ft: Option<SimpleFileType>,
    pub second_ft: Option<SimpleFileType>,
}


impl Ord for PartialFileComparison {
    /* {{{ */
    fn cmp(&self, other: &Self) -> Ordering {
        /* Compare the `file_cmp` member. If that comparison returns a `Less` or `Greater`
         * Ordering, our work is done, otherwise proceed to compare the next member */
        match (&(self.file_cmp)).cmp(&(other.file_cmp)) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Equal => {
                /* Provide a closure for calculating the "value" of a given
                 * `Option<SimpleFileType>` */
                let calculate_ft_value = |input: &Option<SimpleFileType>| {
                    match &input {
                        None => return 0,
                        Some(ft) => match ft {
                            SimpleFileType::RegFile => return 1,
                            SimpleFileType::Directory => return 2,
                            SimpleFileType::SoftLink => return 3,
                        }
                    }
                };

                /* Compare the `first_ft` member. If that comparison returns a `Less` or `Greater`
                 * Ordering, our work is done, otherwise proceed to compare the next member */
                match calculate_ft_value(&(self.first_ft))
                    .cmp(&calculate_ft_value(&(other.first_ft))) {

                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Equal => {
                        /* Compare the `second_ft` member. Since this is the last member to compare,
                         * return whatever it evaluates to. */
                        return calculate_ft_value(&(self.second_ft))
                            .cmp(&calculate_ft_value(&(other.second_ft)));
                    }
                }
            }
        }
    }
    /* }}} */
}


#[derive(Debug,PartialEq,Eq,PartialOrd,Ord)]
pub struct FullFileComparison {
    pub partial_cmp: PartialFileComparison,
    pub first_path: PathBuf,
    pub second_path: PathBuf,
}
