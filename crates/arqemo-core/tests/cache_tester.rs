use arqemo_core::cache::CacheLayout;
use arqemo_core::cache::search_cache_dir;
use arqemo_core::cache::error::CacheError;
use arqemo_core::config::registry::ThemeRegistry;
#[test]
fn test_cache() {
    let cache_dir = search_cache_dir().unwrap();
    println!("{}", cache_dir.display());

    let cache_layout = match CacheLayout::check(cache_dir.as_path()) {
        Ok(layout) => layout,
        Err(e) => panic!("{:?}",e),
    };
    println!("{:?}", cache_layout);
}