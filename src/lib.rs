use std::collections::HashMap;

use mlua::prelude::*;
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

mod api_request;
use api_request::FunctionType;
use api_request::send_request;

static TOKIO_RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create tokio runtime")
});

async fn generate_response(
    _lua: Lua,
    (message_history, on_chunk, on_complete, error_callback): (
        Vec<HashMap<String, String>>,
        LuaFunction,
        LuaFunction,
        LuaFunction,
    ),
) -> LuaResult<()> {
    let _guard = TOKIO_RUNTIME.handle().enter();

    send_request(
        message_history,
        FunctionType::LuaFn(on_chunk),
        FunctionType::LuaFn(on_complete),
        FunctionType::LuaFn(error_callback),
    )
    .await;

    Ok(())
}

#[mlua::lua_module(name = "tools_proto_ai_proto_ai")]
fn main(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set(
        "generate_response",
        lua.create_async_function(generate_response)?,
    )?;
    Ok(exports)
}
