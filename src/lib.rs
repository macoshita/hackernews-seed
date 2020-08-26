// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async {
        Msg::Fetching
    });

    Model::default()
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
#[derive(Default)]
pub struct Model {
    items: Option<Vec<Item>>
}

#[derive(Debug)]
pub struct Item {
    id: i64,
}

// ------ ------
//    Update
// ------ ------

// `Msg` describes the different events you can modify state with.
#[derive(Debug)]
pub enum Msg {
    Fetching,
    Fetched(Vec<Item>),
}

// `update` describes how to handle each `Msg`.
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Fetching => {
            orders.perform_cmd(async {
                let resp = fetch("https://hacker-news.firebaseio.com/v0/topstories.json").await.expect("Failed to request");
                let ids = resp.check_status().expect("Failed to request").json::<Vec<i64>>().await.expect("Failed to request");
                let (ids, _) = ids.split_at(10);
                let result = ids.iter().map(|&id| 
                    Item {
                        id: id,
                    }
                ).collect::<Vec<Item>>();
                Msg::Fetched(result);
            });
        }

        Msg::Fetched(items) => {
            log(&items);
            model.items = Some(items);
        }
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
pub fn view(model: &Model) -> Node<Msg> {
    div![
        ul![
            model.items.as_ref().map(|items| {
                li!["yondayo"]
            })
        ]
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
