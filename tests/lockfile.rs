//! Lockfile integration test

// TODO(tarcieri): add more example `Cargo.lock` files which cover more scenarios

use cargo_lock::{metadata, Lockfile, Version};

/// Load this crate's own `Cargo.lock` file
#[test]
fn load_our_own_lockfile() {
    let lockfile = Lockfile::load("Cargo.lock").unwrap();
    assert!(lockfile.packages.len() > 0);
}

/// Load a non-trivial example `Cargo.lock` file (from the Cargo project itself)
#[test]
fn load_example_cargo_lockfile() {
    let lockfile = Lockfile::load("tests/support/Cargo.lock.example-cargo").unwrap();

    assert_eq!(lockfile.packages.len(), 141);
    assert_eq!(lockfile.metadata.len(), 136);

    let package = &lockfile.packages[0];
    assert_eq!(package.name.as_ref(), "adler32");
    assert_eq!(package.version, Version::parse("1.0.4").unwrap());

    let metadata_key: metadata::Key =
        "checksum adler32 1.0.4 (registry+https://github.com/rust-lang/crates.io-index)"
            .parse()
            .unwrap();

    let metadata_value = &lockfile.metadata[&metadata_key];
    assert_eq!(
        metadata_value.as_ref(),
        "5d2e7343e7fc9de883d1b0341e0b13970f764c14101234857d2ddafa1cb1cac2"
    );
}

/// Load a non-trivial example `Cargo.lock` file (from rustc)
#[test]
fn load_example_rustc_lockfile() {
    let lockfile = Lockfile::load("tests/support/Cargo.lock.example-rustc").unwrap();
    assert_eq!(lockfile.packages.len(), 472);
    assert_eq!(lockfile.metadata.len(), 0);
}

/// Dependency tree tests
#[cfg(feature = "dependency-tree")]
mod tree {
    use super::Lockfile;

    /// Compute a dependency graph from this crate's own `Cargo.lock`
    #[test]
    fn compute_from_our_own_lockfile() {
        let tree = Lockfile::load("Cargo.lock")
            .unwrap()
            .dependency_tree()
            .unwrap();

        assert!(tree.nodes().len() > 0);
    }

    /// Compute a dependency graph from a non-trivial example `Cargo.lock`
    /// (i.e. from the Cargo project itself)
    #[test]
    fn compute_from_example_cargo_lockfile() {
        let tree = Lockfile::load("tests/support/Cargo.lock.example-cargo")
            .unwrap()
            .dependency_tree()
            .unwrap();

        assert_eq!(tree.nodes().len(), 141);
    }

    /// Compute a dependency graph from a non-trivial example `Cargo.lock`
    /// (i.e. from rustc)
    #[test]
    fn compute_from_example_rustc_lockfile() {
        let tree = Lockfile::load("tests/support/Cargo.lock.example-rustc")
            .unwrap()
            .dependency_tree()
            .unwrap();

        assert_eq!(tree.nodes().len(), 472);
    }
}
