use sigmaos_pyalpm::*;

#[test]
fn component_is_constructible_and_runnable() {
    // Arrange
    let mut db = AlpmDatabase::new();
    db.sync().unwrap();
    db.add_package(Package::new("bash", "5.2").with_description("GNU Bourne Again Shell"));

    // Act
    let res = db.install("bash");

    // Assert
    assert!(res.is_ok());
    assert_eq!(db.get_package("bash").unwrap().installed, true);
}
