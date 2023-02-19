use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    // let (count, set_count) = create_signal(cx, 0);

    view! {
        cx,
        // <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        // <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=  move |cx| view! {
                    cx,
                    <div>
                        <p>"Hello, world!"</p>
                    </div>
                    // <main class="my-0 mx-auto max-w-3xl text-center">
                    //     <h2 class="p-6 text-4xl">"Welcome to Leptos with Tailwind"</h2>
                    //     <p class="px-10 pb-10 text-left">"Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."</p>
                    //     <button
                    //         class="bg-sky-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
                    //         on:click=move |_| set_count.update(|count| *count += 1)
                    //     >
                    //         {move || if count() == 0 {
                    //             "Click me!".to_string()
                    //         } else {
                    //             count().to_string()
                    //         }}
                    //     </button>
                    // </main>
            }/>
            </Routes>
        </Router>
    }
}
