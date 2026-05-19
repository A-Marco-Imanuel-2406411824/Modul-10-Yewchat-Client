use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Profile)]
pub fn profile() -> Html {
    let user = use_context::<User>().expect("No context found.");
    let username = user.username.borrow().clone();
    let avatar = format!(
        "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
        username
    );

    html! {
        <div class="bg-gradient-to-b from-indigo-500 via-purple-500 to-pink-500 w-screen h-screen flex items-center justify-center">
            <Link<Route> to={Route::Chat}>
                <button class="absolute top-5 left-5 bg-white text-purple-600 px-4 py-2 rounded-lg font-bold hover:bg-gray-100">
                    {"← Back to Chat"}
                </button>
            </Link<Route>>
            <div class="bg-white rounded-2xl shadow-2xl p-8 max-w-md text-center">
                <div class="mb-6">
                    <img
                        src={avatar}
                        alt={username.clone()}
                        class="w-32 h-32 rounded-full mx-auto border-4 border-purple-500 shadow-lg"
                    />
                </div>

                <h1 class="text-4xl font-bold mb-2">{"👋"}{username.clone()}</h1>

                <div class="bg-gradient-to-r from-purple-100 to-pink-100 rounded-lg p-4 mb-6">
                    <p class="text-2xl mb-2">{"⭐⭐⭐⭐⭐"}</p>
                    <p class="text-sm text-gray-700">{"Top Chatty Member"}</p>
                </div>

                <div class="space-y-4 text-left mb-8">
                    <div class="border-b-2 border-gray-200 pb-3">
                        <p class="text-xs text-gray-500 uppercase">{"Status"}</p>
                        <p class="text-lg font-semibold">{"🟢 Online & Ready to Chat"}</p>
                    </div>
                    <div class="border-b-2 border-gray-200 pb-3">
                        <p class="text-xs text-gray-500 uppercase">{"Joined"}</p>
                        <p class="text-lg font-semibold">{"Today 🎉"}</p>
                    </div>
                    <div>
                        <p class="text-xs text-gray-500 uppercase">{"Favorite Emojis"}</p>
                        <p class="text-2xl">{"😄 🎯 🚀 💡 🎨"}</p>
                    </div>
                </div>

                <div class="bg-blue-50 rounded-lg p-4 mb-6">
                    <p class="text-sm text-gray-700 italic">
                        {format!("\"Hey, I'm {} and I love chatting! 💬\"", username)}
                    </p>
                </div>

                <button class="w-full bg-gradient-to-r from-purple-600 to-pink-600 text-white px-6 py-3 rounded-lg font-bold hover:shadow-lg transform hover:scale-105 transition">
                    {"🎮 Edit Profile"}
                </button>
            </div>
        </div>
    }
}

