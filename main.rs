use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/games")]
    Games,
    #[at("/projects")]
    Projects,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(NavbarComponent)]
fn navbar() -> Html{
    html!{
            <nav class="navbar navbar-expand-lg bg-primary">
                <div class="container-fluid">
                    <a class="navbar-brand">{"Navbar"}</a>
                    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                        <span class="navbar-toggler-icon"></span>
                    </button>
                    <div class="collapse navbar-collapse" id="navbarSupportedContent">
                        <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                        <li class="nav-item">
                            <a class="nav-link"><Link<Route> to={Route::Home}>{ "Home" }</Link<Route>></a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link"><Link<Route> to={Route::Games}>{ "Chess" }</Link<Route>></a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link"><Link<Route> to={Route::Projects}>{ "Projects" }</Link<Route>></a>
                        </li>
                        </ul>
                    </div>
                </div>
            </nav>
            }
}


#[function_component(HomePage)]
fn home() -> Html{
    html!{
        <>
                <NavbarComponent/>
                <h1 class="header">{"Lachlan's Webpage"}</h1>
                <h1>{"Home"}</h1>
            </>
    }
}

#[function_component(GamesPage)]
fn games() -> Html {
    html!(<>
        <NavbarComponent/>
        <p class="header">{"Lachlan's Webpage"}</p>
        <h1>{"Games"}</h1>
        <div class="chessbox">
            <div>
                <h3>{"Tactics"}</h3>
                <iframe src="https://livetactics.chessbase.com" width="400" height="440"></iframe>
            </div>
            <div>
                <h3>{"Challenge Computer"}</h3>
                <iframe src="https://fritz.chessbase.com" style="width:760px;height:440px"></iframe>
            </div>      
        </div>
    </>)
}

#[function_component(ProjectPage)]
fn projects() -> Html {
    html!(<>
        <NavbarComponent/>
        <p class="header">{"Lachlan's Webpage"}</p>
        <h1>{"Games"}</h1>
        <div class="chessbox">
            <div>
                <h3>{"Tactics"}</h3>
                <iframe src="https://livetactics.chessbase.com" width="400" height="440"></iframe>
            </div>
            <div>
                <h3>{"Challenge Computer"}</h3>
                <iframe src="https://fritz.chessbase.com" style="width:760px;height:440px"></iframe>
            </div>      
        </div>
    </>)
}


#[function_component(Main)]
fn app() -> Html {
    let button_state = use_state(|| 0);

    let incr_counter = {
        let button_state = button_state.clone();
        Callback::from(move |_| button_state.set(*button_state + 1))
    };

    let decr_counter = {
        let button_state = button_state.clone();
        Callback::from(move |_| button_state.set(*button_state - 1))
    };
    

    html! {
        <>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>

            <p> {"current count: "} {*button_state} </p>
            <button onclick={incr_counter}> {"+"} </button>
            <button onclick={decr_counter}> {"-"} </button>
            
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <HomePage/>
        },
        Route::Games => html! {
            <GamesPage/>
        },
        Route::Projects => html!{
            <ProjectPage/>
        },
        Route::NotFound => html! {
            <div class="container">
                <p>{ "404" }</p>
            </div>
        },
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}

