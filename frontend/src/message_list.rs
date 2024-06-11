use chat_lib::ChatMessage;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props{
    pub messages: Vec<ChatMessage>,
}


#[function_component(MessageList)]
pub fn message_list(props : &Props) -> Html{
    html!{
        <div id="chat" class="list-group">
            { for props.messages.iter().map(|cm| html!{
                <div class="list-group-item list-group-item-action">
                    <div class="d-flex w-100 justify-content-between">
                        <h5 class="mb-1">{cm.author.clone()}</h5>
                        <small>{cm.created_at.format("%Y-%m-%d %H:%M:%S").to_string()}</small>
                    </div>
                    <p class="mb-1">{cm.message.clone()}</p>
                </div>
            }) }
        </div>
    }
}