use std::fs;
use std::io;

const VERDICTD_KEY_PATH: &str = "/opt/verdictd/keys/";

pub fn get_key(kid: &String) -> Result<Vec<u8>, io::Error> {
    let path = VERDICTD_KEY_PATH.to_string() + kid;
    info!("get key from keyFile: {}", path);

    let data = fs::read(path);
    match data {
        Ok(key) => Ok(key),
        Err(e) => {
            error!("Get kid:{}'s key failed, err: {}", kid, e.to_string());
            Err(e)
        }
    }
}

pub fn set_key(kid: &String, key: &[u8]) -> std::io::Result<()> {
    let path = VERDICTD_KEY_PATH.to_string() + kid;
    info!("set key for keyFile: {}", path);

    fs::write(path, key).expect("Unable to write file");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::ErrorKind;

    fn cleanup_test_files() {
        let _ = fs::remove_file(VERDICTD_KEY_PATH);
    }

    #[test]
    fn test_get_key() {
        let kid = String::from("test_key");
        let key_content = b"test_key_content".to_vec();
        cleanup_test_files();
        fs::create_dir_all(&VERDICTD_KEY_PATH).expect("Unable to create directory");
        let path = VERDICTD_KEY_PATH.to_string() + &kid;
        fs::write(&path, &key_content).expect("Unable to write file");

        let key = get_key(&kid);
        assert_eq!(key.unwrap(), key_content);

        cleanup_test_files()
    }

    #[test]
    fn test_set_key() {
        let kid = String::from("test_key");
        let key_content = b"test_key_content".to_vec();
        cleanup_test_files();
        fs::create_dir_all(&VERDICTD_KEY_PATH).expect("Unable to create directory");
        let path = VERDICTD_KEY_PATH.to_string() + &kid;

        let set_res = set_key(&kid, &key_content);
        assert!(set_res.is_ok());

        let data = fs::read(&path);
        assert!(data.is_ok());
        assert_eq!(data.unwrap(), key_content);

        // Cleanup
        let res = fs::remove_file(&path);
        assert!(res.is_ok());
        cleanup_test_files()
    }
}
