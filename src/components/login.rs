use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div class="bg-gradient-to-br from-blue-500 via-purple-500 to-pink-500 w-screen h-screen flex items-center justify-center">
            <div class="container mx-auto flex flex-col justify-center items-center">
                <div class="text-center mb-8 text-white">
                    <h1 class="text-6xl font-bold mb-2">{"💬 YewChat"}</h1>
                    <p class="text-xl">{"Lightning-Fast Real-Time Messaging"}</p>
                </div>

                <form class="m-4 flex flex-col items-center">
                    <div class="bg-white rounded-2xl shadow-2xl p-8 mb-4">
                        <p class="text-gray-700 text-center mb-6 font-semibold">
                            {"Enter your username to join the fun! 🎉"}
                        </p>
                        <input {oninput} class="rounded-lg p-4 border-2 border-purple-300 text-gray-800 bg-gray-50 w-64 focus:outline-none focus:border-purple-600 focus:shadow-lg transition" placeholder="Your awesome username..."/>
                    </div>
                    <Link<Route> to={Route::Chat}>
                        <button {onclick} disabled={username.len()<1} class="px-8 rounded-lg bg-gradient-to-r from-purple-600 to-pink-600 text-white font-bold p-4 uppercase border-0 hover:shadow-xl disabled:opacity-50 disabled:cursor-not-allowed transform hover:scale-105 transition w-72" >
                            {"🚀 Go Chatting!"}
                        </button>
                    </Link<Route>>
                </form>

                <div class="mt-12 text-white text-center space-y-2">
                    <Link<Route> to={Route::About}>
                        <a class="inline-block mr-4 text-white hover:underline font-semibold">
                            {"📖 About"}
                        </a>
                    </Link<Route>>
                    <Link<Route> to={Route::Profile}>
                        <a class="inline-block text-white hover:underline font-semibold">
                            {"👤 View Profile"}
                        </a>
                    </Link<Route>>
                </div>

                <div class="absolute bottom-4 text-white text-center text-sm">
                    <p>{"Made with ❤️ using Rust & Yew"}</p>
                </div>
            </div>
        </div>
    }
}