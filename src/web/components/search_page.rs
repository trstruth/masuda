use crate::generators::{Game, Method};
use crate::search::{SearchResult, Searcher};
use crate::web::components::filters::RngFilters;
use crate::web::components::results_table::ResultsTable;
use crate::Filter;
use yew::prelude::*;

/// The main search page: renders filter controls and displays results.
#[function_component(SearchPage)]
pub fn search_page() -> Html {
    // Create the searcher: e.g., Emerald, method 1, limit 100_000 frames
    let searcher = use_mut_ref(|| Searcher::new(Game::Emerald, Method::One, 10_000_000));
    // State for holding the latest search results
    let results = use_state(Vec::<SearchResult>::new);

    // Callback invoked when filter controls emit a Filter
    let on_search = {
        let results = results.clone();
        let searcher = searcher.clone();
        Callback::from(move |filter: Filter| {
            let mut s = searcher.borrow_mut();
            let mut found = s.search(Some(filter));
            // keep only first 10
            if found.len() > 10 {
                found.truncate(10);
            }
            results.set(found);
        })
    };

    html! {
        <div class="search-page">
            <RngFilters on_search={on_search.clone()} />
            <ResultsTable results={(*results).clone()} />
        </div>
    }
}

