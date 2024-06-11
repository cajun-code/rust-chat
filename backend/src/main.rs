use std::{collections::HashMap, sync::atomic::{AtomicUsize, Ordering}};

use rocket::{futures::{stream::SplitSink, SinkExt, StreamExt}, tokio::sync::Mutex, State};
use rocket_ws::{stream::DuplexStream, Channel, Message, WebSocket};
use chat_lib::{ChatMessage, WebSocketMessage, WebSocketMessageType};

#[macro_use] extern crate rocket;

static USER_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);


struct User{
    id: usize,
    name: String,
    sink: SplitSink<DuplexStream, Message>,
}

#[derive(Default)]
struct ChatRoom{
    connections: Mutex<HashMap<usize, User>>
}


impl ChatRoom{
    pub async fn add_user(&self, id: usize, sink: SplitSink<DuplexStream, Message>){
        let mut conns = self.connections.lock().await; 
            let user = User{
                id,
                name: format!("User #{}", id),
                sink,
            };
            conns.insert(id, user);
    }

    pub async fn remove_user(&self, id: usize){
        let mut conns = self.connections.lock().await; 
            conns.remove(&id);
    }

    pub async fn broadcast(&self, message: Message, author_id: usize){
        let chat_msg = ChatMessage{
            message: message.to_string(),
            author: format!("User #{}", author_id),
            created_at: chrono::Utc::now().naive_utc(),
        };

        let envlope = WebSocketMessage{
            message_type: WebSocketMessageType::NewMessage,
            message: Some(chat_msg),
            users: None,
        };

        let msg = serde_json::to_string(&envlope).unwrap();
        let mut conns = self.connections.lock().await; 
        for (_id, user) in conns.iter_mut() {
            let _ = user.sink.send(Message::Text(msg.clone())).await;
        }
    }

    pub async fn broadcast_user_list(&self){
        let mut conns = self.connections.lock().await; 
            let envlope = WebSocketMessage{
                message_type: WebSocketMessageType::UserList,
                message: None,
                users: Some(conns.values().map(|k| k.name.clone()).collect()),
            };
            let msg = serde_json::to_string(&envlope).unwrap();
            for (_id, user) in conns.iter_mut() {
                let _ = user.sink.send(Message::Text(msg.clone())).await;
            }
    }

}


#[get("/")]
fn chat<'r>(ws:WebSocket, room_state: &'r State<ChatRoom>)-> Channel<'r>{
    ws.channel(move |mut stream| Box::pin(async move {
        let user_id = USER_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        let(mut sink, mut ws_stream) = stream.split();
        room_state.add_user(user_id, sink).await;
        room_state.broadcast_user_list().await;

        while let Some(message) = ws_stream.next().await {
            room_state.broadcast(message?, user_id).await;
        }
        room_state.remove_user(user_id).await;
        room_state.broadcast_user_list().await;
        Ok(())
    }))
} 


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![chat]).manage(ChatRoom::default())
}
