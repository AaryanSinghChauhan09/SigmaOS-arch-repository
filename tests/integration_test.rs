// Integration test for SigmaOS-repository
// Verifies the crate builds and links as a dependency.

use sigmaos_repository::{RepoManager, Repository};

#[test]
fn crate_links_and_runs() {
    // Arrange
    let mut mgr = RepoManager::new();
    mgr.add_repo(Repository::new("core", "https://repo.example.com/core")).unwrap();

    // Act
    let res = mgr.sync_repo("core");

    // Assert
    assert!(res.is_ok());
    assert_eq!(mgr.list_repos().len(), 1);
}
