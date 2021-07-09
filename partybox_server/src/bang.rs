use crate::Clients;
use tokio::sync::RwLock;
use uuid::Uuid;
use warp::ws::Message;

#[derive(Debug, Clone)]
pub struct Room {
    pub room_id: String,
    pub name: String,
    pub user_count: usize,
}

#[derive(Debug, Clone)]
pub struct BangPlayer {
    pub user_id: usize,
    pub room_id: String,
    pub role: String,
    pub health: usize,
}

type Rooms = RwLock<Vec<Room>>;
type BangPlayers = RwLock<Vec<BangPlayer>>;

lazy_static! {
    static ref BANG_ROOMS: Rooms = Rooms::new(Vec::new());
    static ref BANG_PLAYERS: BangPlayers = BangPlayers::new(Vec::new());
}

pub async fn bang_message_handler(id: &str, args: &[&str], clients: &Clients) {
    //check the first word to check operand
    match args[0] {
        //register a player in the bang player datastructure format that has the data needed for the game
        "PLAY" => {
            let client = clients.read().await.get(id).cloned().unwrap();
            BANG_PLAYERS.write().await.push(BangPlayer {
                user_id: client.user_id,
                room_id: "".to_string(),
                role: "".to_string(),
                health: 0,
            });
        }
        //join a prexisting room
        "JOIN" => {
            if args.len() >= 2 {
                //get the client that wants to join a room
                let client = clients.read().await.get(id).cloned().unwrap();
                let id: String;

                //find a room with the matching name given in the ws message
                match BANG_ROOMS
                    .write()
                    .await
                    .iter_mut()
                    .find(|x| x.name == args[1..].iter().map(|i| i.to_string()).collect::<String>())
                {
                    //if it exists store the UUID of the room and increase the player count
                    Some(x) => {
                        x.user_count += 1;
                        id = x.room_id.clone();
                    }
                    //if not respond to the client accordingly
                    None => {
                        client
                            .sender
                            .unwrap()
                            .send(Ok(Message::text("RESPONSE ERROR The room requested does not exist")))
                            .unwrap();
                        return
                    }
                }

                //find this clients bangplayer struct and set its room_id to be the room requested
                match BANG_PLAYERS
                    .write()
                    .await
                    .iter_mut()
                    .find(|x| x.user_id == client.user_id)
                {
                    Some(x) => x.room_id = id.clone(),
                    //if there is no bangplayer struct tell the client
                    None => {
                        client
                            .sender
                            .unwrap()
                            .send(Ok(Message::text(
                                "RESPONSE ERROR You must join the bang! game before joining/creating a room (send BANG PLAY via ws)",
                            ))).unwrap();
                        return;
                    }
                }
                
                //confirms that joining was successful
                client
                    .sender
                    .unwrap()
                    .send(Ok(Message::text("RESPONSE JOIN")))
                    .unwrap();
            }
        }
        "CREATE" => {
            if args.len() >= 2 {
                //get the client that wants to join a room
                let client = clients.read().await.get(id).cloned().unwrap();

                //create a new room in the room vector with a uuid
                let id = Uuid::new_v4().simple().to_string();
                BANG_ROOMS.write().await.push(Room {
                    room_id: id.clone(),
                    name: args[1..].iter().map(|i| i.to_string()).collect::<String>(),
                    user_count: 1,
                });

                //add a player to the room, same as in the join command
                match BANG_PLAYERS
                    .write()
                    .await
                    .iter_mut()
                    .find(|x| x.user_id == client.user_id)
                {
                    Some(x) => x.room_id = id.clone(),
                    None => {
                        client
                            .sender
                            .unwrap()
                            .send(Ok(Message::text(
                                "RESPONSE ERROR You must join the bang! game before joining/creating a room",
                            )))
                            .unwrap();
                        return;
                    }
                }

                //confirms that creating was successful
                client
                    .sender
                    .unwrap()
                    .send(Ok(Message::text("RESPONSE CREATE")))
                    .unwrap();
            }
        }
        "LISTROOMS" => {
            //get the client that wants to join a room
            let client = clients.read().await.get(id).cloned().unwrap();

            let mut finalstr: String = "RESPONSE LISTROOMS\n".to_string();
            for room in BANG_ROOMS.read().await.iter() {
                finalstr.push_str(&format!(
                    "name: {}, id: {}, players: {}\n",
                    room.name, room.room_id, room.user_count
                ));
            }
            client
                .sender
                .unwrap()
                .send(Ok(Message::text(finalstr)))
                .unwrap();
        }
        _ => {
            return
        }
    }
}
