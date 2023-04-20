use crate::client_api;
use tonic::transport::Server;

use clientApi::gpg_service_server::GpgServiceServer;
use clientApi::image_service_server::ImageServiceServer;
use clientApi::key_manager_service_server::KeyManagerServiceServer;
use clientApi::opa_service_server::OpaServiceServer;
use client_api::key_provider::keyProvider::key_provider_service_server::KeyProviderServiceServer;

pub mod clientApi {
    tonic::include_proto!("clientapi");
}

pub async fn server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let addr = addr.parse()?;
    let gpg_service = client_api::gpg::gpgService::default();
    let image_service = client_api::image::imageService::default();
    let key_manager_service = client_api::key_manager::keyManagerService::default();
    let key_provider_service = client_api::key_provider::keyProviderService::default();
    let opa_service = client_api::opa::opaService::default();

    Server::builder()
        .add_service(GpgServiceServer::new(gpg_service))
        .add_service(ImageServiceServer::new(image_service))
        .add_service(KeyManagerServiceServer::new(key_manager_service))
        .add_service(KeyProviderServiceServer::new(key_provider_service))
        .add_service(OpaServiceServer::new(opa_service))
        .serve(addr)
        .await?;

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use tonic::transport::Channel;
//     // use crate::client_api::api::clientApi::gpg_service_client::GpgServiceClient;

//     use crate::client_api::api::tests::GpgServiceClient;
//     use crate::client_api::api::clientApi::gpg_service_client::GpgServiceClient;

//     #[tokio::test]
//     async fn test_server() -> Result<(), Box<dyn std::error::Error>> {
//         // Start server in separate task
//         let addr = "127.0.0.1:0".to_string(); // Random available port on localhost
//         let server_handle = tokio::spawn(async move {
//             server(&addr).await.unwrap();
//         });

//         // Wait for server to start
//         let server_addr = server_handle
//             .try_join(tokio::time::timeout(std::time::Duration::from_secs(5), async {
//                 loop {
//                     // Try to connect to server address until success or timeout
//                     if let Ok(mut channel) =
//                         Channel::from_shared(format!("http://{}", addr)).unwrap().connect().await
//                     {
//                         channel.shutdown().await.unwrap();
//                         return Ok(addr);
//                     }
//                 }
//             }))
//             .unwrap()
//             .unwrap();

//         // Test client API communication with server
//         let mut client =
//             client_api::gpg_service_client::GpgServiceClient::connect(format!("http://{}", server_addr)).await?;
//         let request = tonic::Request::new(client_api::gpg::DecryptRequest {
//             ciphertext: "test".to_string(),
//         });
//         let response = client.decrypt(request).await?;
//         assert_eq!(response.into_inner().plaintext, "test");

//         // Shut down server
//         server_handle.abort();
//         Ok(())
//     }
// }
