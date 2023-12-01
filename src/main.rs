use leptos::*;
use leptos_router::*;
use base64::engine::{general_purpose::URL_SAFE, Engine};

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
fn Main() -> impl IntoView {
    view! {
        <a class="text-4xl">"Hello, world!"</a><br/>
        <Uiua code="Xy ← ⍘⍉⊞⊟. ÷÷2: -÷2,⇡.200
Rgb ← [:⍘⊟×.Xy ↯△⊢Xy0.5]
u ← ↥<0.2:>0.7.+×2 ×.:⍘⊟Xy
c ← <:⍜⍘√/+ Xy
⍉⊂:-¬u c1 +0.1 ≡↧¤c0.95Rgb" />
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <a class="text-xl">"Youtube: " <Link link="https://youtube.com/@CalionYT"/></a><br/>
        <a class="text-xl">"Github: " <Link link="https://github.com/ZakChrom"/></a><br/>
        <a class="text-xl">"Discord: " <a style="color: var(--main-color);">"@calionreal"</a></a>
    }
}

fn main() {
    mount_to_body(|| view! {
        <Router>
            <nav>
                <a class="text-2xl m-2" href="/">Home</a>
                <a class="text-2xl m-2" href="/about">About</a>
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