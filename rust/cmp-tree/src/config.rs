/* A struct used to define the configuration `cmp-tree` functions will run under. Many functions
 * within `cmp-tree` will require a Config struct and the values of said struct will affect how
 * they work or run. */
pub struct Config {
    pub compare_modification_times: bool,
    pub matches: bool,
    pub pretty: bool,
    pub silent: bool,
    pub single_threaded: bool,
    pub totals: bool,
}


/// Returns the default config for `cmp-tree`.
///
/// #### Return:
/// * a `Config` struct with all its values set to the default values for `cmp-tree`.
pub fn default_config() -> Config {
    /* {{{ */
    return Config {
        compare_modification_times: false,
        matches: false,
        pretty: false,
        silent: false,
        single_threaded: false,
        totals: false,
    };
    /* }}} */
}
