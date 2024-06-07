use std::{collections::HashMap, sync::atomic::{AtomicUsize, Ordering}};

use rocket::{futures::{stream::SplitSink, SinkExt, StreamExt}, tokio::sync::Mutex, State};
use rocket_ws::{stream::DuplexStream, Channel, Message, WebSocket};

#[macro_use] extern crate rocket;

static USER_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Default)]
struct ChatRoom{
    connections: Mutex<HashMap<usize, SplitSink<DuplexStream, Message>>>
}

impl ChatRoom{
    pub async fn add_user(&self, id: usize, sink: SplitSink<DuplexStream, Message>){
        let mut conns = self.connections.lock().await; 
            conns.insert(id, sink);
    }

    pub async fn remove_user(&self, id: usize){
        let mut conns = self.connections.lock().await; 
            conns.remove(&id);
    }

    pub async fn broadcast(&self, msg: Message){
        let mut conns = self.connections.lock().await; 
        for (_id, sink) in conns.iter_mut() {
            let _ = sink.send(msg.clone()).await;
        }
    }

}


#[get("/")]
fn chat<'r>(ws:WebSocket, room_state: &'r State<ChatRoom>)-> Channel<'r>{
    ws.channel(move |mut stream| Box::pin(async move {
        let user_id = USER_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        let(mut sink, mut ws_stream) = stream.split();
        room_state.add_user(user_id, sink).await;

        while let Some(message) = ws_stream.next().await {
            room_state.broadcast(message?).await;
        }
        room_state.remove_user(user_id).await;
        Ok(())
    }))
} 


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![chat]).manage(ChatRoom::default())
}
