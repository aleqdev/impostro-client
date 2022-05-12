use yew::{html, Component, Context, Html};

pub enum AppMsg {

}

pub struct App {

}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <crate::login::LoginMenu/>
            </>
        }
    }
}