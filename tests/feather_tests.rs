use std::fs;
use std::io::Write;
use std::path::PathBuf;
use pelin::feather::FeatherManager;
use pelin::interpreter::Value;

#[test]
fn test_import_nonexistent_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let project_root = temp_dir.path().to_path_buf();
    let mut manager = FeatherManager::new(project_root);
    assert!(manager.import("nonexistent").is_err());
}

#[test]
fn test_feather_manager_creation() {
    let project_root = PathBuf::from("/tmp/test_project");
    let manager = FeatherManager::new(project_root.clone());
    assert_eq!(manager.project_root, project_root);
    assert!(manager.feathers.is_empty());
    assert!(manager.libraries.lock().unwrap().is_empty());
}

#[test]
fn test_import_non_existent_feather() {
    let project_root = PathBuf::from("/tmp/test_project");
    let root_dir = project_root.clone();
    let mut manager = FeatherManager::new(project_root);
    let result = manager.import("non_existent_feather");
    assert!(result.is_err());
    let expected_error = format!(
        "Could not find Feather file: {}",
        root_dir.join("feathers").join("non_existent_feather.pl").display()
    );
    assert_eq!(result.unwrap_err(), expected_error);
}

#[test]
fn test_import_and_call_function() {
    // Create a temporary directory for our test project
    let project_root = tempfile::tempdir().unwrap();
    let feathers_dir = project_root.path().join("feathers");
    fs::create_dir(&feathers_dir).unwrap();

    // Create a test feather file
    let feather_content = r#"
    fn num add(num a, num b) {
        RUST[std_func::add](a, b)
    }
    "#;
    let feather_path = feathers_dir.join("test_math.pl");
    let mut file = fs::File::create(&feather_path).unwrap();
    file.write_all(feather_content.as_bytes()).unwrap();

    // Create FeatherManager and import the test feather
    let mut manager = FeatherManager::new(project_root.path().to_path_buf());
    let import_result = manager.import("test_math");
    assert!(import_result.is_ok());

    // Test calling the imported function
    let result = manager.call_function("test_math", "add", vec![Value::Number(2.0), Value::Number(3.0)]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(5.0));
}

#[test]
fn test_call_non_existent_function() {
    let project_root = PathBuf::from("/tmp/test_project");
    let manager = FeatherManager::new(project_root);
    let result = manager.call_function("test_math", "non_existent_function", vec![]);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Feather 'test_math' not found"
    );
}