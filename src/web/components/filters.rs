use crate::{
    filter::{StatComparison, StatFilter},
    pokemon::{Nature, NATURES},
    Filter, Profile,
};

use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

// Props for the RNG filters: emits a Filter when the user clicks Search
#[derive(Properties, PartialEq)]
pub struct RngFiltersProps {
    pub on_search: Callback<Filter>,
}

#[function_component(RngFilters)]
pub fn rng_filters(props: &RngFiltersProps) -> Html {
    let profile = Profile::new(0, 0);
    // individual state hooks for each filter field
    let shiny = use_state(|| false);
    let hp: UseStateHandle<StatFilter> = use_state(|| StatFilter::HP(StatComparison::Any));
    let atk: UseStateHandle<StatFilter> = use_state(|| StatFilter::Attack(StatComparison::Any));
    let def: UseStateHandle<StatFilter> = use_state(|| StatFilter::Defense(StatComparison::Any));
    let spa: UseStateHandle<StatFilter> =
        use_state(|| StatFilter::SpecialAttack(StatComparison::Any));
    let spd: UseStateHandle<StatFilter> =
        use_state(|| StatFilter::SpecialDefense(StatComparison::Any));
    let spe: UseStateHandle<StatFilter> = use_state(|| StatFilter::Speed(StatComparison::Any));
    let natures: UseStateHandle<Vec<Nature>> = use_state(|| vec![]);

    // When Search is clicked, build a Filter and emit it
    let on_search_click = {
        let shiny = shiny.clone();
        let hp = hp.clone();
        let atk = atk.clone();
        let def = def.clone();
        let spa = spa.clone();
        let spd = spd.clone();
        let spe = spe.clone();
        let natures = natures.clone();
        let on_search = props.on_search.clone();
        Callback::from(move |_| {
            let profile = Profile::new(0, 0);
            let mut filter = Filter::new(&profile);
            if *shiny {
                filter = filter.shiny();
            }
            match *hp {
                StatFilter::HP(StatComparison::Any) => {}
                sf => filter = filter.with_stat(sf),
            }
            match *atk {
                StatFilter::Attack(StatComparison::Any) => {}
                sf => filter = filter.with_stat(sf),
            }
            match *def {
                StatFilter::Defense(StatComparison::Any) => {}
                sf => filter = filter.with_stat(sf),
            }
            match *spa {
                StatFilter::SpecialAttack(StatComparison::Any) => {}
                sf => filter = filter.with_stat(sf),
            }
            match *spd {
                StatFilter::SpecialDefense(StatComparison::Any) => {}
                sf => filter = filter.with_stat(sf),
            }
            match *spe {
                StatFilter::Speed(StatComparison::Any) => {}
                sf => filter = filter.with_stat(sf),
            }
            if !natures.is_empty() {
                filter = filter.with_natures((*natures).clone());
            }
            on_search.emit(filter);
        })
    };

    // Shiny checkbox handler
    let on_shiny = {
        let shiny = shiny.clone();
        Callback::from(move |e: Event| {
            shiny.set(!*shiny);
        })
    };

    // Helper to build stat filter controls
    fn cmp_val(filter: &StatFilter) -> StatComparison {
        match *filter {
            StatFilter::HP(c)
            | StatFilter::Attack(c)
            | StatFilter::Defense(c)
            | StatFilter::SpecialAttack(c)
            | StatFilter::SpecialDefense(c)
            | StatFilter::Speed(c) => c,
        }
    }

    // Handlers for HP
    let hp_comp = cmp_val(&*hp);
    let on_hp_comp = {
        let hp = hp.clone();
        Callback::from(move |e: Event| {
            let sel = e.target_unchecked_into::<HtmlSelectElement>();
            let val = sel.value();
            let prev = cmp_val(&*hp);
            let comp = match val.as_str() {
                "Any" => StatComparison::Any,
                "=" => StatComparison::EqualTo(if let StatComparison::EqualTo(v) = prev {
                    v
                } else {
                    0
                }),
                ">" => StatComparison::GreaterThan(if let StatComparison::GreaterThan(v) = prev {
                    v
                } else {
                    0
                }),
                "<" => StatComparison::LessThan(if let StatComparison::LessThan(v) = prev {
                    v
                } else {
                    0
                }),
                _ => StatComparison::Any,
            };
            hp.set(StatFilter::HP(comp));
        })
    };
    let on_hp_val = {
        let hp = hp.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            let v = input.value().parse::<u8>().unwrap_or(0);
            let prev = cmp_val(&*hp);
            let comp = match prev {
                StatComparison::EqualTo(_) => StatComparison::EqualTo(v),
                StatComparison::GreaterThan(_) => StatComparison::GreaterThan(v),
                StatComparison::LessThan(_) => StatComparison::LessThan(v),
                _ => StatComparison::Any,
            };
            hp.set(StatFilter::HP(comp));
        })
    };

    // Attack
    let atk_comp = cmp_val(&*atk);
    let on_atk_comp = {
        let atk = atk.clone();
        Callback::from(move |e: Event| {
            let sel = e.target_unchecked_into::<HtmlSelectElement>();
            let val = sel.value();
            let prev = cmp_val(&*atk);
            let comp = match val.as_str() {
                "Any" => StatComparison::Any,
                "=" => StatComparison::EqualTo(if let StatComparison::EqualTo(v) = prev {
                    v
                } else {
                    0
                }),
                ">" => StatComparison::GreaterThan(if let StatComparison::GreaterThan(v) = prev {
                    v
                } else {
                    0
                }),
                "<" => StatComparison::LessThan(if let StatComparison::LessThan(v) = prev {
                    v
                } else {
                    0
                }),
                _ => StatComparison::Any,
            };
            atk.set(StatFilter::Attack(comp));
        })
    };
    let on_atk_val = {
        let atk = atk.clone();
        Callback::from(move |e: InputEvent| {
            let v = e
                .target_unchecked_into::<HtmlInputElement>()
                .value()
                .parse::<u8>()
                .unwrap_or(0);
            let prev = cmp_val(&*atk);
            let comp = match prev {
                StatComparison::EqualTo(_) => StatComparison::EqualTo(v),
                StatComparison::GreaterThan(_) => StatComparison::GreaterThan(v),
                StatComparison::LessThan(_) => StatComparison::LessThan(v),
                _ => StatComparison::Any,
            };
            atk.set(StatFilter::Attack(comp));
        })
    };

    // Defense
    let def_comp = cmp_val(&*def);
    let on_def_comp = {
        let def = def.clone();
        Callback::from(move |e: Event| {
            let sel = e.target_unchecked_into::<HtmlSelectElement>();
            let val = sel.value();
            let prev = cmp_val(&*def);
            let comp = match val.as_str() {
                "Any" => StatComparison::Any,
                "=" => StatComparison::EqualTo(if let StatComparison::EqualTo(v) = prev {
                    v
                } else {
                    0
                }),
                ">" => StatComparison::GreaterThan(if let StatComparison::GreaterThan(v) = prev {
                    v
                } else {
                    0
                }),
                "<" => StatComparison::LessThan(if let StatComparison::LessThan(v) = prev {
                    v
                } else {
                    0
                }),
                _ => StatComparison::Any,
            };
            def.set(StatFilter::Defense(comp));
        })
    };
    let on_def_val = {
        let def = def.clone();
        Callback::from(move |e: InputEvent| {
            let v = e
                .target_unchecked_into::<HtmlInputElement>()
                .value()
                .parse::<u8>()
                .unwrap_or(0);
            let prev = cmp_val(&*def);
            let comp = match prev {
                StatComparison::EqualTo(_) => StatComparison::EqualTo(v),
                StatComparison::GreaterThan(_) => StatComparison::GreaterThan(v),
                StatComparison::LessThan(_) => StatComparison::LessThan(v),
                _ => StatComparison::Any,
            };
            def.set(StatFilter::Defense(comp));
        })
    };

    // Special Attack
    let spa_comp = cmp_val(&*spa);
    let on_spa_comp = {
        let spa = spa.clone();
        Callback::from(move |e: Event| {
            let sel = e.target_unchecked_into::<HtmlSelectElement>();
            let val = sel.value();
            let prev = cmp_val(&*spa);
            let comp = match val.as_str() {
                "Any" => StatComparison::Any,
                "=" => StatComparison::EqualTo(if let StatComparison::EqualTo(v) = prev {
                    v
                } else {
                    0
                }),
                ">" => StatComparison::GreaterThan(if let StatComparison::GreaterThan(v) = prev {
                    v
                } else {
                    0
                }),
                "<" => StatComparison::LessThan(if let StatComparison::LessThan(v) = prev {
                    v
                } else {
                    0
                }),
                _ => StatComparison::Any,
            };
            spa.set(StatFilter::SpecialAttack(comp));
        })
    };
    let on_spa_val = {
        let spa = spa.clone();
        Callback::from(move |e: InputEvent| {
            let v = e
                .target_unchecked_into::<HtmlInputElement>()
                .value()
                .parse::<u8>()
                .unwrap_or(0);
            let prev = cmp_val(&*spa);
            let comp = match prev {
                StatComparison::EqualTo(_) => StatComparison::EqualTo(v),
                StatComparison::GreaterThan(_) => StatComparison::GreaterThan(v),
                StatComparison::LessThan(_) => StatComparison::LessThan(v),
                _ => StatComparison::Any,
            };
            spa.set(StatFilter::SpecialAttack(comp));
        })
    };

    // Special Defense
    let spd_comp = cmp_val(&*spd);
    let on_spd_comp = {
        let spd = spd.clone();
        Callback::from(move |e: Event| {
            let sel = e.target_unchecked_into::<HtmlSelectElement>();
            let val = sel.value();
            let prev = cmp_val(&*spd);
            let comp = match val.as_str() {
                "Any" => StatComparison::Any,
                "=" => StatComparison::EqualTo(if let StatComparison::EqualTo(v) = prev {
                    v
                } else {
                    0
                }),
                ">" => StatComparison::GreaterThan(if let StatComparison::GreaterThan(v) = prev {
                    v
                } else {
                    0
                }),
                "<" => StatComparison::LessThan(if let StatComparison::LessThan(v) = prev {
                    v
                } else {
                    0
                }),
                _ => StatComparison::Any,
            };
            spd.set(StatFilter::SpecialDefense(comp));
        })
    };
    let on_spd_val = {
        let spd = spd.clone();
        Callback::from(move |e: InputEvent| {
            let v = e
                .target_unchecked_into::<HtmlInputElement>()
                .value()
                .parse::<u8>()
                .unwrap_or(0);
            let prev = cmp_val(&*spd);
            let comp = match prev {
                StatComparison::EqualTo(_) => StatComparison::EqualTo(v),
                StatComparison::GreaterThan(_) => StatComparison::GreaterThan(v),
                StatComparison::LessThan(_) => StatComparison::LessThan(v),
                _ => StatComparison::Any,
            };
            spd.set(StatFilter::SpecialDefense(comp));
        })
    };

    // Speed
    let spe_comp = cmp_val(&*spe);
    let on_spe_comp = {
        let spe = spe.clone();
        Callback::from(move |e: Event| {
            let sel = e.target_unchecked_into::<HtmlSelectElement>();
            let val = sel.value();
            let prev = cmp_val(&*spe);
            let comp = match val.as_str() {
                "Any" => StatComparison::Any,
                "=" => StatComparison::EqualTo(if let StatComparison::EqualTo(v) = prev {
                    v
                } else {
                    0
                }),
                ">" => StatComparison::GreaterThan(if let StatComparison::GreaterThan(v) = prev {
                    v
                } else {
                    0
                }),
                "<" => StatComparison::LessThan(if let StatComparison::LessThan(v) = prev {
                    v
                } else {
                    0
                }),
                _ => StatComparison::Any,
            };
            spe.set(StatFilter::Speed(comp));
        })
    };
    let on_spe_val = {
        let spe = spe.clone();
        Callback::from(move |e: InputEvent| {
            let v = e
                .target_unchecked_into::<HtmlInputElement>()
                .value()
                .parse::<u8>()
                .unwrap_or(0);
            let prev = cmp_val(&*spe);
            let comp = match prev {
                StatComparison::EqualTo(_) => StatComparison::EqualTo(v),
                StatComparison::GreaterThan(_) => StatComparison::GreaterThan(v),
                StatComparison::LessThan(_) => StatComparison::LessThan(v),
                _ => StatComparison::Any,
            };
            spe.set(StatFilter::Speed(comp));
        })
    };

    // Nature toggles
    let on_toggle_nature = {
        let natures = natures.clone();
        move |nature: Nature| {
            let natures = natures.clone();
            Callback::from(move |e: Event| {
                let mut vec = (*natures).clone();
                let checked = e.target_unchecked_into::<HtmlInputElement>().checked();
                if checked {
                    vec.push(nature)
                } else {
                    vec.retain(|n| n != &nature);
                }
                natures.set(vec);
            })
        }
    };

    html! {
        <div>
            <div>
                <label>
                    <input type="checkbox" onchange={on_shiny.clone()} checked={*shiny} /> {"Shiny only"}
                </label>
            </div>

            <div>
                <label>{"HP:"}</label>
                <select onchange={on_hp_comp.clone()}>
                    <option value="Any" selected={hp_comp==StatComparison::Any}>{"Any"}</option>
                    <option value="=" selected={matches!(hp_comp, StatComparison::EqualTo(_))}>{"="}</option>
                    <option value=">" selected={matches!(hp_comp, StatComparison::GreaterThan(_))}>{">"}</option>
                    <option value="<" selected={matches!(hp_comp, StatComparison::LessThan(_))}>{"<"}</option>
                </select>
                if hp_comp != StatComparison::Any {
                    <input type="number" min="0" max="31"
                        value={match hp_comp { StatComparison::EqualTo(v)|StatComparison::GreaterThan(v)|StatComparison::LessThan(v) => v.to_string(), _ => "".to_string() }}
                        oninput={on_hp_val.clone()} />
                }
            </div>

            <div>
                <label>{"Attack:"}</label>
                <select onchange={on_atk_comp.clone()}>
                    <option value="Any" selected={atk_comp==StatComparison::Any}>{"Any"}</option>
                    <option value="=" selected={matches!(atk_comp, StatComparison::EqualTo(_))}>{"="}</option>
                    <option value=">" selected={matches!(atk_comp, StatComparison::GreaterThan(_))}>{">"}</option>
                    <option value="<" selected={matches!(atk_comp, StatComparison::LessThan(_))}>{"<"}</option>
                </select>
                if atk_comp != StatComparison::Any {
                    <input type="number" min="0" max="31"
                        value={match atk_comp { StatComparison::EqualTo(v)|StatComparison::GreaterThan(v)|StatComparison::LessThan(v) => v.to_string(), _ => "".to_string() }}
                        oninput={on_atk_val.clone()} />
                }
            </div>

            <div>
                <label>{"Defense:"}</label>
                <select onchange={on_def_comp.clone()}>
                    <option value="Any" selected={def_comp==StatComparison::Any}>{"Any"}</option>
                    <option value="=" selected={matches!(def_comp, StatComparison::EqualTo(_))}>{"="}</option>
                    <option value=">" selected={matches!(def_comp, StatComparison::GreaterThan(_))}>{">"}</option>
                    <option value="<" selected={matches!(def_comp, StatComparison::LessThan(_))}>{"<"}</option>
                </select>
                if def_comp != StatComparison::Any {
                    <input type="number" min="0" max="31"
                        value={match def_comp { StatComparison::EqualTo(v)|StatComparison::GreaterThan(v)|StatComparison::LessThan(v) => v.to_string(), _ => "".to_string() }}
                        oninput={on_def_val.clone()} />
                }
            </div>

            <div>
                <label>{"Sp. Atk:"}</label>
                <select onchange={on_spa_comp.clone()}>
                    <option value="Any" selected={spa_comp==StatComparison::Any}>{"Any"}</option>
                    <option value="=" selected={matches!(spa_comp, StatComparison::EqualTo(_))}>{"="}</option>
                    <option value=">" selected={matches!(spa_comp, StatComparison::GreaterThan(_))}>{">"}</option>
                    <option value="<" selected={matches!(spa_comp, StatComparison::LessThan(_))}>{"<"}</option>
                </select>
                if spa_comp != StatComparison::Any {
                    <input type="number" min="0" max="31"
                        value={match spa_comp { StatComparison::EqualTo(v)|StatComparison::GreaterThan(v)|StatComparison::LessThan(v) => v.to_string(), _ => "".to_string() }}
                        oninput={on_spa_val.clone()} />
                }
            </div>

            <div>
                <label>{"Sp. Def:"}</label>
                <select onchange={on_spd_comp.clone()}>
                    <option value="Any" selected={spd_comp==StatComparison::Any}>{"Any"}</option>
                    <option value="=" selected={matches!(spd_comp, StatComparison::EqualTo(_))}>{"="}</option>
                    <option value=">" selected={matches!(spd_comp, StatComparison::GreaterThan(_))}>{">"}</option>
                    <option value="<" selected={matches!(spd_comp, StatComparison::LessThan(_))}>{"<"}</option>
                </select>
                if spd_comp != StatComparison::Any {
                    <input type="number" min="0" max="31"
                        value={match spd_comp { StatComparison::EqualTo(v)|StatComparison::GreaterThan(v)|StatComparison::LessThan(v) => v.to_string(), _ => "".to_string() }}
                        oninput={on_spd_val.clone()} />
                }
            </div>

            <div>
                <label>{"Speed:"}</label>
                <select onchange={on_spe_comp.clone()}>
                    <option value="Any" selected={spe_comp==StatComparison::Any}>{"Any"}</option>
                    <option value="=" selected={matches!(spe_comp, StatComparison::EqualTo(_))}>{"="}</option>
                    <option value=">" selected={matches!(spe_comp, StatComparison::GreaterThan(_))}>{">"}</option>
                    <option value="<" selected={matches!(spe_comp, StatComparison::LessThan(_))}>{"<"}</option>
                </select>
                if spe_comp != StatComparison::Any {
                    <input type="number" min="0" max="31"
                        value={match spe_comp { StatComparison::EqualTo(v)|StatComparison::GreaterThan(v)|StatComparison::LessThan(v) => v.to_string(), _ => "".to_string() }}
                        oninput={on_spe_val.clone()} />
                }
            </div>

            <fieldset>
                <legend>{"Natures"}</legend>
                { for NATURES.iter().map(|&nature| {
                    let checked = (*natures).contains(&nature);
                    html! {
                        <label style="margin-right:8px">
                            <input type="checkbox" checked={checked} onchange={on_toggle_nature(nature)} />
                            { nature.to_string() }
                        </label>
                    }
                }) }
            </fieldset>
            <button onclick={on_search_click.clone()}>{ "Search" }</button>
        </div>
    }
}
