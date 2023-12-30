use tonic::{transport::Server, Request, Response, Status};

use trundle::trundle_api_server::{TrundleApi, TrundleApiServer};

use blocks::{GenerateBlockRequest, GenerateBlockResponse};

tonic::include_proto!("trundleapi");

#[derive(Debug, Default)]
pub struct TrundleApi {}

#[tonic::async_trait]
impl Api for TrundleApi {
    async fn generate_block(&self, request: Request<GenerateBlockRequest>) -> Result<Response<GenerateBlockResponse>, Status> {
        println!("Got a request: {:?}", request);

        let message = format!("Hello {}!", request.into_inner().name).into();

        let reply = GenerateBlockResponse {
            message,
        };

        return Ok(Response::new(reply));
    }
}
