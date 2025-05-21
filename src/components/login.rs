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
       <div class="login-container min-h-screen flex w-screen">
            <div class="container mx-auto px-4 py-10 flex flex-col justify-center items-center">
                <div class="w-full max-w-md bg-white rounded-xl shadow-lg p-8 transform transition duration-500 hover:scale-105">
                    <div class="text-center mb-8">
                        <h1 class="text-3xl font-bold text-indigo-600 mb-2">{"YewChat"}</h1>
                        <p class="text-gray-600">{"Connect and chat in real-time"}</p>
                    </div>
                    
                    <div class="mb-8 flex justify-center">
                        <img src="https://avatars.dicebear.com/api/adventurer-neutral/welcome.svg" 
                             alt="Welcome Avatar" 
                             class="w-32 h-32 rounded-full border-4 border-indigo-200 p-1" />
                    </div>
                    
                    <form class="space-y-6">
                        <div>
                            <label for="username" class="block text-sm font-medium text-gray-700 mb-1">{"Username"}</label>
                            <input 
                                id="username"
                                {oninput} 
                                class="w-full px-4 py-3 rounded-lg border border-gray-300 focus:outline-none focus:ring-2 focus:ring-indigo-500 transition" 
                                placeholder="Enter your username" 
                            />
                        </div>
                        
                        <div>
                            <Link<Route> to={Route::Chat}>
                                <button 
                                    {onclick} 
                                    disabled={username.len()<1} 
                                    class="w-full bg-gradient-to-r from-indigo-500 to-purple-600 text-white font-bold py-3 px-4 rounded-lg hover:opacity-90 transition disabled:opacity-50 disabled:cursor-not-allowed"
                                >
                                    {"Start Chatting"}
                                </button>
                            </Link<Route>>
                        </div>
                    </form>
                    
                    <div class="mt-6 text-center text-sm text-gray-500">
                        {"Enter any username to start chatting instantly"}
                    </div>
                </div>
            </div>
        </div>
    }
}
