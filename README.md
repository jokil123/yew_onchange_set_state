# yew_onchange_set_state
A small utility macro to update state when the onchange event is fired in yew.

When working with yew, often times you need to update the state whenever the value of the input is changed. This can be rather verbose for something that has to be repeated often.

## Example

Normally you would have to write sth similar to this:
```rust
#[function_component(Example)]
fn example() -> Html {
    let state: UseStateHandle<String> = use_state(|| "".to_string());

    html! {
        <input type="text" onchange={
            let your_state = your_state.clone();
            move |e: Event| match e
              .target()
              .map(|t| t.dyn_into::<HtmlInputElement>())
              .map(|t| t.map(|t| t.value()))
            {
              Some(Ok(s)) => your_state.set(s),
              _ => {}
            }
        } value={*state}/>
    }
}
```

With this crate you can just write this instead:
```rust
#[function_component(Example)]
fn example() -> Html {
    let state: UseStateHandle<String> = use_state(|| "".to_string());

    html! {
        <input type="text" onchange={set_state!(state)} value={*state}/>
    }
}
```
