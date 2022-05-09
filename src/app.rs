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
                <head>
                    <link href={"https://fonts.googleapis.com/css?family=Comfortaa" } rel={ "stylesheet" }/>
                    <style>
                    {"
                    body {
                        font-family: 'Comfortaa';font-size: 22px;
                    }
                    "}
                    </style>
                </head>
                <body>
                    <crate::login::LoginMenu/>
                </body>
            </>
        }
    }
}