use mlua::prelude::LuaFunction;

#[allow(dead_code)]
pub enum FunctionType {
    LuaFn(LuaFunction),
    RustFn(Box<dyn Fn(String)>),
}
