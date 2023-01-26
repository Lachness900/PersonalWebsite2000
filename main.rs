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
                <h1 class="header">{"Lachlan's Webpage"}</h1>
                <NavbarComponent/>
                <h1>{"Home"}</h1>
                <p>{"This is a collection of stuff I wanted to put on a website"}</p>
        </>
    }
}

#[function_component(GamesPage)]
fn games() -> Html {
    html!(<>
        <h1 class="header">{"Lachlan's Webpage"}</h1>
        <NavbarComponent/>
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
        <h1 class="header">{"Lachlan's Webpage"}</h1>
        <NavbarComponent/>
        <h1>{"Projects"}</h1>
        <ul>
            <li><a href="https://github.com/Lachness900">{"Github Account"}</a></li>
            <li><a href="https://lachness900.itch.io/waves-of-pygame">{"Waves of Pygame"}</a></li>
            <li><a href="https://wokwi.com/projects/354815499605834753">{"Ardiuno Connect 4"}</a></li>
            <li>{"This"}</li>
        </ul>
        
    </>)
}




#[function_component(Main)]
fn app() -> Html {
    let button_state = use_state(|| 0.0);
    let button_state_multiplier = use_state(||1.0);
    

    let incr_counter = {
        let button_state = button_state.clone();
        let button_state_multiplier = button_state_multiplier.clone();
        Callback::from(
            move |_| button_state.set(*button_state + (1.0 * *button_state_multiplier))
        )
    };

    let incr_counter5 = {
        let button_state = button_state.clone();
        let button_state_multiplier = button_state_multiplier.clone();
        Callback::from(
            move |_| button_state.set(*button_state + (5.0 * *button_state_multiplier))
        )
    };
    let upgrade1 = {
        let button_state = button_state.clone();
        let button_state_multiplier = button_state_multiplier.clone();
        Callback::from(
            move |_| button_state.set(*button_state_multiplier + 0.2)
        )
    };

    html! {
        <>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>

            <p> {"The number: "} {*button_state} </p>
            <button onclick={incr_counter}> {"+"} </button>
            if *button_state>10.0{
                <button onclick={incr_counter5}> {"+5"} </button>
            }
            if *button_state>50.0{
                <p>{"Upgrades"}</p>
                <button onclick={upgrade1} class="upgrade">{"make clicking stronger"}</button>
                
            }
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

