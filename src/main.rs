use clap::Parser;

/// Main entry point for the "rmtree" command.
fn main() {
    let params = rmtree::Params::parse().update();
    rmtree::rmtrees_with_params(params);
}
