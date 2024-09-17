use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/app")]
    App,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Home)]
fn home() -> Html {
    let navigator = use_navigator().unwrap();

    html! {
        <div class="container">
            <button onclick = {move |_| navigator.push(&Route::App)} >
                { "Go to app" }
            </button>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let navigator = use_navigator().unwrap();
    let counter = use_state(|| 0i64);
    let onclick_inc = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };
    let onclick_dec = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter - 1))
    };
    let onclick_back = Callback::from(move |_| {
        navigator.back();
    });
    html! {
        <div class="container">
            <p>{ *counter }</p>
            <button onclick = {onclick_inc}>{ "Inc" }</button>
            <button class="margined-btn" onclick = {onclick_dec} >{ "Dec" }</button>
            <button class="margined-btn" onclick = {onclick_back} >{ "Back" }</button>
        </div>
    }
}

#[function_component(NotFound)]
fn not_found() -> Html {
    html! {
        <h1>{ "404 - Page Not Found" }</h1>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::App => html! { <App /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

// Root component with router
#[function_component(Root)]
fn root() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Root>::new().render();
}
