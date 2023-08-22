use worker::*;
use leptos::*;
use leptos::ssr::render_to_stream;
use futures::StreamExt;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view!{cx,
        <div class="w-screen h-screen bg-black flex flex-col justify-center items-center text-white">
            <h1 class="text-xl font-black">"is-even"</h1>
            <input name="num-input" type="number" placeholder="Enter a number" class="p-3 bg-inherit text-2xl text-center focus:outline-none" hx-get="/hx/check-even" hx-target="#result" hx-swap="outerHTML" hx-trigger="change"/>
            <div id="result"></div>
        </div>
    }
}

fn check_even(req: Request) -> Result<Response> {
    let url = req.url()?.to_string();
    let parts = url.split('?').collect::<Vec<&str>>();
    let parts2 = parts[1].split('=').collect::<Vec<&str>>();
    let num = parts2[1].parse::<i32>();
    if let Ok(n) = num {
        if n % 2 == 1 {
            let stream = render_to_stream(move |cx| view!{cx,
                <div id="result" class="bg-red-300 text-black py-1 px-5 rounded-full">
                    <small>{n} is ODD</small>
                </div>
            }.into_view(cx))
            .map(|html| Result::Ok(html.into_bytes()));
            let mut res = Response::from_stream(stream)?;
            res.headers_mut().set("Content-Type", "text/html")?;
            return Ok(res);
        }
        let stream = render_to_stream(move |cx| view!{cx,
            <div id="result" class="bg-green-300 text-black py-1 px-5 rounded-full">
                <small>{n} is EVEN</small>
            </div>
        }.into_view(cx))
        .map(|html| Result::Ok(html.into_bytes()));
        let mut res = Response::from_stream(stream)?;
        res.headers_mut().set("Content-Type", "text/html")?;
        return Ok(res);
    } else {
        let stream = render_to_stream(move |cx| view!{cx,
            <div id="result" class="bg-orange-300 text-black py-1 px-5 rounded-full">
                <small>Enter a valid number</small>
            </div>
        }.into_view(cx))
        .map(|html| Result::Ok(html.into_bytes()));
        let mut res = Response::from_stream(stream)?;
        res.headers_mut().set("Content-Type", "text/html")?;
        return Ok(res);
    }
}

fn index() -> Result<Response> {
    let head = format!(
        r#"<!DOCTYPE html>
        <html lang="en">
            <head>
                <script src="https://cdn.tailwindcss.com"></script>
                <script src="https://unpkg.com/htmx.org@1.9.4" integrity="sha384-zUfuhFKKZCbHTY6aRR46gxiqszMk5tcHjsVFxnUo8VMus4kHGVdIYVbOYYNlKmHV" crossorigin="anonymous"></script>

                <style>
                    /* Chrome, Safari, Edge, Opera */
                    input::-webkit-outer-spin-button,
                    input::-webkit-inner-spin-button {{
                      -webkit-appearance: none;
                      margin: 0;
                    }}

                    /* Firefox */
                    input[type=number] {{
                      -moz-appearance: textfield;
                    }}
                </style>
            </head>
            <body>"#
    );
    let tail = "</body></html>";
    let stream = futures::stream::once(async move { head.clone() })
        .chain(render_to_stream(
            |cx| view! { cx, <App /> }.into_view(cx)
        ))
        .chain(futures::stream::once(async { tail.to_string() }))
        .inspect(|html| println!("{html}"))
        .map(|html| Result::Ok(html.into_bytes()));

    let mut res = Response::from_stream(stream)?;
    res.headers_mut().set("Content-Type", "text/html")?;
    return Ok(res);
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    log_request(&req);
    let router = Router::new();
    return router
        .get("/hx/check-even", |req, _| {
            return check_even(req);
        })
        .get("/", |_req, _ctx| {
            return index();
        })
        .run(req, env).await;
}
