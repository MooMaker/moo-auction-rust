use crate::{auction::MooAuction, Client, Clients, MOOAUCTION};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};

pub async fn client_connection(ws: WebSocket, clients: Clients) {
    println!("establishing client connection... {:?}", ws);

    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            println!("error sending websocket msg: {}", e);
        }
    }));

    let uuid = Uuid::new_v4().simple().to_string();

    let new_client = Client {
        client_id: uuid.clone(),
        sender: Some(client_sender),
    };

    clients.lock().await.insert(uuid.clone(), new_client);

    clients.lock().await.remove(&uuid);
    println!("{} disconnected", uuid);
}

async fn client_msg(client_id: &str, msg: Message, clients: &Clients) {
    println!("received message from {}: {:?}", client_id, msg);

    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    if message == "ping" || message == "ping\n" {
        let locked = clients.lock().await;
        match locked.get(client_id) {
            Some(v) => {
                if let Some(sender) = &v.sender {
                    println!("sending pong");
                    let _ = sender.send(Ok(Message::text("pong")));
                }
            }
            None => return,
        }
        return;
    }
    // If it's not ping we assume it's a solution
    else {
        let locked = clients.lock().await;

        // TODO: implement json validation
        // TODO: send solution to auction
        // MOOAUCTION.add_bid();

        let reply_text = "Solution received";

        match locked.get(client_id) {
            Some(v) => {
                if let Some(sender) = &v.sender {
                    println!("sending reply");
                    let _ = sender.send(Ok(Message::text(reply_text)));
                }
            }
            None => return,
        }
        return;
    };
}

pub async fn publish_auction(auction_json: String, clients: &Clients) {
    println!("publishing COW auction to market makers");
    // Iterate over the clients
    for (_, client) in clients.lock().await.iter() {
        if let Some(sender) = &client.sender {
            // Send the auction JSON message to the client
            let _ = sender.send(Ok(Message::text(auction_json.clone())));
        }
    }
}
