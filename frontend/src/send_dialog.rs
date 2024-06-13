use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SendDialogProps {
    pub on_send: Callback<String>,
    pub on_username_change: Callback<String>,
    pub username: String,
}

#[function_component(SendDialog)]
pub fn send_dialog(props: &SendDialogProps) -> Html {
    let new_message_handle = use_state(String::default);
    let new_message = (*new_message_handle).clone();
    let cloned_new_message_handle = new_message_handle.clone();
    let on_new_message_change = Callback::from(move |e:Event|{
        let input: HtmlTextAreaElement = e.target_unchecked_into();
        cloned_new_message_handle.set(input.value());
    });
    let cloned_new_message = new_message.clone();
    let cloned_send = props.on_send.clone();
    let on_message_send = Callback::from( move |_: MouseEvent| {
        cloned_send.emit(cloned_new_message.clone());
        new_message_handle.set(String::default());
    });
    let username = props.username.clone();
    let username_callback = props.on_username_change.clone();
    let on_username_click = Callback::from( move |_: MouseEvent| {
        //DialogService::prompt("Enter a new username:", props.username.as_str());
        if let Some(username)  = gloo_dialogs::prompt("Enter a new username:", Some(username.as_str())){
            username_callback.emit(username);
        }
        
    });
    
    html! {
        <div>
            <div class="row"><a href="javascript:void(0)" onclick={on_username_click}>{&props.username}</a></div>
            <div class="input-group">
                <textarea class="form-control" onchange={on_new_message_change} value={new_message}></textarea>
                <button class="btn btn-primary" type="submit" onclick={on_message_send}>{"Send"}</button>
            </div>
        </div>
        
    }
}