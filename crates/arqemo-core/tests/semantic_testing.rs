use std::path;
use arqemo_core::validate::semantic::validate_semantic;
use arqemo_core::validate::file::validate_file;
use path::Path;
use arqemo_core::schema::ThemeConfig;

#[test]
fn test_semantic_validation() {
    let file = Path::new("/mnt/shared/arqemo/test_theme.toml");
    let file_validation = validate_file(file).map_err(|e| e.to_string()).unwrap();

    println!("{:#?}", file_validation);

    let sem = validate_semantic(&file_validation).unwrap();
    println!("{:#?}", sem);
}