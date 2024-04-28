use crate::file::File;
use clap::Parser;

/// filter a list of files by properties
///
/// stest takes a list of files and filters by the files' properties, analogous to test. Files
/// which pass all tests are printed to stdout. If no files are given, stest reads files from
/// stdin.
/// /*env!("VERSION")*/
#[derive(Clone, Debug, Parser)]
#[command(author, version = "9", about, long_about)]
pub struct Config {
    /// Test hidden files.
    #[arg(short = 'a')]
    pub requires_each_file_is_hidden: bool,
    /// Test that files are block specials.
    #[arg(short = 'b')]
    pub requires_each_file_is_block_special: bool,
    /// Test that files are character specials.
    #[arg(short = 'c')]
    pub requires_each_file_is_character_special: bool,
    /// Test that files are directories.
    #[arg(short = 'd')]
    pub requires_each_file_is_directory: bool,
    /// Test that files exist.
    ///
    /// This option does not actually alter the behavior of stest. By default, stest will always
    /// test that a file exists.
    ///
    /// This behavior deviates from the behavior of the POSIX test utility, which requires the -e
    /// option in order to perform an existence test on a file. This deviation, together with the
    /// fact that it's counterintuitive to provide an option that has no effect, implies that this
    /// may be a bug in the C implementation of stest in the original dmenu repository. The
    /// behavior is reproduced here because this rust implementation strives to be a drop-in
    /// replacement for the original and because it's unlikely that this behavior could
    /// meaningfully impact the experience of using dmenu_run: non-existing files on your PATH
    /// cannot be executed successfully, so it's always reasonable to filter them out.
    #[arg(short = 'e')]
    pub requires_each_file_exists: bool,
    /// Test that files are regular files.
    #[arg(short = 'f')]
    pub requires_each_file_is_file: bool,
    /// Test that files have their set-group-ID flag set.
    #[arg(short = 'g')]
    pub requires_each_file_has_set_group_id: bool,
    // Using -h here works and matches the original stest, however it overrides one of clap's
    // autogenerated help flags and displays confusing help information when calling with the long
    // option --help. We can disable clap's help flags if we want to but it complicates usage.
    /// Test that files are symbolic links.
    #[arg(short = 'h')]
    pub requires_each_file_is_symbolic_link: bool,
    /// Test the contents of a directory given as an argument.
    #[arg(short = 'l')]
    pub test_contents_of_directories: bool,
    /// Test that files are newer (by modification time) than file.
    ///
    /// If this option is given a non-existing file as its argument, the test is ignored.
    ///
    /// This behavior is similar to assuming the last modification time of a non-existent file is a
    /// lower bound like the unix epoch or zero, and is consistent with the behavior of GNU
    /// coreutils' test.
    #[arg(short = 'n')]
    pub oldest_file: Option<File>,
    /// Test that files are older (by modification time) than file.
    ///
    /// If this option is given a non-existing file as its argument, the test is ignored.
    ///
    /// This behavior is similar to assuming the last modification time of a non-existent file is
    /// an upper bound like infinity. Note that this behavior is not consistent with the behavior
    /// of GNU coreutils' test: checking that a file is older than (-ot) a non-existent file with
    /// GNU coreutils' test always fails, while checking the same with stest (-o) always passes.
    #[arg(short = 'o')]
    pub newest_file: Option<File>,
    /// Test that files are named pipes.
    #[arg(short = 'p')]
    pub requires_each_file_is_pipe: bool,
    /// No files are printed, only the exit status is returned.
    #[arg(short = 'q')]
    pub quiet: bool,
    /// Test that files are readable.
    #[arg(short = 'r')]
    pub requires_each_file_is_readable: bool,
    /// Test that files are not empty.
    #[arg(short = 's')]
    pub requires_each_file_has_size_greater_than_zero: bool,
    /// Test that files have their set-user-ID flag set.
    #[arg(short = 'u')]
    pub requires_each_file_has_set_user_id: bool,
    /// Invert the sense of tests, only failing files pass.
    #[arg(short = 'v')]
    pub has_inverted_tests: bool,
    /// Test that files are writable.
    #[arg(short = 'w')]
    pub requires_each_file_is_writable: bool,
    /// Test that files are executable.
    #[arg(short = 'x')]
    pub requires_each_file_is_executable: bool,
    /// files to filter on
    pub files: Vec<File>,
}
