mod greeter;
mod utils;

use actions_toolkit_bindings::core;
use js_sys::JsString;
use std::error::Error;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Perhaps we need a hook that calls core::set_failed() on panic.
    // This would make sure the action outputs an error command for
    // the runner and returns exit code 1.
    utils::set_panic_hook();

    // Unhandled errors raised by the action call set_failed to output
    // an error command for the runner and return exit code 1.
    if let Err(e) = run() {
        let msg = format!("{}", e);
        core::set_failed(&JsString::from(msg));
    }
    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    // Get the action input.
    let actor = core::get_input(&"actor".into(), None)
        .as_string()
        .unwrap_or_default();

    // Greet the workflow actor.
    let greeting = greeter::greet(&actor);
    core::info(&greeting.into());

    // Set the action output.
    core::set_output(&"result".into(), &"success".into());

    panic!("blah")

    //Ok(())
}
