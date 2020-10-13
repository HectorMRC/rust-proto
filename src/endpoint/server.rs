use crate::transaction::traits::{Tx, Body};
use tonic::{transport::Server, Request, Response, Status};
use crate::endpoint::transactions::{request::TxRequest};
use crate::transaction::transaction::Transaction;

// Import the generated rust code into module
pub mod endpoint_proto {
   tonic::include_proto!("endpoint");
}

// Proto generated server traits
use endpoint_proto::endpoint_server::{Endpoint, EndpointServer};

// Proto message structs
use endpoint_proto::{ProtoRequest, ProtoResponse };


#[derive(Default)]
pub struct EndpointImplementation {}

#[tonic::async_trait]
impl Endpoint for EndpointImplementation {
    async fn send( &self, request: Request<ProtoRequest>) -> Result<Response<ProtoResponse>, Status> {
        let msg_ref = request.get_ref();
        let signup = TxRequest::new(
            &msg_ref.msg,
        );
        
        let body: Box<dyn Body> = Box::new(signup);
        let tx = Transaction::new(body);
        let mut tx_signup: Box<dyn Tx> = Box::new(tx);
        tx_signup.execute();

        let response = ProtoResponse {
            msg: "".to_string(),
        };

        Ok(Response::new(response))
    }
}

pub async fn start_server(address: String) -> Result<(), Box<dyn std::error::Error>> {
   let addr = address.parse().unwrap();
   let endpoint_server = EndpointImplementation::default();

   println!("ServiceServer listening on {}", addr);

   Server::builder()
       .add_service(EndpointServer::new(endpoint_server))
       .serve(addr)
       .await?;

   Ok(())
}