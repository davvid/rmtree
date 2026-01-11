# Changelog

## v1.3.0 (Upcoming)

**Development**:

- The minimum supported Rust version is now `1.86.0`.

- The `clap-cargo` and `rayon` dependencies were upgraded.


## v1.2.1

*Released 2026-01-10*

**Development**:

- Dependencies were pinned to maintain the minimum supported Rust version of `1.74.0`.


## v1.2.0

*Released 2025-07-16*

**Features**:

- `rmtree -p | --parents` can now be used to remove leading parent directories that
become empty after removing the paths passed to `rmtree`. This is equivalent to
`rm -rf "$path"` followed by `rmdir -p "$(dirname "$path")"` to remove empty leading
directories.

**Development**:

- `rmtree::rmtrees_with_parents(...)` was added to the public API.

- `rmtree::Params` now includes a `pub parents` field to control
whether `rmtree::rmtrees_with_params(...)` dispatches to
`rmtree::rmtrees_with_parents(...)`.

- `rmtree::rmtrees_with_params(...)` was updated to respects the `parents`
field in `struct Params`.


## v1.1.0

*Released 2025-07-15*

**Documentation**:

- Documentation improvements.

**Development**:

- The thread pool limits can now be controlled independently by calling
`rmtree::Params::update()` on a `Params` instance that has its `threads`
field set.

- The `rmtree::Params` struct and its fields are now `pub`.


## v1.0.0

*Released 2025-07-15*

**Features**:

- The initial release was feature complete and stable.
