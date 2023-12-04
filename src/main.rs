use std::default;

use log::info;
use log::log;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
struct ListProps {
    list: Vec<ListItem>,
}
#[function_component(ListComponent)]
fn list_render(ListProps { list }: &ListProps) -> Html {
    list.iter()
        .map(|item| {
            html! {
                <p key={item.id}>{item.name.clone()} <span>{item.phone.clone()}</span></p>
            }
        })
        .collect()
}

struct Model {
    value: i64,
    list: Vec<ListItem>,
}

#[derive(Clone, Properties, PartialEq)]
struct ListItem {
    id: i32,
    name: String,
    phone: i32,
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model {
        value: 0,
        list: vec![ListItem {
            id: 1,
            name: String::from("Tom"),
            phone: 5555555,
        }],
    });

    let inputField = use_state(|| String::new());

    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    let plusStateHandler = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1,
                list: state.list.clone(),
            })
        })
    };

    let minusStateHnadler = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                value: state.value - 1,
                list: state.list.clone(),
            })
        })
    };

    let addPersonToList = {
        let state = state.clone();
        let newName = input_value.clone();

        Callback::from(move |_| {
            let newName = newName.clone();
            let newItem = ListItem {
                id: state.list.len() as i32 + 1,
                name: newName,
                phone: 5555555,
            };

            let mut newList = state.list.clone();
            newList.push(newItem);

            state.set(Model {
                value: state.value,
                list: newList,
            })
        })
        // .into()
    };

    let addNewPersonName = {
        let inputField = inputField.clone();

        Callback::from({
            let inputField = inputField.clone();
            move |input_event: InputEvent| {
                let target: HtmlInputElement =
                    input_event.current_target().unwrap().dyn_into().unwrap();

                inputField.set(target.value());
            }
        })
        // .into()
    };

    let on_cautious_change = {
        let input_value_handle = input_value_handle.clone();

        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                input_value_handle.set(input.value());
            }
        })
    };

    let on_dangerous_change = Callback::from(move |e: Event| {
        // You must KNOW target is a HtmlInputElement, otherwise
        // the call to value would be Undefined Behaviour (UB).
        input_value_handle.set(e.target_unchecked_into::<HtmlInputElement>().value());
    });

    html! {
        <div>
        <p>{ state.value} </p>
            <button onclick={plusStateHandler} >{ "Plus" }</button>
            <button onclick={minusStateHnadler} >{ "Minus" }</button>
            <div>
            <>
            <label for="cautious-input">
                { "My cautious input:" }
                <input onchange={on_cautious_change}
                    id="cautious-input"
                    type="text"
                    value={input_value.clone()}
                />
            </label>
            <label for="dangerous-input">
                { "My dangerous input:" }
                <input onchange={on_dangerous_change}
                    id="dangerous-input"
                    type="text"
                    value={input_value}
                />
            </label>
        </>
            <div style="display: flex; flex-direction: column; width: 300px; margin-top: 20px;">
            <p>{&*inputField} </p>
                // <input type="text" placeholder="add new user" style="height: 50px" onchange={addNewPersonName} value={(*inputField).clone()} />
                <button style="height: 50px" onclick={addPersonToList}>{ "Add New Person" }</button>
            </div>

                <ListComponent list={state.list.clone()} />
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
