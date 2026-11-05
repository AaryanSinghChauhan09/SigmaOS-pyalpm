use sigmaos_pyalpm::*;

#[test]
fn component_is_constructible_and_runnable() {
    // Arrange
    let c = sigmaos_pyalpm::Pyalpm::new("sigmaos-pyalpm").expect("valid name");
    // Act
    let out = c.run().expect("run succeeds");
    // Assert
    assert!(out.contains("sigmaos-pyalpm"));
}
