use mlua::{Lua, Table};
use std::sync::{Arc, Mutex};

pub(super) fn make_fn(
    lua: &Lua,
    globals: &Table,
    lua_outcome: Arc<Mutex<String>>,
) -> design_scaffold::Result<()> {
    use super::RENDERED_PAGE_MAX_LENGTH;

    let echo = move |_vm, message: String| -> Result<(), mlua::Error> {
        let echo_outcome = Arc::clone(&lua_outcome);
        let mut mtx_string = match echo_outcome.lock() {
            Ok(buffer) => buffer,
            Err(error_from_mutex) => {
                return Err(mlua::Error::MemoryError(error_from_mutex.to_string()));
            }
        };
        let page_max_length: usize = match RENDERED_PAGE_MAX_LENGTH.try_into() {
            Ok(max_length) => max_length,
            Err(error) => {
                tracing::error!(
                    "Error on RENDERED_PAGE_MAX_LENGTH.try_into() at `make_fn() for `echo`. Reason: {error:?}"
                );
                return Err(mlua::Error::MemoryLimitNotAvailable);
            }
        };

        if (mtx_string.len() + message.len()) > page_max_length {
            return Err(mlua::Error::SafetyError(
                "Safety error: Rendered page cannot be larger than 2 Mbytes.".to_string(),
            ));
        }
        let msg = message.as_str();
        mtx_string.push_str(msg);
        Ok(())
    };
    globals.set("echo", lua.create_function(echo)?)?;

    Ok(())
}
