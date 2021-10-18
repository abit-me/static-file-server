use std::path::Path;
pub const FILE_DIR: &str = "./files";

pub fn create() {

    let _ = std::fs::create_dir_all(FILE_DIR);
    // 假设文件目录不存在，创建一个目录
    if Path::new(FILE_DIR).exists() == false {
        let _ = std::fs::create_dir_all(FILE_DIR);
    }
}