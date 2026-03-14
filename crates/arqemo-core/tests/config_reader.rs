use arqemo_core::config::registry::ThemeRegistry;
use arqemo_core::config::root::ConfigRoot;
#[test]
fn test_config_reader() {
    let cfg = ConfigRoot::locate().unwrap();
    println!("{:?}", cfg);
    let reg = ThemeRegistry::scan(&cfg).unwrap();
    println!("{:?}", reg.available_names());
    let _ = ThemeRegistry::theme_path(&reg, "brutalist").unwrap();
}
