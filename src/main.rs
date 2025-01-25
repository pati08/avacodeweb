#![feature(iter_intersperse)]

use yew::prelude::*;

fn generate_word() -> String {
    use rand::prelude::IteratorRandom;

    include_str!("dict.txt")
        .lines()
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string()
}

fn encode_word(input: &str) -> String {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let mut consonants = input.chars().filter(|i| !VOWELS.contains(i)).rev();
    input
        .chars()
        .map(|i| {
            if let Some(pos) = VOWELS.iter().position(|c| *c == i) {
                VOWELS[(pos + 1) % VOWELS.len()]
            } else if i.is_alphabetic() {
                consonants.next().unwrap()
            } else {
                i
            }
        })
        .collect()
}

fn encode(input: &str) -> String {
    input
        .split_whitespace()
        .map(encode_word)
        .intersperse(" ".to_string())
        .collect()
}

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let input_value = use_state(String::new);
    let message = use_state(|| Option::<&'static str>::None);

    let target_string = use_state(generate_word);

    let on_input = {
        let input_value = input_value.clone();
        let message = message.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            input_value.set(input.value());
            message.set(None);
        })
    };

    let on_submit = {
        let input_value = input_value.clone();
        let counter = counter.clone();
        let message = message.clone();
        let current_target = target_string.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let user_input = (*input_value).clone();

            if user_input == encode(&current_target) {
                message.set(Some("correct"));
                input_value.set(String::new());
                current_target.set(generate_word())
            } else {
                message.set(Some("incorrect"));
                counter.set(*counter + 1);
            }
        })
    };

    html! {
        <div class="min-h-screen bg-gray-50">
            <header class="bg-white shadow-sm">
                <h1 class="text-center text-3xl font-bold py-4 text-gray-800">
                    { "Secret Code Practice" }
                </h1>
            </header>

            <main class="container mx-auto px-4 py-8 max-w-2xl">
                <div class="bg-white rounded-lg shadow-md p-6 mb-6">
                    <h2 class="text-lg font-semibold text-gray-700 mb-3">
                        { "Current Target:" }
                    </h2>
                    <div class="border-2 border-gray-200 p-4 rounded-md font-mono bg-gray-50 text-gray-800">
                        { (*target_string).clone() }
                    </div>
                </div>

                <form onsubmit={on_submit} class="flex gap-2 mb-4">
                    <input
                        type="text"
                        value={(*input_value).clone()}
                        oninput={on_input}
                        placeholder="Encode the text"
                        class="flex-1 p-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent"
                    />
                    <button
                        type="submit"
                        class="px-4 py-2 bg-green-500 text-white rounded-md hover:bg-green-600 transition-colors font-medium focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2"
                    >
                        { "Verify" }
                    </button>
                </form>

                {
                    if let Some(msg) = *message {
                        let (color_class, message_text) = match msg {
                            "correct" => ("text-green-600", "Correct!"),
                            _ => ("text-red-600", "Incorrect! Try again."),
                        };

                        html! {
                            <div class={format!("p-3 rounded-md {} bg-{}/10 font-medium", color_class, color_class.split('-').nth(1).unwrap_or("green"))}>
                                { message_text }
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
            </main>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
}
