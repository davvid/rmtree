# Changelog

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
