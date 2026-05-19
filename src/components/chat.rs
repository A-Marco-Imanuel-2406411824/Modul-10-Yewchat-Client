use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use yew_router::prelude::*;

use crate::{User, services::websocket::WebsocketService, Route};
use crate::services::event_bus::EventBus;

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    wss: WebsocketService,
    messages: Vec<MessageData>,
    _producer: Box<dyn Bridge<EventBus>>,
}
impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                )
                                    .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    //log::debug!("got input: {:?}", input.value());
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        let user = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set")
            .0;
        let username = user.username.borrow().clone();

        html! {
            <div class="flex w-screen">
                <div class="flex-none w-56 h-screen bg-gradient-to-b from-purple-100 to-pink-100 border-r-2 border-purple-200">
                    <div class="text-xl p-4 font-bold bg-gradient-to-r from-purple-500 to-pink-500 text-white rounded-b-lg">
                        <div>{"👥 Users"}</div>
                        <div class="text-xs font-normal mt-1">{format!("Online: {}", self.users.len())}</div>
                    </div>
                    {
                        self.users.clone().iter().map(|u| {
                            let is_current = u.name == username;
                            html!{
                                <div class={if is_current { "flex m-3 bg-gradient-to-r from-purple-300 to-pink-300 rounded-lg p-2 border-2 border-purple-500" } else { "flex m-3 bg-white rounded-lg p-2 hover:shadow-lg transition" }}>
                                    <div class="relative">
                                        <img class="w-12 h-12 rounded-full" src={u.avatar.clone()} alt="avatar"/>
                                        <div class="absolute bottom-0 right-0 w-3 h-3 bg-green-500 rounded-full border-2 border-white"></div>
                                    </div>
                                    <div class="flex-grow p-3">
                                        <div class="flex text-xs justify-between">
                                            <div class="font-semibold">{u.name.clone()}</div>
                                            if is_current {
                                                <span class="text-xs">{"(You)"}</span>
                                            }
                                        </div>
                                        <div class="text-xs text-gray-500">
                                            {"💬 Active now"}
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
                <div class="grow h-screen flex flex-col bg-gradient-to-br from-white to-gray-50">
                    <div class="w-full bg-gradient-to-r from-purple-500 to-pink-500 text-white border-b-2 border-gray-300 p-3 flex justify-between items-center">
                        <div>
                            <div class="text-xl font-bold">{"💬 YewChat Pro"}</div>
                            <div class="text-xs opacity-90">{"Real-time messaging at its finest ⚡"}</div>
                        </div>
                        <div class="flex space-x-2">
                            <Link<Route> to={Route::Profile}>
                                <button class="bg-white text-purple-600 px-3 py-1 rounded-lg text-sm font-semibold hover:bg-gray-100 transition">
                                    {"👤"}
                                </button>
                            </Link<Route>>
                            <Link<Route> to={Route::About}>
                                <button class="bg-white text-purple-600 px-3 py-1 rounded-lg text-sm font-semibold hover:bg-gray-100 transition">
                                    {"📖"}
                                </button>
                            </Link<Route>>
                        </div>
                    </div>
                    <div class="w-full grow overflow-auto border-b-2 border-gray-300 p-4 space-y-3">
                        if self.messages.is_empty() {
                            <div class="flex items-center justify-center h-full">
                                <div class="text-center text-gray-400">
                                    <p class="text-4xl mb-2">{"👋"}</p>
                                    <p>{"Start chatting to see messages here!"}</p>
                                    <p class="text-sm mt-2">{"Type a message below to get started 👇"}</p>
                                </div>
                            </div>
                        }
                        {
                            self.messages.iter().map(|m| {
                                let user = self.users.iter().find(|u| u.name == m.from).unwrap();
                                let is_own_message = m.from == username;
                                html!{
                                    <div class={if is_own_message { "flex justify-end" } else { "flex justify-start" }}>
                                        <div class={if is_own_message { "flex items-end w-3/6 bg-gradient-to-r from-blue-400 to-blue-500 text-white m-2 rounded-tl-3xl rounded-tr-3xl rounded-bl-3xl shadow-md" } else { "flex items-end w-3/6 bg-gray-200 m-2 rounded-tl-3xl rounded-tr-3xl rounded-br-3xl" }}>
                                            if !is_own_message {
                                                <img class="w-8 h-8 rounded-full m-3" src={user.avatar.clone()} alt="avatar"/>
                                            }
                                            <div class="p-3">
                                                if !is_own_message {
                                                    <div class="text-sm font-semibold text-gray-800">
                                                        {m.from.clone()}
                                                    </div>
                                                }
                                                <div class={if is_own_message { "text-sm text-white" } else { "text-sm text-gray-700" }}>
                                                    if m.message.ends_with(".gif") {
                                                        <img class="mt-3 rounded-lg" src={m.message.clone()} alt="gif"/>
                                                    } else {
                                                        {m.message.clone()}
                                                    }
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                    <div class="w-full bg-white border-t-2 border-gray-300 flex px-4 py-3 items-center space-x-3">
                        <input ref={self.chat_input.clone()} type="text" placeholder="Type something awesome... 💡" class="block w-full py-3 pl-4 bg-gray-100 rounded-full outline-none focus:text-gray-700 focus:ring-2 focus:ring-purple-500 focus:bg-white transition" name="message" required=true />
                        <button onclick={submit} class="p-3 shadow-md bg-gradient-to-r from-purple-600 to-pink-600 hover:shadow-lg w-12 h-12 rounded-full flex justify-center items-center color-white transition transform hover:scale-110">
                            <svg fill="#ffffff" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="fill-white">
                                <path d="M0 0h24v24H0z" fill="none"></path><path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }
    }


}