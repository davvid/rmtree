use clap::Parser;
use rayon::prelude::*;

/// Remove directory trees and files based on the specified Params.
pub fn rmtrees_with_params(params: Params) {
    let params = params.update();
    if params.parents {
        rmtrees_and_parents(&params.paths);
    } else {
        rmtrees(&params.paths);
    }
}

/// Remove the specified directory trees and files in parallel.
pub fn rmtrees(paths: &Vec<std::path::PathBuf>) {
    paths.par_iter().for_each(|path| {
        rmtree(path).ok(); // Errors are ignored.
    });
}

/// Remove the specified directory trees and files and their parent directories in parallel.
pub fn rmtrees_and_parents(paths: &Vec<std::path::PathBuf>) {
    paths.par_iter().for_each(|path| {
        rmtree(path).ok(); // Errors are ignored.
        remove_leading_directories(path);
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
    /// Specifies the number of threads to run simultaneously
    #[arg(
        long,
        short = 't',
        visible_short_alias = 'j',
        require_equals = false,
        num_args = 0..=1,
        default_missing_value = "0",
        value_name = "THREADS",
    )]
    pub threads: Option<usize>,
    /// Remove empty leading directories
    #[arg(long, short = 'p', default_value_t = false)]
    pub parents: bool,
    /// Paths to delete
    #[arg(required = true)]
    pub paths: Vec<std::path::PathBuf>,
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

/// Remove empty leading parent directories leading up to the specified paths.
fn remove_leading_directories(path: &std::path::Path) {
    let mut parent_option = path.parent();
    while let Some(parent) = parent_option {
        if !parent.exists() {
            break;
        }
        if std::fs::remove_dir(parent).is_err() {
            break;
        }
        parent_option = parent.parent();
    }
}
