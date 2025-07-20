use clap::Parser;
use rayon::prelude::*;

/// Remove directory trees and files based on the specified Params.
pub fn rmtrees_with_params(params: Params) {
    rmtrees(&params.paths);
}

/// Remove the specified directory trees and files in parallel.
pub fn rmtrees(paths: &Vec<std::path::PathBuf>) {
    paths.par_iter().for_each(|path| {
        rmtree(path).ok(); // Errors are ignored.
    });
}

/// Remove either a single file or a directory and its contents.
pub fn rmtree(path: &std::path::Path) -> std::io::Result<()> {
    if path.is_symlink() || path.is_file() {
        std::fs::remove_file(path)
    } else {
        remove_directory(path)
    }
}

/// Parameters that control the rmtree command.
#[derive(Parser, Clone, Debug)]
#[command(author, version, about, long_about = None)]
#[command(styles = clap_cargo::style::CLAP_STYLING)]
pub struct Params {
    /// Run commands in parallel using the specified number of threads
    #[arg(
        long,
        short = 't',
        visible_short_alias = 'j',
        require_equals = false,
        num_args = 0..=1,
        default_missing_value = "0",
        value_name = "THREADS",
    )]
    threads: Option<usize>,
    /// Paths to delete
    #[arg(required = true)]
    paths: Vec<std::path::PathBuf>,
}

impl Params {
    /// Initialize the global rayon thread pool used by rmtree.
    /// You must call update() before calling rmtrees_with_params(),
    /// rmtrees(), or rmtree() in order for the thread limits to be applied.
    pub fn update(mut self) -> Self {
        let Some(threads) = self.threads else {
            return self;
        };
        let num_threads = if threads == 0 {
            default_num_threads()
        } else {
            threads
        };
        if num_threads != threads {
            self.threads = Some(num_threads);
        }
        rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build_global()
            .ok();

        self
    }
}

/// Get the default number of threads to run in parallel
fn default_num_threads() -> usize {
    let min_num_threads = 2;
    match std::thread::available_parallelism() {
        Ok(value) => std::cmp::max(value.get(), min_num_threads),
        Err(_) => min_num_threads,
    }
}

/// Recursively traverse the child subdirectories in parallel.
/// Files are depth-first removed. The remaining empty directory is processed after
/// The recursive traversal completes.
fn remove_directory(path: &std::path::Path) -> std::io::Result<()> {
    let Ok(read_dir) = std::fs::read_dir(path) else {
        return rm_rf::ensure_removed(path).map_err(|err| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Unable to read directory {path:?}: {err}"),
            )
        });
    };
    let entries: Vec<_> = read_dir.collect();
    entries.par_iter().for_each(|entry_result| {
        if let Ok(entry) = entry_result {
            let path = entry.path();
            if path.is_symlink() || path.is_file() {
                std::fs::remove_file(&path).unwrap_or(());
            } else {
                remove_directory(&path).unwrap_or(());
            }
        }
    });

    rm_rf::ensure_removed(path).map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Unable to remove directory {path:?}: {err}"),
        )
    })
}
