use chat_lib::{ChatMessage, WebSocketMessage, WebSocketMessageType};

use yew::prelude::*;
use yew_hooks::use_websocket;

mod message_list;
mod send_dialog;
mod user_list;

use message_list::MessageList;
use send_dialog::SendDialog;
use user_list::UserList;

#[function_component]
fn App() -> Html {
    let messages_handle = use_state(Vec::default);
    let user_handle = use_state(Vec::default);
    let mut cloned_users = (*user_handle).clone();
    let messages = (*messages_handle).clone();
    let users = (*user_handle).clone();
    let ws = use_websocket("ws://localhost:8000".to_string());
    let mut cloned_messages = messages.clone();
    let ws_clone = ws.clone();
    use_effect_with(ws.message, move |ws_message| {
        if let Some(message) = &**ws_message {
            let envelope: WebSocketMessage = serde_json::from_str(&message).unwrap();
            match envelope.message_type {
                WebSocketMessageType::UserList => {
                    let users = envelope.users.expect("User list is missing payload");
                    cloned_users.clear();
                    cloned_users.extend(users);
                    user_handle.set(cloned_users);
                }
                WebSocketMessageType::NewMessage => {
                    let chat_message:ChatMessage = envelope.message.expect("Message is missing payload");
                    cloned_messages.push(chat_message);
                    messages_handle.set(cloned_messages);
                }
            }
            
        }
    });
    
    let on_send_message = Callback::from(move |msg:String| {
        ws_clone.send(msg.clone());
    });
    
    html!{
        <div class="container" >
            <div class="row">
                <div class="col col-3">
                    <UserList users={users} />
                </div>
                <div class="col col-9">
                    <div class="row">
                        <MessageList messages={messages} />
                    </div>
                    <div class="row">
                        <SendDialog on_send={on_send_message} />
                    </div>
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
