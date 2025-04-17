use crate::search::SearchResult;
use yew::prelude::*;

/// Props for the results table component
#[derive(Properties, PartialEq)]
pub struct ResultsTableProps {
    pub results: Vec<SearchResult>,
}

/// Displays a list of search results (up to 10 rows)
#[function_component(ResultsTable)]
pub fn results_table(props: &ResultsTableProps) -> Html {
    html! {
        <div class="results-table">
            <h3>{"Results"}</h3>
            <ul>
                { for props.results.iter().map(|res| html! {
                    <li>{ format!("frame {}: {:?}", res.frame, res.pokemon) }</li>
                }) }
            </ul>
        </div>
    }
}

