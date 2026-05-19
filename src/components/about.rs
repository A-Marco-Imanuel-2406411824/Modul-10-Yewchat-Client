use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="bg-gradient-to-br from-purple-600 via-pink-500 to-red-500 w-screen h-screen flex items-center justify-center">
            <Link<Route> to={Route::Chat}>
                <button class="absolute top-5 left-5 bg-white text-purple-600 px-4 py-2 rounded-lg font-bold hover:bg-gray-100">
                    {"← Back to Chat"}
                </button>
            </Link<Route>>
            <div class="bg-white rounded-2xl shadow-2xl p-8 max-w-2xl text-center">
                <h1 class="text-5xl font-bold mb-4">{"🚀 YewChat Pro Edition"}</h1>
                <p class="text-lg text-gray-700 mb-6">
                    {"A lightning-fast WebSocket chat application built with Rust & Yew!"}
                </p>

                <div class="grid grid-cols-2 gap-4 mb-8">
                    <div class="bg-blue-100 rounded-lg p-4">
                        <h3 class="text-2xl mb-2">{"⚡"}</h3>
                        <p class="text-sm font-semibold">{"Ultra Fast"}</p>
                        <p class="text-xs text-gray-600">{"Real-time messaging"}</p>
                    </div>
                    <div class="bg-green-100 rounded-lg p-4">
                        <h3 class="text-2xl mb-2">{"🔐"}</h3>
                        <p class="text-sm font-semibold">{"Secure"}</p>
                        <p class="text-xs text-gray-600">{"WebSocket encrypted"}</p>
                    </div>
                    <div class="bg-yellow-100 rounded-lg p-4">
                        <h3 class="text-2xl mb-2">{"🎨"}</h3>
                        <p class="text-sm font-semibold">{"Beautiful"}</p>
                        <p class="text-xs text-gray-600">{"Modern UI/UX"}</p>
                    </div>
                    <div class="bg-pink-100 rounded-lg p-4">
                        <h3 class="text-2xl mb-2">{"👥"}</h3>
                        <p class="text-sm font-semibold">{"Social"}</p>
                        <p class="text-xs text-gray-600">{"Connect instantly"}</p>
                    </div>
                </div>

                <div class="bg-gradient-to-r from-purple-200 to-pink-200 rounded-lg p-6 mb-8">
                    <h2 class="text-2xl font-bold mb-4">{"✨ Features"}</h2>
                    <ul class="text-left space-y-2 text-sm">
                        <li>{"✓ Real-time WebSocket messaging"}</li>
                        <li>{"✓ User avatars with DiceBear API"}</li>
                        <li>{"✓ Live user presence"}</li>
                        <li>{"✓ GIF support in messages"}</li>
                        <li>{"✓ Responsive design"}</li>
                        <li>{"✓ Emoji-rich experience"}</li>
                    </ul>
                </div>

                <p class="text-gray-500 text-sm">
                    {"Built with ❤️ using Rust, Yew, WebSockets & Tailwind CSS"}
                </p>
            </div>
        </div>
    }
}

