mod endpoint;
mod transaction;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   let address = String::from("127.0.0.1:50051");
   println!("Start the server on: {:?}", address);
   endpoint::server::start_server(address).await?;

   Ok(())
}