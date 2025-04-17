use yew::prelude::*;
use masuda::web::components::search_page::SearchPage;

#[function_component]
fn App() -> Html {
    html! {
        <SearchPage />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
