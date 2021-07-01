use crate::{Client, Clients};
use futures::{FutureExt, StreamExt};
use serde::Deserialize;
use serde_json;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};

#[derive(Deserialize, Debug)]
pub struct TopicsRequest {
    topics: Vec<String>,
}

pub async fn client_connection(ws: WebSocket, id: String, clients: Clients, mut client: Client) {
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();
    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));

    client.sender = Some(client_sender);
    clients.write().await.insert(id.clone(), client);

    println!("{} connected", id);

    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("error receiving ws message for id: {}): {}", id.clone(), e);
                break;
            }
        };
        client_msg(&id, msg, &clients).await;
    }

    clients.write().await.remove(&id);
    println!("{} disconnected", id);
}

//handle a message from the client
async fn client_msg(id: &str, msg: Message, clients: &Clients) {
    //convert message to a string
    println!("received message from {}: {:?}", id, msg);
    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    //split message by spaces
    let split_msg: Vec<&str> = message.split(' ').collect();

    //check the first word to check operand
    match split_msg[0] {
        "ADD" => {
            if split_msg.len() == 2 {
                add_topic(id, clients, split_msg[1]).await
            }
        }
        "TELL" => {
            if split_msg.len() >= 3 {
                broadcast_to_topic(
                    clients,
                    split_msg[1],
                    &split_msg[2..]
                        .iter()
                        .map(|i| i.to_string())
                        .collect::<String>(),
                )
                .await;
            }
        }
        _ => println!("{} sent unknown operator {}", id, split_msg[0]),
    }
}

async fn add_topic(id: &str, clients: &Clients, topic: &str) {
    let mut locked = clients.write().await;
    match locked.get_mut(id) {
        Some(v) => {
            v.topics.push(topic.to_string());
        }
        None => return,
    };
}

async fn broadcast_to_topic(clients: &Clients, topic: &str, message: &str) {
    let mut locked = clients.write().await;
    locked
        .iter_mut()
        .filter(|(_, client)| client.topics.contains(&topic.to_string()))
        .for_each(|(_, client)| {
            if let Some(sender) = &client.sender {
                let _ = sender.send(Ok(Message::text(message.to_string().clone())));
            }
        });
}
