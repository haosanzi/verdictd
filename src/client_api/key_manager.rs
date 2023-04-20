use crate::client_api::api;
use crate::resources::directory_key_manager;
use base64;
use rand::*;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use api::clientApi::key_manager_service_server::KeyManagerService;
use api::clientApi::{CreateKeyRequest, CreateKeyResponse};
use api::clientApi::{DeleteKeyRequest, DeleteKeyResponse};
use api::clientApi::{GetKeyRequest, GetKeyResponse};

#[derive(Debug, Default)]
pub struct keyManagerService {}

#[tonic::async_trait]
impl KeyManagerService for keyManagerService {
    async fn create_key(
        &self,
        _request: Request<CreateKeyRequest>,
    ) -> Result<Response<CreateKeyResponse>, Status> {
        let kid = Uuid::new_v4().to_string();
        // generate a new key file with a new random key
        let mut key: [u8; 32] = [0; 32];
        rand::rngs::OsRng.fill_bytes(&mut key);
        let res = directory_key_manager::set_key(&kid, &key)
            .and_then(|_| {
                let res = CreateKeyResponse {
                    status: "OK".as_bytes().to_vec(),
                    uuid: kid.into_bytes(),
                };
                Ok(res)
            })
            .unwrap_or_else(|_| CreateKeyResponse {
                status: "Greate key failed".as_bytes().to_vec(),
                uuid: "".as_bytes().to_vec(),
            });

        Ok(Response::new(res))
    }

    async fn get_key(
        &self,
        request: Request<GetKeyRequest>,
    ) -> Result<Response<GetKeyResponse>, Status> {
        let kid = String::from_utf8(request.into_inner().uuid)
            .unwrap_or_else(|_| "00000000-0000-0000-0000-000000000000".to_string());
        info!("kid: {}", kid);

        let res = directory_key_manager::get_key(&kid)
            .and_then(|data| {
                let res = GetKeyResponse {
                    status: "OK".as_bytes().to_vec(),
                    key: base64::encode(data).into_bytes(),
                };
                Ok(res)
            })
            .unwrap_or_else(|_| GetKeyResponse {
                status: "key is not exist".as_bytes().to_vec(),
                key: "".as_bytes().to_vec(),
            });

        Ok(Response::new(res))
    }

    async fn delete_key(
        &self,
        _request: Request<DeleteKeyRequest>,
    ) -> Result<Response<DeleteKeyResponse>, Status> {
        let res = DeleteKeyResponse {
            status: "Not implemented".as_bytes().to_vec(),
        };
        Ok(Response::new(res))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Read;
    use std::path::PathBuf;
    use tempdir::TempDir;

    // #[tokio::test]
    // async fn test_create_and_get_key() {
    //     let temp_dir = TempDir::new("test_dir").unwrap();
    //     let service = keyManagerService {};

    //     // Create key
    //     let request = CreateKeyRequest {};
    //     let response = service.create_key(Request::new(request)).await.unwrap();
    //     let uuid = String::from_utf8(response.get_ref().uuid.clone()).unwrap();
    //     assert_eq!(response.get_ref().status, b"OK");

    //     // Get key
    //     let request = GetKeyRequest {
    //         uuid: response.get_ref().uuid.clone(),
    //     };
    //     let response = service.get_key(Request::new(request)).await.unwrap();
    //     assert_eq!(response.get_ref().status, b"OK");

    //     // Verify key
    //     let key_str = String::from_utf8(response.get_ref().key.clone()).unwrap();
    //     let key_bytes = base64::decode(key_str).unwrap();
    //     let key_file_path = PathBuf::from(temp_dir.path()).join(uuid);
    //     let mut key_file = fs::File::open(key_file_path.clone()).unwrap();
    //     let mut key_content = Vec::new();
    //     key_file.read_to_end(&mut key_content).unwrap();
    //     assert_eq!(key_bytes, key_content);

    //     // Clean up
    //     fs::remove_file(key_file_path).unwrap();
    // }

    #[tokio::test]
    async fn test_get_key_not_exist() {
        let service = keyManagerService {};
        let request = GetKeyRequest {
            uuid: b"notexist".to_vec(),
        };
        let response = service.get_key(Request::new(request)).await.unwrap();
        assert_eq!(response.get_ref().status, b"key is not exist");
    }
}
