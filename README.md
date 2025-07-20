# rmtree

`rmtree` removes files and directory trees quickly by performing removals in parallel

`rmtree` is a faster placement for `rm -rf`.

```bash
# Install the "rmtree" command-line tool.
cargo install rmtree
```

## Documentation

Run `rmtree --help` for more usage details.

`rmtree` scans and removes files and directoresi from the filesystem in parallel.
This allows it to achieve significant performance gains over the coreutils `rm` command.


## Installation

* [Pre-built rmtree binaries](https://github.com/davvid/rmtree/releases)

Pre-built `rmtree` binaries are statically linked using
[musl libc](https://musl.libc.org/) so that they can run on any Linux system.

## Benchmarks

These timings were gathered on a 40 core Intel(R) Xeon(R) Gold 6242R CPU @ 3.10GHz.
The storage was accessed over NFS against a live cluster from a Linux host.

A modestly-sized C++ project with build artifacts was used in these measurements.
The test directory contains 1663 directories and 5822 files totalling 12 GB.
This skews towards many small files and many directories.

### Timings

All timings were performed with a warm NFS cache.
As a read-only baseline, timings for `find` and [sharkdp/fd](https://github.com/sharkdp/fd) are included.

| Command           | Time (s)  |
|-------------------|-----------|
| `fd`              | 0.249     |
| `find`            | 0.448     |

The following table summarizes the timings for `rm -rf` from GNU coreutils 8.32 and
`rmtree` with different thread settings.

| Command           | Time (s)  | Speedup   | Scaling   | Normalized    |
|-------------------|-----------|-----------|-----------|---------------|
| `rm -rf`          | 25.697    |  1.000    |           |               |
| `rmtree -t 1`     | 23.729    |  1.083    |  1.000    | 1.000         |
| `rmtree -t 2`     | 11.146    |  2.305    |  2.129    | 1.064         |
| `rmtree -t 3`     |  7.866    |  3.267    |  3.017    | 1.006         |
| `rmtree -t 4`     |  5.476    |  4.693    |  4.333    | 1.083         |
| `rmtree -t 5`     |  4.556    |  5.640    |  5.208    | 1.042         |
| `rmtree -t 6`     |  4.041    |  6.359    |  5.872    | 0.978         |
| `rmtree -t 7`     |  3.581    |  7.176    |  6.626    | 0.947         |
| `rmtree -t 8`     |  3.039    |  8.456    |  7.808    | 0.976         |
| `rmtree -t 9`     |  2.712    |  9.475    |  8.750    | 0.972         |
| `rmtree -t 10`    |  2.635    |  9.752    |  9.005    | 0.901         |
| `rmtree -t 11`    |  2.179    | 11.793    | 10.890    | 0.990         |
| `rmtree -t 12`    |  2.039    | 12.603    | 11.638    | 0.970         |
| `rmtree -t 16`    |  1.941    | 13.239    | 12.225    | 0.764         |
| `rmtree -t 20`    |  1.634    | 15.726    | 14.522    | 0.726         |
| `rmtree -t 24`    |  1.354    | 18.979    | 17.525    | 0.730         |
| `rmtree -t 32`    |  1.655    | 15.527    | 14.338    | 0.448         |

Relative performance is the performance relative to `rm -rf`.
Thread scaling is measured relative to `rmtree -j1`.

YMMV depending on your storage hardware, but in this test the sweet spot was somewhere
between 5 and 9 threads.


## Links

* [rmtree on crates.io](https://crates.io/crates/rmtree)


## Code Status

[![Build status](https://gitlab.com/davvid/rmtree/badges/main/pipeline.svg)](https://gitlab.com/davvid/rmtree/-/pipelines)
[![MIT License](https://img.shields.io/gitlab/license/davvid/rmtree.svg)](LICENSE)

RmTree is actively maintained and its core functionality is stable and feature-complete.
