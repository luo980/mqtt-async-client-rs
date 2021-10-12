use std::time::Duration;
use serde::{Serialize, Deserialize};

use log::error;
use mqtt_async_client::{
    client::{
        Client,
        Publish as PublishOpts,
        QoS,
    },
    Result,
};

struct ClientArgs{
    url:            String,
}

struct PubArgs{
    payload:        String,
    topic:          String,
    qos:            u8,
}

#[tokio::main]
async fn main () {
    let serialized_payload;
    println!("Here");
    let client_args = ClientArgs{
        url:        "mqtt://192.168.1.5:1883".to_string(),
    };

    let pub_args = PubArgs{
        payload:    "Online".to_string(),
        topic:      "/Online".to_string(),
        qos:        1,
    };

    let temperature_args = PubArgs{
        payload:    "asdsa".to_string(),
        topic:      "",
        qos:        1,
    };

    println!("Here");
    if let Err(e) = publish(pub_args, client_args).await{
        error!("Error is {:?}", e)
    };

}

fn client_from_args(args: ClientArgs) -> Result<Client> {
    let mut b = Client::builder();
    if let Err(e) = b.set_url_string(&args.url){
        println!("Error is {:?}", e);
    }
    b.set_connect_retry_delay(Duration::from_secs(1));
    println!("Here");
    b.build()
}

async fn publish (pubargs: PubArgs, args: ClientArgs) -> Result<()> {
    println!("Here");
    let mut client = client_from_args(args)?;
    println!("Here");
    client.connect().await?;
    let mut p = PublishOpts::new(pubargs.topic, pubargs.payload.as_bytes().to_vec());
    p.set_qos(int_to_qos(pubargs.qos));
    let results_client = client.publish(&p);
    if let Err(e) = results_client.await{
        error!("Error is {}", e);
    }
    Ok(())
}

fn int_to_qos(qos: u8) -> QoS {
    match qos {
        0 => QoS::AtMostOnce,
        1 => QoS::AtLeastOnce,
        2 => QoS::ExactlyOnce,
        _ => panic!("Not reached"),
    }
}
