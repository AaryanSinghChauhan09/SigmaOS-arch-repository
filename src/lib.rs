//! Package repository management for SigmaOS.
//!
//! Manages repository definitions, sync operations, and package queries.
//! Inspired by Arch Linux's pacman repository handling.

use std::collections::HashMap;
use std::fmt;
pub mod sigmarepositorymanager;

/// Errors for repository operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepoError {
    EmptyName,
    DuplicateRepo(String),
    RepoNotFound(String),
    InvalidUrl,
    SyncFailed(String),
}

impl fmt::Display for RepoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "repository name must not be empty"),
            Self::DuplicateRepo(n) => write!(f, "duplicate repository: {n}"),
            Self::RepoNotFound(n) => write!(f, "repository not found: {n}"),
            Self::InvalidUrl => write!(f, "invalid repository URL"),
            Self::SyncFailed(n) => write!(f, "sync failed for {n}"),
        }
    }
}

impl std::error::Error for RepoError {}

/// Repository definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Repository {
    pub name: String,
    pub url: String,
    pub enabled: bool,
    pub last_sync: Option<u64>,
    pub package_count: usize,
}

impl Repository {
    pub fn new(name: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            url: url.into(),
            enabled: true,
            last_sync: None,
            package_count: 0,
        }
    }

    pub fn validate(&self) -> Result<(), RepoError> {
        if self.name.trim().is_empty() {
            return Err(RepoError::EmptyName);
        }
        if !self.url.starts_with("http") {
            return Err(RepoError::InvalidUrl);
        }
        Ok(())
    }
}

/// Repository manager.
#[derive(Debug, Clone, Default)]
pub struct RepoManager {
    repos: HashMap<String, Repository>,
}

impl RepoManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_repo(&mut self, repo: Repository) -> Result<(), RepoError> {
        repo.validate()?;
        if self.repos.contains_key(&repo.name) {
            return Err(RepoError::DuplicateRepo(repo.name.clone()));
        }
        let name = repo.name.clone();
        self.repos.insert(name, repo);
        Ok(())
    }

    pub fn remove_repo(&mut self, name: &str) -> Result<(), RepoError> {
        self.repos.remove(name).ok_or_else(|| RepoError::RepoNotFound(name.to_string()))?;
        Ok(())
    }

    pub fn get_repo(&self, name: &str) -> Option<&Repository> {
        self.repos.get(name)
    }

    pub fn list_repos(&self) -> Vec<&Repository> {
        self.repos.values().collect()
    }

    pub fn list_enabled(&self) -> Vec<&Repository> {
        self.repos.values().filter(|r| r.enabled).collect()
    }

    pub fn sync_repo(&mut self, name: &str) -> Result<(), RepoError> {
        let repo = self.repos.get_mut(name)
            .ok_or_else(|| RepoError::RepoNotFound(name.to_string()))?;
        repo.last_sync = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs());
        repo.package_count += 1; // Simulate package discovery
        Ok(())
    }

    pub fn sync_all(&mut self) -> Result<(), RepoError> {
        let names: Vec<String> = self.repos.keys().cloned().collect();
        for name in names {
            self.sync_repo(&name)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_repo_aaa() {
        // Arrange
        let mut mgr = RepoManager::new();
        let repo = Repository::new("core", "https://repo.example.com/core");

        // Act
        let res = mgr.add_repo(repo);

        // Assert
        assert!(res.is_ok());
        assert_eq!(mgr.list_repos().len(), 1);
    }

    #[test]
    fn test_duplicate_repo_rejected_aaa() {
        // Arrange
        let mut mgr = RepoManager::new();
        let repo = Repository::new("core", "https://repo.example.com/core");
        mgr.add_repo(repo.clone()).unwrap();

        // Act
        let res = mgr.add_repo(repo);

        // Assert
        assert!(res.is_err());
    }

    #[test]
    fn test_invalid_url_rejected_aaa() {
        // Arrange
        let mut mgr = RepoManager::new();
        let repo = Repository::new("core", "ftp://invalid");

        // Act
        let res = mgr.add_repo(repo);

        // Assert
        assert!(res.is_err());
    }

    #[test]
    fn test_sync_repo_aaa() {
        // Arrange
        let mut mgr = RepoManager::new();
        mgr.add_repo(Repository::new("core", "https://repo.example.com/core")).unwrap();

        // Act
        let res = mgr.sync_repo("core");

        // Assert
        assert!(res.is_ok());
        assert!(mgr.get_repo("core").unwrap().last_sync.is_some());
        assert!(mgr.get_repo("core").unwrap().package_count > 0);
    }
}
