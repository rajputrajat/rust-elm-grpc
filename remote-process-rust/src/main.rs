use communication::{
    responder_server::{Responder, ResponderServer},
    RequestMsg, ResponseMsg,
};
use tonic::{Request, Response, Status};

mod communication {
    tonic::include_proto!("communication");
}

struct RemoteResponder;

#[tonic::async_trait]
impl Responder for RemoteResponder {
    async fn calling_out(&self, req: Request<RequestMsg>) -> Result<Response<ResponseMsg>, Status> {
        println!("request: {req:?}");
        Ok(Response::new(ResponseMsg {
            loud_reply: "We are fine. you take care.".to_owned(),
        }))
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
