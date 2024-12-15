use std::path::Path;

pub fn resolve(base: &str, path: &str) -> String {
    let base_path = if base.ends_with('/') {
        Path::new(base.strip_suffix('/').unwrap())
    } else {
        Path::new(base).parent().unwrap()
    };
    let path_path = Path::new(path);

    if path_path.is_relative() {
        String::from(base_path.join(path_path).to_str().unwrap())
    } else {
        String::from(path_path.to_path_buf().to_str().unwrap())
    }
}
