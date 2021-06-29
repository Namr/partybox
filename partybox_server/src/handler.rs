use crate::{ws, Client, Clients, Result};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, reply::json, Reply};

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    user_id: usize,
}

#[derive(Serialize, Debug)]
pub struct RegisterResponse {
    url: String,
}

#[derive(Deserialize, Debug)]
pub struct Event {
    topic: String,
    user_id: Option<usize>,
    message: String,
}

pub async fn register_handler(body: RegisterRequest, clients: Clients) -> Result<impl Reply> {
    //create uuid, make and save a client
    let user_id = body.user_id;
    let uuid = Uuid::new_v4().simple().to_string();
    register_client(uuid.clone(), user_id, clients).await;

    //return this uuid in the form of a URL the client can use to communicate with the websocket
    Ok(json(&RegisterResponse {
        url: format!("ws://127.0.0.1:8000/ws/{}", uuid),
    }))
}

//helper function that adds the client to the hashmap using the uuid as the key
async fn register_client(id: String, user_id: usize, clients: Clients) {
    clients.write().await.insert(
        id,
        Client {
            user_id,
            topics: vec![String::from("cats")],
            sender: None,
        },
    );
}

//delete client from hashmap if they are to be unregistered
pub async fn unregister_handler(id: String, clients: Clients) -> Result<impl Reply> {
    clients.write().await.remove(&id);
    Ok(StatusCode::OK)
}

//attempt to establish websocket connection
pub async fn ws_handler(ws: warp::ws::Ws, id: String, clients: Clients) -> Result<impl Reply> {
    let client = clients.read().await.get(&id).cloned();
    //see if this client exists
    match client {
        Some(c) => Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, id, clients, c))),
        None => Err(warp::reject::not_found()),
    }
}

pub async fn health_handler() -> Result<impl Reply> {
    Ok(StatusCode::OK)
}
