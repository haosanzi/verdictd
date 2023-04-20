use std::fs;
use std::io::prelude::*;
use std::path::Path;

pub fn export_string(name: &str) -> Result<String, String> {
    fs::File::open(name)
        .map_err(|e| e.to_string())
        .and_then(|mut file| {
            let mut contents = String::new();
            let res = file
                .read_to_string(&mut contents)
                .map_err(|e| e.to_string())
                .and_then(|_| Ok(contents));
            res
        })
}

pub fn export_raw(name: &str) -> Result<Vec<u8>, String> {
    fs::File::open(name)
        .map_err(|e| e.to_string())
        .and_then(|mut file| {
            let mut contents = Vec::new();
            let res = file
                .read_to_end(&mut contents)
                .map_err(|e| e.to_string())
                .and_then(|_| Ok(contents));
            res
        })
}

pub fn export_base64(name: &str) -> Result<String, String> {
    export_raw(name)
        .map_err(|e| e.to_string())
        .and_then(|contents| Ok(base64::encode(contents)))
}

pub fn write(src: &str, content: &str) -> Result<(), String> {
    // Open the file in write-only mode
    // If the file with the same name already exists, it will be overwritten
    fs::File::create(src)
        .map_err(|e| e.to_string())
        .and_then(|mut file| {
            let res = file
                .write_all(content.as_bytes())
                .map_err(|e| e.to_string())
                .and_then(|_| Ok(()));
            res
        })
}

pub fn set(name: &str, content: &str) -> Result<(), String> {
    let src = name;
    let bak = name.clone().to_owned() + ".bak";

    if Path::new(&src).exists() {
        fs::copy(&src, &bak).unwrap();
    }

    write(&src, content)
        .map_err(|e| {
            if Path::new(&bak).exists() {
                fs::copy(&bak, &src).unwrap();
            }
            e
        })
        .and_then(|_| {
            if Path::new(&bak).exists() {
                fs::remove_file(&bak).unwrap();
            }
            Ok(())
        })
}

pub fn size(name: &str) -> Result<usize, String> {
    fs::metadata(name)
        .map_err(|e| e.to_string())
        .and_then(|metadata| Ok(metadata.len() as usize))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_FILE_CONTENT: &[u8] = b"test_file_content";
    const TEST_FILE_CONTENT_STR: &str = "test_file_content";
    const TEST_FILE_NAME: &str = "test_file.txt";

    fn cleanup_test_files() {
        let _ = fs::remove_file(TEST_FILE_NAME);
        let _ = fs::remove_file(TEST_FILE_NAME.to_owned() + ".bak");
    }

    // #[test]
    // fn test_export_string() {
    //     cleanup_test_files();

    //     // Write test file
    //     let mut file = fs::File::create(TEST_FILE_NAME).unwrap();
    //     let _ = file.write_all(TEST_FILE_CONTENT).unwrap();

    //     // Test export_string function
    //     let result = export_string(TEST_FILE_NAME);
    //     assert!(result.is_ok());
    //     assert_eq!(result.unwrap(), TEST_FILE_CONTENT_STR);

    //     cleanup_test_files();
    // }

    // #[test]
    // fn test_export_raw() {
    //     cleanup_test_files();

    //     // Write test file
    //     let mut file = fs::File::create(TEST_FILE_NAME).unwrap();
    //     let _ = file.write_all(TEST_FILE_CONTENT).unwrap();

    //     // Test export_raw function
    //     let result = export_raw(TEST_FILE_NAME);
    //     assert!(result.is_ok());
    //     assert_eq!(result.unwrap(), TEST_FILE_CONTENT.to_vec());

    //     cleanup_test_files();
    // }

    // #[test]
    // fn test_export_base64() {
    //     cleanup_test_files();

    //     // Write test file
    //     let mut file = fs::File::create(TEST_FILE_NAME).unwrap();
    //     let _ = file.write_all(TEST_FILE_CONTENT).unwrap();

    //     // Test export_base64 function
    //     let result = export_base64(TEST_FILE_NAME);
    //     assert!(result.is_ok());
    //     assert_eq!(result.unwrap(), base64::encode(TEST_FILE_CONTENT));

    //     cleanup_test_files();
    // }

    #[test]
    fn test_write() {
        cleanup_test_files();

        // Test write function
        let result = write(TEST_FILE_NAME, TEST_FILE_CONTENT_STR);
        assert!(result.is_ok());

        // Read test file to check if content is written correctly
        let result = fs::read(TEST_FILE_NAME);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), TEST_FILE_CONTENT.to_vec());

        cleanup_test_files();
    }

    // #[test]
    // fn test_set() {
    //     cleanup_test_files();

    //     // Write test file
    //     let mut file = fs::File::create(TEST_FILE_NAME).unwrap();
    //     let _ = file.write_all(b"original_content").unwrap();

    //     // Test set function
    //     let result = set(TEST_FILE_NAME, TEST_FILE_CONTENT_STR);
    //     assert!(result.is_ok());

    //     // Check file content to see if set() function actually writes content to file
    //     let result = fs::read(TEST_FILE_NAME);
    //     assert!(result.is_ok());
    //     assert_eq!(result.unwrap(), TEST_FILE_CONTENT.to_vec());

    //     // Check backup file to see if set() function actually creates a backup file
    //     let result = fs::read(TEST_FILE_NAME.to_owned() + ".bak");
    //     assert!(result.is_ok());
    //     assert_eq!(result.unwrap(), b"original_content".to_vec());

    //     // Test set function with error handling
    //     let result = set("/", TEST_FILE_CONTENT_STR);
    //     assert!(result.is_err());

    //     cleanup_test_files();
    // }

    // #[test]
    // fn test_size() {
    //     cleanup_test_files();

    //     // Write test file
    //     let mut file = fs::File::create(TEST_FILE_NAME).unwrap();
    //     let _ = file.write_all(TEST_FILE_CONTENT).unwrap();

    //     // Test size function
    //     let result = size(TEST_FILE_NAME);
    //     assert!(result.is_ok());
    //     assert_eq!(result.unwrap(), TEST_FILE_CONTENT.len());

    //     cleanup_test_files();
    // }
}
