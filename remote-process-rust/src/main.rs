use communication::{
    responder_server::{Responder, ResponderServer},
    RequestMsg, ResponseMsg,
};
use local_ip_address::{list_afinet_netifas, Error as LocalIpError};
use std::net::{AddrParseError, IpAddr, SocketAddr};
use tonic::{
    transport::{Error as TonicError, Server},
    Request, Response, Status,
};

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
async fn main() -> Result<(), Error> {
    let addr = format!("{}:50051", get_ip()?).parse::<SocketAddr>()?;
    let responder = RemoteResponder;
    Server::builder()
        .add_service(ResponderServer::new(responder))
        .serve(addr)
        .await?;
    Ok(())
}

#[derive(Debug)]
enum Error {
    OrgIpNotListed,
    LocalIp(LocalIpError),
    AddrParser(AddrParseError),
    Tonic(TonicError),
}

fn get_ip() -> Result<IpAddr, Error> {
    list_afinet_netifas()?
        .iter()
        .find_map(|v| {
            if v.1.to_string().contains("172.") {
                Some(v.1)
            } else {
                None
            }
        })
        .ok_or(Error::OrgIpNotListed)
}

impl From<LocalIpError> for Error {
    fn from(e: LocalIpError) -> Self {
        Error::LocalIp(e)
    }
}

impl From<AddrParseError> for Error {
    fn from(e: AddrParseError) -> Self {
        Error::AddrParser(e)
    }
}

impl From<TonicError> for Error {
    fn from(e: TonicError) -> Self {
        Error::Tonic(e)
    }
}
