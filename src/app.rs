use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize, Clone)]
struct Music {
    title: String,
    artist: String,
    url: String,
}

impl Music {
    pub fn new() -> Music {
        Music {
            title: "".to_string(),
            artist: "".to_string(),
            url: "".to_string(),
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (music_list, _set_music_list) = signal(vec![
        Music {
            title: "春".to_string(),
            artist: "ヴィヴァルディ".to_string(),
            url: "https://classix.sitefactory.info/mp3classic/vivaldi/1757.mp3".to_string(),
        },
        Music {
            title: "アイーダ 凱旋行進曲".to_string(),
            artist: "ヴェルディ".to_string(),
            url: "https://classix.sitefactory.info/mp3classic/verdi/1209.mp3".to_string(),
        },
        Music {
            title: "威風堂々".to_string(),
            artist: "エルガー".to_string(),
            url: "https://classix.sitefactory.info/mp3classic/elgar/644.mp3".to_string(),
        }
    ]);

    let (play_music, set_play_music) = signal(Music::new());

    view! {
        <main class="container">
            <div class="play-music">
                <h1 class="play-music-title">{move || play_music.get().title}</h1>
                <h3 class="play-music-artist">{move || play_music.get().artist}</h3>
            </div>

            <audio controls autoplay=true src={move || play_music.get().url}>
            </audio>

            <ul class="music-list">
                <For
                    each=move || music_list.get()
                    key=|music| music.url.clone()
                    let(music)
                >
                    <li class="music-item">
                        {
                            let music = music.clone();

                            view! {
                                <button
                                    class="play-button"
                                    on:click=move |_| { set_play_music.set(music.clone()); }
                                >
                                    "▶"
                                </button>
                            }
                        }
                        {
                            let music = music.clone();

                            view! {
                                <div class="music-info">
                                    <p class={move || {
                                        if music.url == play_music.get().url {
                                            "music-info-title-play"
                                        } else {
                                            "music-info-title"
                                        }
                                    }}>{music.title}</p>
                                    <p class="music-info-artist">{music.artist}</p>
                                </div>
                            }
                        }
                    </li>
                </For>
            </ul>
        </main>
    }
}
