use impostro_shared::{session::SessionId, api::ValidateSessionId};
use serde::Serialize;
use wasm_bindgen::JsValue;
use web_sys::{Request, RequestInit};
use yew::{html, Component, Context, Html};

pub enum LoginMsg {
    InputUpdate(String),
    Connect,
    ValidationResult(bool),
    None
}

pub struct LoginMenu {
    pub session_id: String
}

impl Component for LoginMenu {
    type Message = LoginMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            session_id: "".into()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::InputUpdate(input) => {
                self.session_id = input;
            },
            Self::Message::Connect => {
                use reqwest_wasm::*;
                gloo_console::log!("Sending req");

                let mut id: [char; 6] = Default::default();
                for (i, c) in self.session_id.chars().enumerate() {
                    id[i] = c;
                }
                
                let client = Client::new();
                let res = client.post(crate::API_VALIDATE_SESSION)
                    .json(&ValidateSessionId {
                        session: SessionId(id)
                    })
                    .send();

                _ctx.link().send_future((|| async move {
                    let resp = res.await.unwrap();
                    let text = resp.text().await.unwrap();
                    gloo_console::log!("Resp: {}", &text);
                    match text.as_str() {
                        "true" => Self::Message::ValidationResult(true),
                        _ => Self::Message::ValidationResult(false)
                    }
                    
                })())
            },
            Self::Message::ValidationResult(is_valid) => {},
            Self::Message::None => {}
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change = {
            ctx.link().callback(move |e: yew::InputEvent| {
                use yew::TargetCast;
                let input = e.target_dyn_into::<web_sys::HtmlInputElement>();
    
                if let Some(input) = input {
                    LoginMsg::InputUpdate(input.value())
                }
                else {
                    LoginMsg::None
                }
            })
        };

        let button_class = if self.session_id.len() == 6 {"active"} else {"inactive"};

        html! {
            <div id="login-client-div">
                <label id="login-client-label">
                    { "Введите код сессии:"  }
                </label>

                <br/>

                <input type="text" id="login-client-input" minlength="6" maxlength="6" 
                       oninput={on_change}/>

                <br/> <br/>

                <button id="login-client-button" class={button_class} onclick={ctx.link().callback(|_| Self::Message::Connect)}>
                    { "Подключиться" }
                </button>
            </div>
        }
    }
}