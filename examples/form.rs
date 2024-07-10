use dioxus::prelude::*;
use dioxus_form::Form;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
struct Example {
    pub amount: u64,
    pub vector: Vec<u64>,
    pub string: String,
    pub boolean: bool,
    pub nested: NestedExample,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
struct NestedExample {
    pub tuple: (u64, u64),
}

#[component]
pub fn App() -> Element {
    let example = use_signal(|| Example {
        amount: 19,
        vector: vec![1, 2, 3, 4, 5],
        string: String::from("Dioxus!"),
        boolean: true,
        nested: NestedExample { tuple: (19, 67) },
    });

    rsx! {
        Form { value: example }
    }
}

pub fn main() {
    launch(App)
}
