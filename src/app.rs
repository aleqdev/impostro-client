use std::rc::Rc;

use futures::{*, lock::Mutex};
use yew::{html, Component, Context, Html};
use ws_stream_wasm::*;

pub enum AppMsg {
    WsConnectionEstablished(WsMeta, WsStream),
    WsSend(String),
    WsNext(String),
    None
}

pub struct WsState {
    pub meta: WsMeta,
    pub stream: WsStream
}

pub struct App {
    pub ws: Option<Rc<Mutex<WsState>>>
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        _ctx.link().send_future(async {
            let (meta, stream) = WsMeta::connect(crate::API_WS, None).await.expect("could not connect wss");
            Self::Message::WsConnectionEstablished(meta, stream)
        });

        Self {
            ws: None
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::WsConnectionEstablished(meta, stream) => {
                self.ws = Some(Rc::new(Mutex::new(WsState {
                    meta: meta,
                    stream: stream
                })));

                let ws_cloned1 = self.ws.clone();
                let ws_cloned2 = self.ws.clone();
                let ws_cloned3 = self.ws.clone();

                _ctx.link().send_future(async move {
                    app_ws_send(ws_cloned1, "123".into()).await 
                });

                _ctx.link().send_future(async move {
                    app_ws_next(ws_cloned2).await 
                });

                _ctx.link().send_future(async move {
                    app_ws_send(ws_cloned3, "123".into()).await 
                });
            },
            AppMsg::WsSend(_) => todo!(),
            AppMsg::WsNext(str) => {
                gloo_console::log!("{}", str)
            }
            AppMsg::None => todo!(),
        }
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

pub async fn app_ws_next(state: Option<Rc<Mutex<WsState>>>) -> AppMsg {
    let mut res = AppMsg::None;
    if let Some(state) = state {
        let mut guard = state.lock().await;
        if let WsMessage::Text(text) = guard.stream.next().await.unwrap() {
            res = AppMsg::WsNext(text);
        }
    }
    res
}

pub async fn app_ws_send(state: Option<Rc<Mutex<WsState>>>, str: String) -> AppMsg {
    if let Some(state) = state {
        let mut guard = state.lock().await;
        guard.stream.send(WsMessage::Text(str)).await.unwrap();
    }
    AppMsg::None
}