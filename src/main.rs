use std::fmt::format;

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

        Callback::from(move |_| {
            let newItem = ListItem {
                id: state.list.len() as i32 + 1,
                name: format!("Name {}", state.list.len() + 1),
                phone: 5555555,
            };

            let mut newList = state.list.clone();
            newList.push(newItem);

            state.set(Model {
                value: state.value,
                list: newList,
            })
        })
    };

    html! {
        <div>
        <p>{ state.value} </p>
            <button onclick={plusStateHandler} >{ "Plus" }</button>
            <button onclick={minusStateHnadler} >{ "Minus" }</button>
            <div>
            <button onclick={addPersonToList}>{ "Add New Person" }</button>
                <ListComponent list={state.list.clone()} />
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
