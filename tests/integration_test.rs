// Integration test for SigmaOS-pyalpm
// Verifies the crate builds and links as a dependency.

use sigmaos_pyalpm::{AlpmDatabase, Package};

#[test]
fn crate_links_and_runs() {
    // Arrange
    let mut db = AlpmDatabase::new();
    db.sync().unwrap();
    db.add_package(Package::new("vim", "9.0"));

    // Act
    let res = db.install("vim");

    // Assert
    assert!(res.is_ok());
}
