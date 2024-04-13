use yew::prelude::*;
use yew_hooks::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;

use crate::services::{rounds, fields};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub callback: Callback<i32>
}


#[function_component(RoundFilter)]
pub fn round_filter(props: &Props) -> Html {
    
    let round_list = {
        use_async_with_options(
            async move {rounds::all().await},
            UseAsyncOptions::enable_auto(), 
        )
    };

    let on_change = {
        let callback = props.callback.clone();
        Callback::from(move | e: Event | {
            let target = e.target();
            let option = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(option) = option {
                let value: i32 = option.value().parse().unwrap_or(0);
                callback.emit(value);
            }
            
        })
    };

    html! {
        <div>
            <select onchange={on_change} name="test" id="test"> 
            <option value="" selected={true}> </option>
            {
                if let Some(round_list) = &round_list.data {
                    if !round_list.rounds.is_empty() {
                        html! {
                            <>
                                {for round_list.rounds.iter().map(|round| {
                                    html! { 
                                        <option value={round.id.to_string()}>{&round.name}</option>
                                    }
                                })}
                            </>
                        }
                    } else {
                        html! {
                            <option value="nothing to see here yet"></option>
                        }
                    }
                } else {
                    html! {
                        <option value="nothing to see here"></option>
                    }
                }
            }
            </select>
        </div>
    }

}

#[function_component(FieldFilter)]
pub fn field_filter(props: &Props) -> Html {
    
    let field_list = {
        use_async_with_options(
            async move {fields::all().await},
            UseAsyncOptions::enable_auto(), 
        )
    };

    let on_change = {
        let callback = props.callback.clone();
        Callback::from(move | e: Event | {
            let target = e.target();
            let option = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(option) = option {
                let value: i32 = option.value().parse().unwrap_or(0);
                callback.emit(value);
            }
            
        })
    };

    html! {
        <div>
            <select onchange={on_change} name="test" id="test"> 
            <option value="" selected={true}> </option>
            {
                if let Some(field_list) = &field_list.data {
                    if !field_list.fields.is_empty() {
                        html! {
                            <>
                                {for field_list.fields.iter().map(|field| {
                                    html! { 
                                        <option value={field.id.to_string()}>{&field.name}</option>
                                    }
                                })}
                            </>
                        }
                    } else {
                        html! {
                            <option value="nothing to see here yet"></option>
                        }
                    }
                } else {
                    html! {
                        <option value="nothing to see here"></option>
                    }
                }
            }
            </select>
        </div>
    }

}