use std::collections::HashMap;
use std::io;
use std::io::Write;

mod api_request;
use api_request::FunctionType;

use api_request::send_request;

// simulate function calling from lua
fn on_chunk(chunk: String) {
    println!("{}", chunk);
    io::stdout().flush().unwrap();
}
fn on_complete(_full_response: String) {}
fn error_callback(error: String) {
    println!("Error: {}", error);
}

// Note:
// In order to run this example for test,
// - change mlua feature to vendored
// - remove mlua macro in lib.rs method main

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // simulate parameters from lua
    let mut message_history: Vec<HashMap<String, String>> = vec![];

    message_history.push(HashMap::from([
        ("role".to_string(), "system".to_string()),
        (
            "content".to_string(),
            "You are a helpful assistant.".to_string(),
        ),
    ]));

    message_history.push(HashMap::from([
        ("role".to_string(), "user".to_string()),
        ("content".to_string(), "Hello".to_string()),
    ]));

    message_history.push(HashMap::from([
        ("role".to_string(), "assistant".to_string()),
        ("content".to_string(), "Hello, world!".to_string()),
    ]));

    message_history.push(HashMap::from([
        ("role".to_string(), "user".to_string()),
        (
            "content".to_string(),
            "Can you write me a 10 paragraph essay about LLM.".to_string(),
        ),
    ]));

    send_request(
        message_history,
        FunctionType::RustFn(Box::new(on_chunk)),
        FunctionType::RustFn(Box::new(on_complete)),
        FunctionType::RustFn(Box::new(error_callback)),
    )
    .await;

    Ok(())
}
