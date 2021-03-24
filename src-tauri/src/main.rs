#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use serde::{Deserialize, Serialize};
mod cmd;

#[derive(Serialize)]
struct Response<'a> {
  code: &'a str,
}

// An error type we define
// We could also use the `anyhow` lib here
#[derive(Debug, Clone)]
struct CommandError<'a> {
  message: &'a str,
}

impl<'a> CommandError<'a> {
  fn new(message: &'a str) -> Self {
    Self { message }
  }
}

impl<'a> std::fmt::Display for CommandError<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

// Tauri uses the `anyhow` lib so custom error types must implement std::error::Error
// and the function call should call `.into()` on it
impl<'a> std::error::Error for CommandError<'a> {}

fn main() {
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => {
          Err(e.to_string())
        }
        Ok(command) => {
          match command {
            Generate { filename, callback, error } => tauri::execute_promise(
              _webview,
              move || {
                let code = "secret-code-time";
                let response = Response {
                  code: code 
                };
                
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                ctx.set_contents(code.to_owned()).unwrap();
                Ok(response)
              },
              callback,
              error,
            ),
          }
          Ok(())
        }
    }
  })
  .build()
  .run();
}