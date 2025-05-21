use crate::data_structures::FileCmp;
use crate::data_structures::SimpleFileType;
use crate::data_structures::PartialFileComparison;
use crate::data_structures::FullFileComparison;


/* A struct used to keep count of the max number and the found number of files, directories, and
 * soft links in a given directory tree comparison */
pub struct Totals {
    pub max_file_matches: u128,
    pub max_dir_matches: u128,
    pub max_softlink_matches: u128,
    pub file_matches: u128,
    pub dir_matches: u128,
    pub softlink_matches: u128,
}


impl Totals {
    /// Attempts to construct a `Totals` from a directory tree comparison, i.e. a
    /// `&Vec<FullFileComparison>`.
    ///
    /// #### Parameters:
    /// * `directory_tree_comparison` the `&Vec<FullFileComparison>` we wish to calculate a total
    ///     for.
    /// #### Return:
    /// * a `Totals` representing data about the various matches in the directory tree comparison
    ///     and the maximum number of matches possible.
    pub fn calculate_from(directory_tree_comparison: &Vec<FullFileComparison>) -> Totals {
        /* {{{ */
        let mut totals_count = default_totals();

        /* For every comparison in the list... */
        for cmp in directory_tree_comparison {
            totals_count.update(&cmp.partial_cmp);
        }

        return totals_count;
        /* }}} */
    }

    /// Modifies a `Totals` struct by incrementing the relevant members inside it based on the
    /// result of a `PartialFileComparison` represented by `p_cmp`.
    ///
    /// #### Parameters:
    /// * `p_cmp` a `PartialFileComparison` containing only the necessary the information about the
    ///     2 files that were compared.
    // TODO: this function is difficult to read and long. See you if you can fix it sometime
    fn update(&mut self, p_cmp: &PartialFileComparison) {
        /* {{{ */
        /* First we determine how the given `PartialFileComparison` should affect the max file,
         * directory, etc. match counts in the `Totals` struct */
        match &p_cmp.first_ft {
            Some(f_ft) => match f_ft {
                SimpleFileType::Directory => self.max_dir_matches += 1,
                _ => {
                    match &p_cmp.second_ft {
                        Some(s_ft) => match s_ft {
                            SimpleFileType::Directory => self.max_dir_matches += 1,
                            _ => (),
                        },
                        None => (),
                    }
                }
            },
            None => match &p_cmp.second_ft {
                Some(s_ft) => match s_ft {
                    SimpleFileType::Directory => self.max_dir_matches += 1,
                    _ => (),
                },
                None => (),
            },
        }
        match &p_cmp.first_ft {
            Some(f_ft) => match f_ft {
                SimpleFileType::RegFile => self.max_file_matches += 1,
                _ => match &p_cmp.second_ft {
                    Some(s_ft) => match s_ft {
                        SimpleFileType::RegFile => self.max_file_matches += 1,
                        _ => (),
                    },
                    None => (),
                }
            },
            None => {
                match &p_cmp.second_ft {
                    Some(s_ft) => match s_ft {
                        SimpleFileType::RegFile => self.max_file_matches += 1,
                        _ => (),
                    },
                    None => (),
                }
            },
        }
        match &p_cmp.first_ft {
            Some(f_ft) => match f_ft {
                SimpleFileType::SoftLink => self.max_softlink_matches += 1,
                _ => match &p_cmp.second_ft {
                    Some(s_ft) => match s_ft {
                        SimpleFileType::SoftLink => self.max_softlink_matches += 1,
                        _ => (),
                    },
                    None => (),
                }
            },
            None => {
                match &p_cmp.second_ft {
                    Some(s_ft) => match s_ft {
                        SimpleFileType::SoftLink => self.max_softlink_matches += 1,
                        _ => (),
                    },
                    None => (),
                }
            },
        }

        /* Second, we determine how the given `PartialFileComparison` should affect the actual
         * file, directory, etc. match counts in the `Totals` struct */
        match &p_cmp.file_cmp {
            FileCmp::Match => match p_cmp.first_ft.clone().unwrap() {
                SimpleFileType::RegFile => self.file_matches += 1,
                SimpleFileType::Directory => self.dir_matches += 1,
                SimpleFileType::SoftLink => self.softlink_matches += 1,
            },
            /* If the file comparison is anything but a match, do nothing to the totals */
            _ => (),
        }
        /* }}} */
    }
}


/// Returns a freshly initialized Totals struct.
///
/// #### Return:
/// * a `Totals` struct with all its values set to suitable defaults.
pub fn default_totals() -> Totals {
    return Totals {
        max_file_matches: 0,
        max_dir_matches: 0,
        max_softlink_matches: 0,
        file_matches: 0,
        dir_matches: 0,
        softlink_matches: 0,
    };
}
