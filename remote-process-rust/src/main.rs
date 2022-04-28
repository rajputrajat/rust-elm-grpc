use communication::{
    responder_server::{Responder, ResponderServer},
    RequestMsg, RespnseMsg,
};

mod communication {
    tonic::include_proto!("communication");
}

struct RemoteResponder;

// #[tonic::async_trait]
// impl

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
