// src/routes/login.rs
use crate::types::{Message, User};
use yew::prelude::*;

pub enum Msg {
    UpdateUsername(String),
    UdatePassword(String),
    Login,
}

impl Component for Login {
    type Nessage = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Login {
            link,
            username: String::new(),
            password: String::new(),
        }
    }

    fn upate(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateUsername(username) => self.username = username,
            Msg::UpdatePassword(password) => self.password = password,
            Msg::Login => {
                // @Todo: log User
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <input
                    value=&self.username
                    oniput=self.link.callback(|e: InputData| Msg::UpdateUsername(e.value))
                    placeholder="Username"
                />
                <input
                    value =&self.password
                    oninput=self.link.callback(|e: InputDate| Msg::UpdatePassword(e.value))
                    type="password"
                />
                <button onclick=self.link.callback(|_| Msg::Login)>{"Login"}</button>
            </div>
        }
    }
}
