tonic::include_proto!("communication");

use grpc_web_client::Client;
use responder_client::ResponderClient;
use tonic::Request;

pub async fn send_msg_to_remote() -> Result<(), Error> {
    let client = Client::new("http://127.0.0.1:8080".to_owned());
    let client = ResponderClient::new(client);

    // let resp = client
    //     .calling_out(Request::new(RequestMsg {
    //         please_msg: "how are you, Server?".into(),
    //     }))
    //     .await?;
    Ok(())
}

#[derive(Debug)]
enum Error {}
