
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props{
    pub users: Vec<String>,
}


#[function_component(UserList)]
pub fn users_list(props : &Props) -> Html{
    html!{
        <div id="users" class="list-group">
            { for props.users.iter().map(|user_name| html!{
                <div class="list-group-item list-group-item-action">
                    <h5 class="mb-1">{user_name.clone()}</h5>
                </div>
            }) }
        </div>
    }

}