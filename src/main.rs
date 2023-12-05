use leptos::*;
use leptos_router::*;
use base64::engine::{general_purpose::URL_SAFE, Engine};
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
struct Config {
    commit: String
}

#[component]
fn Link(link: &'static str) -> impl IntoView {
    let linkreal: String = link.to_owned();
    view! {
        <a class="text-lg" href={linkreal.clone()}>{linkreal}</a>
    }
}

// #[component]
// fn Project(link: &'static str, description: &'static str) -> impl IntoView {
//     view! {
//         <div style="border: 2px solid; border-color: var(--main-color)">
//             <a class="text-xl">{description}</a>
//             <Link link={link}/>
//         </div>
//     }
// }

#[component]
fn Uiua(code: &'static str) -> impl IntoView {
    view! {
            <a class="uiua" href={"https://uiua.org/pad?src=0_2_0__".to_owned() + &URL_SAFE.encode(code)}inner_html={code.replace("\n", "<br/>")}/>
    }
}

#[component]
fn Text(children: Children, class: Option<&'static str>) -> impl IntoView {
    view! {
        <a class={class.unwrap_or("")}>
            {children()
                .nodes
                .into_iter()
                .map(|child| {
                    let mut err: bool = false;
                    let mut text: String = "".to_string();
                    if let Some(t) = child.as_text() {
                        text = t.content.replace(":flag_ee:", FLAG_EE);
                    } else { err = true; }

                    if err {
                        child
                    } else {
                        text.into_view()
                    }
                }).collect::<Vec<_>>()}
        </a>
    }
}

#[component]
fn Main() -> impl IntoView {
    view! {
        <a class="text-4xl">"Hello, world!"</a><br/><br/>
        <Link link="https://nohello.net/"/><br/>
        <Link link="https://xyproblem.info/"/><br/>
        <Link link="https://dontasktoask.com/"/><br/><br/>
        <Uiua code="Xy ← ⍘⍉⊞⊟. ÷÷2: -÷2,⇡.200
Rgb ← [:⍘⊟×.Xy ↯△⊢Xy0.5]
u ← ↥<0.2:>0.7.+×2 ×.:⍘⊟Xy
c ← <:⍜⍘√/+ Xy
⍉⊂:-¬u c1 +0.1 ≡↧¤c0.95Rgb" />
    }
}

const FLAG_EE: &str = "🇪🇪";

#[component]
fn About() -> impl IntoView {
    view! {
        <Text class=Some("text-xl")>Hello! I am Calion. Im from Estonia :flag_ee:</Text><br/>
        <Text class=Some("text-xl")>My favourite programming languages are c and rust.<br/>I also program alot in zig but i dont like how safe it is and its just annoying to use.</Text><br/><br/>
        <Text class=Some("text-xl")>"Youtube: " <Link link="https://youtube.com/@CalionYT"/></Text><br/>
        <Text class=Some("text-xl")>"Github: " <Link link="https://github.com/ZakChrom"/></Text><br/>
        <Text class=Some("text-xl")>"Discord: " <a style="color: var(--main-color);">"@calionreal"</a></Text>
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

    mount_to_body(move || view! {
        <Router>
            <nav class="flex flex-wrap">
                <a class="text-2xl m-2" href="/">Home</a>
                <a class="text-2xl m-2" href="/about">About</a>
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
                    <Route path="/about" view=About/>
                    <Route path="/*any" view=|| view! {
                        <div class="flex h-screen items-center justify-center">
                            <a class="text-9xl mr-4" style="color: var(--danger-color)">404</a>
                            <a class="text-4xl">"Looks like you tried to access a page that doesnt exist!"</a>
                        </div>
                    }/>
                </Routes>
            </main>
        </Router>
        //<Project link="https://github.com/ZakChrom/Aslion" description="Aslion is an Astro8 Emulator that was made for debugging altough i havent added that yet :staring_cat:"/>
    });
}