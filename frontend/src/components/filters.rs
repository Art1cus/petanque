use yew::prelude::*;
use yew_hooks::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;

use crate::services::{rounds, fields, start_times};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub callback: Callback<i32>
}

#[derive(Properties, Clone, PartialEq)]
pub struct StartTimeProps {
    pub callback: Callback<String>
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
            <select onchange={on_change} class="custom-select"> 
                <option value="" selected={true}>{"Alle rondes"}</option>
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
            <select onchange={on_change} class="custom-select"> 
                <option value="" selected={true}>{"Alle velden"}</option>
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

#[function_component(StartTimeFilter)]
pub fn start_time_filter(props: &StartTimeProps) -> Html {
    
    let start_time_list = {
        use_async_with_options(
            async move {start_times::all().await}, 
            UseAsyncOptions::enable_auto(), 
        )
    };

    let on_change = {
        let callback = props.callback.clone();
        Callback::from(move | e: Event | {
            let target = e.target();
            let option = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(option) = option {
                let value: String = option.value();
                callback.emit(value);
            }
            
        })
    };

    html! {
        <div>
            <select onchange={on_change} class="custom-select"> 
                <option value="" selected={true}>{"Start tijd"}</option>
                {
                    if let Some(start_time_list) = &start_time_list.data {
                        if !start_time_list.start_times.is_empty() {
                            html! {
                                <>
                                    {for start_time_list.start_times.iter().map(|start_time| {
                                        html! { 
                                            <option value={start_time.start_time_dt.format("%Y-%m-%dT%H:%M:%S").to_string()}>{&start_time.start_time_dt.format("%H:%M").to_string().clone()}</option>
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
