use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view!{cx,
        <div class="w-screen h-screen bg-black flex flex-col justify-center items-center text-white">
            <h1 class="text-xl font-black">"is-even"</h1>
            <input name="num-input" type="number" placeholder="Enter a number" class="p-3 bg-inherit text-2xl text-center focus:outline-none" hx-get="/hx/check-even" hx-target="#result" hx-swap="outerHTML" hx-trigger="change"/>
            <div id="result"></div>
        </div>
    }
}
