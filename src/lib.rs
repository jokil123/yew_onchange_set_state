pub use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
pub use web_sys::{Event, HtmlInputElement};
pub use yew::{use_state, UseStateHandle};

/// Macro to simplify setting the state of an input element when the value changes
///
/// # Example
/// Instead of writing:
///
/// ```rust
/// let your_state = your_state.clone();
/// move |e: Event| match e
///     .target()
///     .map(|t| t.dyn_into::<HtmlInputElement>())
///     .map(|t| t.map(|t| t.value()))
/// {
///     Some(Ok(s)) => your_state.set(s),
///     _ => {}
/// }
/// ```
///
/// You can just write:
///
/// ```rust
/// set_state!(your_state);
/// ```
///
#[macro_export]
macro_rules! set_state {
    ($state_handle: ident) => {{
        let state_handle = $state_handle.clone();
        // $crate::wasm_bindgen::dyn_into();
        (|| {
            move |e: Event| match $crate::value_from_event(e) {
                Some(s) => state_handle.set(s),
                _ => {}
            }
        })()
    }};
}

pub fn value_from_event(e: Event) -> Option<String> {
    match e
        .target()
        .map(|t| t.dyn_into::<HtmlInputElement>())
        .map(|t| t.map(|t| t.value()))
    {
        Some(Ok(s)) => Some(s),
        _ => None,
    }
}

// fn cargo_expand() {
//     let state: UseStateHandle<String> = use_state(|| "".to_string());
//     let a = set_value!(state);
// }

#[function_component(Example)]
fn example() -> Html {
    let state: UseStateHandle<String> = use_state(|| "".to_string());

    html! {
        <input type="text" onchange={
            set_state!(state)
        } value={*state}/>
    }
}
