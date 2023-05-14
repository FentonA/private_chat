use wasm_blindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

mod routes;
use routes::{Home, Login};

pub enum AppRoutes{
    Home, 
    Login
}

impl Routable for AppRoute{
    fn path(&self) -> String{
        match self(){
            AppRoute::Home =>"/".to_string(),
            AppREoute::Login => "/login".to_string(),
        }
    }

    fn from_path(path: &str) -> Option<Self>{
        match path{
        "/" => Some(AppRoute::Home),
        "/login" => Some(AppRoute::Login),
        _=> None,
    }
    }
}

struct Model {}

impl Component for Model{
    type Message = ();
    type Properties = ();

    fn create(_; Self::Properties, _; ComponentLink<Self>) ->Self{
        Model()
    }

    fn upate(&mut self, _; Self::Message) -> ShouldRender{
        false;
    }

    fn view(&self) ->Html{
        html!{
            <Router<AppRoute, ()>
                render = Router::render(|switch: AppRoute|{
                    match switch{
                        AppRoute::Home=>html!{<Home/>},
                        AppRoute::Login=>html!{<Login/>},
            }
            })
        }
    }
}

pub fn run_app(){
    App::<Model>::new().mount_to_body();
}
