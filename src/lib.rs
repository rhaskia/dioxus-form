mod serializer;
mod deserializer;
use serde::{ser::Serialize, de::Deserialize};
use dioxus::prelude::*;
use serializer::create_form;
use std::fmt::{Display, Debug};

/// Dioxus Component for automatically creating a html form
/// Simply pass a Signal into the Component and the Signal will be automatically updated as the
/// form is updated.
///
/// ```rs 
/// struct FormStruct {
///     int: i32,
///     string: String,
///     b: bool,
/// }
///
/// #[component]
/// fn App() -> Element {
///     let signal = use_signal(|| FormStruct { int: 2, string: String::new(), b: true });
///
///     rsx! {
///         Form { value: signal }
///     }
/// }
/// ```
#[component]
pub fn Form<T: Clone + Serialize + 'static + PartialEq + for<'de> Deserialize<'de>>(value: Signal<T>) -> Element {
    let html = use_signal(|| create_form(value));

    rsx! {
        form {
            oninput: move |i| {
                let values = i.values();
                let result: Result<T, Error> = deserializer::from_values(values); 
                match result {
                    Ok(v) => value.set(v),
                    Err(e) => panic!("{e:?}"),
                }
            },
            dangerous_inner_html: html.read().clone()?,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    _message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { Display::fmt(&self, f) }
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error {
            _message: msg.to_string(),
        }
    }
}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error {
            _message: msg.to_string(),
        }
    }
}

impl serde::ser::StdError for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> { None }

    fn description(&self) -> &str { "description() is deprecated; use Display" }

    fn cause(&self) -> Option<&dyn std::error::Error> { self.source() }

    //fn provide<'a>(&'a self, request: &mut std::error::Request<'a>) {}
}
