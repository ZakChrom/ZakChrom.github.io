use leptos::*;
use leptos_router::*;
use base64::engine::{general_purpose::URL_SAFE, Engine};
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
struct Config {
    commit: String,
    emojis: Vec<Emoji>
}

#[derive(Deserialize, Serialize, Clone)]
struct Emoji {
    name: String,
    url: String,
}

#[component]
fn Link(link: &'static str) -> impl IntoView {
    let linkreal: String = link.to_owned();
    view! {
        <a class="text-lg" href={linkreal.clone()}>{linkreal}</a>
    }
}

#[component]
fn Emoji(name: &'static str) -> impl IntoView {
    let config = use_context::<Resource<(), Config>>().unwrap();
    view!{
        {move || match config.get() {
            Some(config) => {
                if let Some(index) = config.emojis.iter().position(|e| e.name == name) {
                    let emoji = &config.emojis[index];
                    return view! {
                        <img class="inline-block align-middle h-[1.2em] w-auto" src={emoji.url.clone()} />
                    }.into_any()
                } else {
                    view! {<a>{name}</a>}.into_any()
                }
            },
            None => view! {<a>{name}</a>}.into_any()
        }}
    }
}

#[component]
fn Projects() -> impl IntoView {
    // <div class="flex justify-center gap-10">
    //     <div class="text-center" style="border: 1px solid var(--main-color); border-radius: 30px;background-color: var(--main-color); width: 500px; height: 100px;">
    //         <a class="text-xl">Aslion</a><br/>
    //         <a class="text-xl">Astro8 Emulator written in zig</a>
    //     </div>
    //     <div class="text-center" style="border: 1px solid var(--main-color); border-radius: 30px;background-color: var(--main-color); width: 500px; height: 100px;">

    //     </div>
    // </div>
    view! {
        <a class="text-xl" href="https://github.com/ZakChrom/Aslion">Aslion</a><br/>
        <a class="text-xl">Astro8 Emulator written in zig.</a><br/><br/>

        <hr style="width: 31ch;"/><br/>

        <a class="text-xl" href="https://github.com/ZakChrom/ZakChrom.github.io">This website!</a><br/>
        <a class="text-xl">Its written in rust using leptos.</a><br/><br/>

        <hr style="width: 31ch;"/><br/>

        <a class="text-xl" href="https://github.com/ZakChrom/A8CM">Cell Machine in Astro8</a><br/>
        <a class="text-xl">Its Cell Machine on the Astro8</a>
    }
}

#[component]
fn Uiua(code: &'static str) -> impl IntoView {
    view! {
        <a class="uiua" href={"https://uiua.org/pad?src=0_2_0__".to_owned() + &URL_SAFE.encode(code)}inner_html={code.replace("\n", "<br/>")}/>
    }
}

#[component]
fn Main() -> impl IntoView {
    view! {
        <a class="text-4xl">Hello, world! <Emoji name="staring_cat"/></a><br/><br/>
        <Link link="https://nohello.net/"/><br/>
        <Link link="https://xyproblem.info/"/><br/>
        <Link link="https://dontasktoask.com/"/><br/><br/>
        <div id="matter"></div>
        /*<Uiua code="Xy ← ⍘⍉⊞⊟. ÷÷2: -÷2,⇡.200
Rgb ← [:⍘⊟×.Xy ↯△⊢Xy0.5]
u ← ↥<0.2:>0.7.+×2 ×.:⍘⊟Xy
c ← <:⍜⍘√/+ Xy
⍉⊂:-¬u c1 +0.1 ≡↧¤c0.95Rgb" />*/
    }
}

const ESTONIA_FLAG: &str = "🇪🇪";

#[component]
fn About() -> impl IntoView {
    view! {
        <a class=Some("text-xl")>Hello! I am Calion. Im from Estonia {ESTONIA_FLAG}</a><br/>
        <a class=Some("text-xl")>My favourite programming languages are <Emoji name="c"/>c and <Emoji name="rust"/>rust.<br/>I also program alot in <Emoji name="zig"/>zig but i dont like how safe it is and its just annoying to use.</a><br/><br/>
    }
}

#[component]
fn Contact() -> impl IntoView {
    view! {
        <a class=Some("text-xl")>"Youtube: " <a href="https://youtube.com/@CalionYT">"@CalionYT"</a></a><br/>
        <a class=Some("text-xl")>"Github: " <a href="https://github.com/ZakChrom">"ZakChrom"</a></a><br/>
        <a class=Some("text-xl")>"Discord: @calionreal"</a><br/>
        <a class=Some("text-xl")>"Email: mail@calion.dev"</a>
    }
}

fn main() {
    let config = create_resource(|| (), |_| async {
        Request::get("/config.json")
            .send()
            .await
            .unwrap()
            .json::<Config>()
            .await
            .unwrap()
    });

    provide_context(config);

    mount_to_body(move || view! {
        <Router>
            <nav class="flex flex-wrap">
                <a class="text-2xl m-2" href="/">Home</a>
                <a class="text-2xl m-2" href="/projects">Projects</a>
                <a class="text-2xl m-2" href="/about">About</a>
                <a class="text-2xl m-2" href="/contact">Contact</a>
                <a class="text-2xl m-2" style="margin-left: auto;">
                {move || if let Some(config) = config.get() {
                    config.commit
                } else {
                    "Loading...".to_owned()
                }}
                </a>
            </nav>
            <hr style="border-color: var(--text-normal);"/>
            <main class="m-5">
                <Routes>
                    <Route path="/" view=Main/>
                    <Route path="/projects" view=Projects/>
                    <Route path="/about" view=About/>
                    <Route path="/contact" view=Contact/>
                    <Route path="/*any" view=|| view! {
                        <div class="flex h-screen items-center justify-center">
                            <a class="text-9xl mr-4" style="color: var(--danger-color)">404</a>
                            <a class="text-4xl">"Looks like you tried to access a page that doesnt exist!"</a>
                        </div>
                    }/>
                </Routes>
            </main>
        </Router>
    });
}