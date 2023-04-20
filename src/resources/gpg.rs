use crate::resources::file;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use std::fs;
use std::path::Path;

lazy_static! {
    // Global file lock
    pub static ref FILE_LOCK: RwLock<u32> = RwLock::new(0);
}

pub const GPG_PATH: &str = "/opt/verdictd/gpg/";
pub const GPG_KEYRING: &str = "/opt/verdictd/gpg/keyring.gpg";

pub fn export_base64() -> Result<String, String> {
    let lock = FILE_LOCK.read();
    assert_eq!(*lock, 0);

    file::export_base64(GPG_KEYRING).map_err(|e| format!("export GPG keyring failed:{:?}", e))
}

pub fn size_base64() -> Result<usize, String> {
    let lock = FILE_LOCK.read();
    assert_eq!(*lock, 0);

    file::export_base64(GPG_KEYRING)
        .map_err(|e| format!("Fetch GPG keyring size failed:{:?}", e))
        .and_then(|content| Ok(content.len()))
}

pub fn default() -> Result<(), String> {
    if !Path::new(&GPG_PATH.to_string()).exists() {
        fs::create_dir_all(GPG_PATH).map_err(|_| format!("create {:?} failed", GPG_PATH))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // #[test]
    // fn test_export_base64() {
    //     let _lock = FILE_LOCK.write(); // 获取文件锁
    //     let expected_content = "test_content".to_string();

    //     // 创建测试用的 keyring 文件
    //     let keyring_path = GPG_KEYRING.to_string();
    //     fs::write(&keyring_path, &expected_content).unwrap();

    //     // 测试 export_base64() 函数
    //     let result = export_base64();
    //     assert!(result.is_ok(), "export_base64() 函数测试失败");
    //     assert_eq!(
    //         result.unwrap(),
    //         expected_content,
    //         "export_base64() 函数返回的数据不正确"
    //     );

    //     // 清除测试用的 keyring 文件
    //     fs::remove_file(keyring_path).unwrap();
    // }

    // #[test]
    // fn test_size_base64() {
    //     let _lock = FILE_LOCK.write(); // 获取文件锁
    //     let expected_content = "test_content".to_string();
    //     let expected_size = expected_content.len();

    //     // 创建测试用的 keyring 文件
    //     let keyring_path = GPG_KEYRING.to_string();
    //     fs::write(&keyring_path, &expected_content).unwrap();

    //     // 测试 size_base64() 函数
    //     let result = size_base64();
    //     assert!(result.is_ok(), "size_base64() 函数测试失败");
    //     assert_eq!(
    //         result.unwrap(),
    //         expected_size,
    //         "size_base64() 函数返回的数据不正确"
    //     );

    //     // 清除测试用的 keyring 文件
    //     fs::remove_file(keyring_path).unwrap();
    // }

    #[test]
    fn test_default() {
        // 测试 default() 函数
        let result = default();
        assert!(result.is_ok(), "default() 函数测试失败");
    }
}
