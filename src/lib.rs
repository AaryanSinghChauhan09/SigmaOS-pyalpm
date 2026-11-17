//! ALPM package management interface for SigmaOS.
//!
//! Provides package database operations: sync, install, remove, query.
//! Inspired by Arch Linux's pyalpm but implemented as a standalone Rust crate.

use std::collections::HashMap;
use std::fmt;

/// Errors for ALPM operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlpmError {
    DatabaseNotInitialized,
    PackageNotFound(String),
    AlreadyInstalled(String),
    DependencyConflict(String),
    TransactionFailed(String),
}

impl fmt::Display for AlpmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DatabaseNotInitialized => write!(f, "database not initialized"),
            Self::PackageNotFound(p) => write!(f, "package not found: {p}"),
            Self::AlreadyInstalled(p) => write!(f, "package already installed: {p}"),
            Self::DependencyConflict(d) => write!(f, "dependency conflict: {d}"),
            Self::TransactionFailed(t) => write!(f, "transaction failed: {t}"),
        }
    }
}

impl std::error::Error for AlpmError {}

/// Package information.
#[derive(Debug, Clone, PartialEq)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub installed: bool,
}

impl Package {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            description: String::new(),
            dependencies: Vec::new(),
            installed: false,
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    pub fn with_dependencies(mut self, deps: Vec<String>) -> Self {
        self.dependencies = deps;
        self
    }
}

/// Transaction type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionType {
    Install,
    Remove,
    Upgrade,
}

/// ALPM database.
#[derive(Debug, Clone, Default)]
pub struct AlpmDatabase {
    packages: HashMap<String, Package>,
    synced: bool,
}

impl AlpmDatabase {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn sync(&mut self) -> Result<(), AlpmError> {
        self.synced = true;
        Ok(())
    }

    pub fn is_synced(&self) -> bool {
        self.synced
    }

    pub fn add_package(&mut self, pkg: Package) {
        self.packages.insert(pkg.name.clone(), pkg);
    }

    pub fn get_package(&self, name: &str) -> Result<&Package, AlpmError> {
        if !self.synced {
            return Err(AlpmError::DatabaseNotInitialized);
        }
        self.packages.get(name).ok_or_else(|| AlpmError::PackageNotFound(name.to_string()))
    }

    pub fn install(&mut self, name: &str) -> Result<(), AlpmError> {
        if !self.synced {
            return Err(AlpmError::DatabaseNotInitialized);
        }
        let pkg = self.packages.get_mut(name)
            .ok_or_else(|| AlpmError::PackageNotFound(name.to_string()))?;
        if pkg.installed {
            return Err(AlpmError::AlreadyInstalled(name.to_string()));
        }
        pkg.installed = true;
        Ok(())
    }

    pub fn remove(&mut self, name: &str) -> Result<(), AlpmError> {
        if !self.synced {
            return Err(AlpmError::DatabaseNotInitialized);
        }
        let pkg = self.packages.get_mut(name)
            .ok_or_else(|| AlpmError::PackageNotFound(name.to_string()))?;
        pkg.installed = false;
        Ok(())
    }

    pub fn list_installed(&self) -> Vec<&Package> {
        self.packages.values().filter(|p| p.installed).collect()
    }

    pub fn list_available(&self) -> Vec<&Package> {
        self.packages.values().filter(|p| !p.installed).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_required_aaa() {
        // Arrange
        let db = AlpmDatabase::new();

        // Act
        let res = db.get_package("bash");

        // Assert
        assert!(res.is_err());
    }

    #[test]
    fn test_install_package_aaa() {
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

    #[test]
    fn test_double_install_fails_aaa() {
        // Arrange
        let mut db = AlpmDatabase::new();
        db.sync().unwrap();
        db.add_package(Package::new("bash", "5.2"));
        db.install("bash").unwrap();

        // Act
        let res = db.install("bash");

        // Assert
        assert!(res.is_err());
    }
}
