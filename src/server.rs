use tonic::transport::Server;

mod api::blocks;

pub mod trundle {
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let api = ApiServer::new(trundle_api::TrundleApi::default());

    Server::builder()
        .add_service(api)
        .serve(addr)
        .await?;

    return Ok(());
}
