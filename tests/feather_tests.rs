use std::fs;
use pelin::feather::FeatherManager;

#[test]
fn test_import_feather() {
    let temp_dir = tempfile::tempdir().unwrap();
    let project_root = temp_dir.path().to_path_buf();
    fs::create_dir(project_root.join("feathers")).unwrap();
    fs::write(
        project_root.join("feathers").join("test_feather.pl"),
        "// Test feather content",
    )
    .unwrap();

    let mut manager = FeatherManager::new(project_root);
    assert!(manager.import("test_feather").is_ok());
    assert!(manager.feathers.contains_key("test_feather"));
}

#[test]
fn test_import_local_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let project_root = temp_dir.path().to_path_buf();
    fs::write(project_root.join("local_file.pl"), "// Local file content").unwrap();

    let mut manager = FeatherManager::new(project_root);
    assert!(manager.import(".local_file").is_ok());
    assert!(manager.feathers.contains_key(".local_file"));
}

#[test]
fn test_import_nonexistent_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let project_root = temp_dir.path().to_path_buf();
    let mut manager = FeatherManager::new(project_root);
    assert!(manager.import("nonexistent").is_err());
}
