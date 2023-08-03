mod echo;

use crate::model::applet::AppletRunner;
use bytesize::ByteSize;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

const CODE_B64_MAX_LENGTH: u64 = 2_097_152; // 2 MBytes
const RENDERED_PAGE_MAX_LENGTH: u64 = 2_097_152; // 2 MBytes
const MAX_RUNNING_TIME: Duration = Duration::from_secs(10); // 10 seconds

impl AppletRunner {
    fn run_code(self, lua_outcome: Arc<Mutex<String>>) -> design_scaffold::Result<()> {
        use mlua::Lua;
        let (applet_method, applet_query_string, applet_cookies, applet_form, artifact) =
            self.take();
        let (applet_oid, _, code, _, _, _) = artifact.take().take();
        let code_in_base64 = code.take();
        // * SECURITY_LIMIT
        if code_in_base64.len() > CODE_B64_MAX_LENGTH.try_into()? {
            return Err(design_scaffold::Error::AppletRunnerValidation(format!(
                "Code is larger than {}.",
                ByteSize::b(CODE_B64_MAX_LENGTH)
            )));
        }

        let bytes_from_b64 = base64::decode(code_in_base64)?;
        let code = String::from_utf8(bytes_from_b64)?;

        let lua = Lua::new();
        let globals = lua.globals();
        // * Set Globals
        let req_applet_oid = applet_oid.take();
        let req_applet_method = applet_method.as_str();

        globals.set("applet_oid", req_applet_oid)?;
        globals.set("request_method", req_applet_method)?;

        if let Some(req_applet_query_string) = applet_query_string {
            let query_string = req_applet_query_string.take();
            let table = lua.create_table_from(query_string)?;
            globals.set("request_query_string", table)?;
        }

        if let Some(req_applet_cookies) = applet_cookies {
            let cookies = req_applet_cookies.take();
            let table = lua.create_table_from(cookies)?;
            globals.set("request_cookies", table)?;
        }

        if let Some(req_applet_form) = applet_form {
            let form = req_applet_form.take();
            let table = lua.create_table_from(form)?;
            globals.set("request_form", table)?;
        }

        // * Set Functions
        echo::make_fn(&lua, &globals, lua_outcome)?;

        lua.load(&code).exec()?;
        Ok(())
    }
    pub fn run(self) -> design_scaffold::Result<String> {
        use std::str::FromStr;
        let global_output: String;
        let global_outcome = Arc::new(Mutex::new("".to_string()));
        // TODO: Implement some security guarantees
        let (channel_sender, channel_receiver) = mpsc::channel();
        thread::spawn(move || {
            let lua_outcome = Arc::clone(&global_outcome);

            if let Err(error) = self.run_code(lua_outcome) {
                channel_sender.send(Err(error))?;
            }

            let outcome_from_lua = Arc::clone(&global_outcome);
            let mtx_string = match outcome_from_lua.lock() {
                Ok(data) => data,
                Err(error) => {
                    let brickpack_error =
                        design_scaffold::Error::MluaError(mlua::Error::RuntimeError(format!(
                            "Unable to catch program output. Reason: {}",
                            error
                        )));
                    return channel_sender.send(Err(brickpack_error));
                }
            };

            let output = match String::from_str(&mtx_string) {
                Ok(data) => data,
                Err(error) => {
                    let brickpack_error =
                        design_scaffold::Error::MluaError(mlua::Error::RuntimeError(format!(
                            "Unable to catch program output. Reason: {}",
                            error
                        )));
                    return channel_sender.send(Err(brickpack_error));
                }
            };
            channel_sender.send(Ok(output))
        });

        // * SECURITY_LIMIT
        match channel_receiver.recv_timeout(MAX_RUNNING_TIME) {
            Ok(data) => match data {
                Ok(output_from_lua) => global_output = output_from_lua,
                Err(error) => {
                    // TODO:
                    dbg!(&error);
                    return Err(design_scaffold::Error::MluaError(mlua::Error::RuntimeError(
                        "Unable to catch program output".to_string(),
                    )));
                }
            },
            Err(error) => {
                // TODO:
                dbg!(&error);
                return Err(design_scaffold::Error::MluaError(mlua::Error::RuntimeError(
                    "Timeout error".to_string(),
                )));
            }
        }

        Ok(global_output)
    }
}
