use chat_lib::ChatMessage;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew_hooks::use_websocket;

#[function_component]
fn App() -> Html {
    let messages_handle = use_state(Vec::default);
    let new_message_handle = use_state(String::default);
    let new_message = (*new_message_handle).clone();
    let messages = (*messages_handle).clone();
    let ws = use_websocket("ws://localhost:8000".to_string());
    let mut cloned_messages = messages.clone();
    let ws_clone = ws.clone();
    use_effect_with(ws.message, move |ws_message| {
        if let Some(message) = &**ws_message {
            let chat_message:ChatMessage = serde_json::from_str(&message).unwrap();
            cloned_messages.push(chat_message);
            messages_handle.set(cloned_messages);
        }
    });
    let new_msg_handle_clone = new_message_handle.clone();
    let on_new_message_change = Callback::from(move |e:Event|{
        let input: HtmlTextAreaElement = e.target_unchecked_into();
        new_msg_handle_clone.set(input.value());
    });
    
    let cloned_new_message = new_message.clone();
    let on_message_send = Callback::from(move |_: MouseEvent| {
        ws_clone.send(cloned_new_message.clone());
        new_message_handle.set(String::default());
    });
    

    html!{
        <div class="container" >
            <div class="row">
            <div id="chat" class="list-group">
                { for messages.iter().map(|cm| html!{
                    <div class="list-group-item list-group-item-action">
                        <div class="d-flex w-100 justify-content-between">
                            <h5 class="mb-1">{cm.author.clone()}</h5>
                            <small>{cm.created_at.format("%Y-%m-%d %H:%M:%S").to_string()}</small>
                        </div>
                        <p class="mb-1">{cm.message.clone()}</p>
                    </div>
                }) }
            </div>
            </div>
            <div class="row">
                <div class="input-group">
                    <textarea class="form-control" onchange={on_new_message_change} value={new_message.clone()}></textarea>
                    <button class="btn btn-primary" type="submit" onclick={on_message_send}>{"Send"}</button>
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
