mod models;
use futures_util::StreamExt;
pub use models::FunctionType;
pub use models::ResponseModel;

use mlua::prelude::*;
use reqwest_eventsource::{Event, EventSource};
use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(untagged)]
enum MessageValue {
    Role(String),
    Parts(Vec<HashMap<String, String>>),
}

#[derive(Serialize, Debug)]
struct Payload {
    contents: Vec<HashMap<String, MessageValue>>,
    system_instruction: HashMap<String, Vec<HashMap<String, String>>>,
    tools: Vec<HashMap<String, HashMap<String, String>>>,
}

pub async fn send_request(
    message_history: Vec<HashMap<String, String>>,
    on_chunk: FunctionType,
    on_complete: FunctionType,
    error_callback: FunctionType,
) {
    // construct important variables
    let model: &str = "gemini-2.5-flash-preview-04-17";

    // get api key from environment variable
    let api_key = match std::env::var("GOOGLE_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            let error_message =
                "API key not found, considering add it in your environment variable.\n`export GOOGLE_API_KEY=YOUR_API_KEY`".to_string();
            call_function(error_message, &error_callback);
            return;
        }
    };

    // construct request
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:streamGenerateContent",
        model
    );

    let params: HashMap<String, String> = HashMap::from([
        ("key".to_string(), api_key),
        ("alt".to_string(), "sse".to_string()),
    ]);

    // construct payload from data
    let mut system_instructions: HashMap<String, Vec<HashMap<String, String>>> =
        HashMap::from([("parts".to_string(), vec![])]);
    let mut message_turns: Vec<HashMap<String, MessageValue>> = Vec::new();

    for content in message_history.iter() {
        let role: &String = match content.get("role") {
            Some(r) => r,
            None => {
                call_function(
                    "Role not found in message history.".to_string(),
                    &error_callback,
                );
                return;
            }
        };

        let content: &String = match content.get("content") {
            Some(c) => c,
            None => {
                call_function(
                    "Content not found in message history.".to_string(),
                    &error_callback,
                );
                return;
            }
        };

        // add instruction if system
        if role == "system" {
            system_instructions
                .get_mut("parts")
                .unwrap()
                .push(HashMap::from([("text".to_string(), content.to_string())]))
        } else if role == "assistant" {
            message_turns.push(HashMap::from([
                ("role".to_string(), MessageValue::Role("model".to_string())),
                (
                    "parts".to_string(),
                    MessageValue::Parts(vec![HashMap::from([(
                        "text".to_string(),
                        content.to_string(),
                    )])]),
                ),
            ]))
        } else if role == "user" {
            message_turns.push(HashMap::from([
                ("role".to_string(), MessageValue::Role("user".to_string())),
                (
                    "parts".to_string(),
                    MessageValue::Parts(vec![HashMap::from([(
                        "text".to_string(),
                        content.to_string(),
                    )])]),
                ),
            ]))
        } else {
            call_function(
                format!("Unknown role: `{}` from message history.", role),
                &error_callback,
            );
            return;
        }
    }

    // construct payload to reqwest
    let payload = Payload {
        contents: message_turns,
        system_instruction: system_instructions,
        tools: vec![HashMap::from([(
            "google_search".to_string(),
            HashMap::new(), // simulating empty dictionary
        )])],
    };

    // TODO
    // change hashmap into structs, they can Deserialize by serde later

    // create async client
    let client = reqwest::Client::new();

    // send request
    let response = client
        .post(&url)
        .header("Content-Type", "text/event-stream")
        .query(&params)
        .json(&payload);

    // let the event source take over with the streaming data
    let sse = EventSource::new(response);

    if let Err(e) = sse {
        call_function(
            format!("Could not connect to event source: {}", e.to_string()),
            &error_callback,
        );
        return;
    }

    let mut sse = sse.unwrap();

    // send notice to lua callback that everything is ready
    call_function("...".to_string(), &on_chunk);

    let mut chunk_buffer = String::new();
    while let Some(event) = sse.next().await {
        match event {
            Ok(Event::Open) => {
                // I don't know why this is not called in lua
                // call_function("...".to_string(), &on_chunk);
            }
            Ok(Event::Message(message)) => {
                let response_model: ResponseModel = match serde_json::from_str(&message.data) {
                    Ok(parsed) => parsed,
                    Err(e) => {
                        call_function(
                            format!(
                                "Error parsing JSON: {}\n\ndata:\n{}",
                                e.to_string(),
                                message.data
                            ),
                            &error_callback,
                        );
                        return;
                    }
                };

                chunk_buffer.push_str(&response_model.candidates[0].content.parts[0].text);

                call_function(chunk_buffer.to_string(), &on_chunk);
            }
            Err(e) => {
                let error_message = e.to_string();
                if error_message != "Stream ended".to_string() {
                    call_function(
                        format!(
                            "Unknown error occured while streaming sse: {}",
                            error_message
                        ),
                        &error_callback,
                    );
                    return;
                }
                sse.close();
            }
        }
    }

    call_function(chunk_buffer.to_string(), &on_complete);
}

fn call_function(message: String, function: &FunctionType) {
    match function {
        FunctionType::LuaFn(lua_fn) => {
            let _ = lua_fn.call::<LuaString>(message);
        }
        FunctionType::RustFn(fn_string) => {
            fn_string(message);
        }
    };
}
