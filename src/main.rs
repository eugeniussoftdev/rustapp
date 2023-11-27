use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <button>Press</button>
        </div>
    }
}

fn main() {
    yew::start_app::App();
}
