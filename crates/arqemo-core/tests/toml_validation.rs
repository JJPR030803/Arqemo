use arqemo_core::validate::file::validate_file;
use std::path::PathBuf;

#[test]
fn basic_toml_validation() {
    let path = PathBuf::from("/mnt/shared/arqemo/test_theme.toml");
    let ok = validate_file(&path).unwrap();
    assert_eq!(ok.meta.name, "test");
}
