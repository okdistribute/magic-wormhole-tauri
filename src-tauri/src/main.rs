#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use serde::{Deserialize, Serialize};
use serde_json::json;
use futures::executor::block_on;
use hex;

mod dana;
mod cmd;

#[derive(Serialize)]
struct Response {
  message: String
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
        Err(e) => Err(e.to_string()),
        Ok(command) => {
          match command {
            GenerateCode { filename, callback, error } => tauri::execute_promise(
              _webview,
              move || {
                let code = block_on(dana::Code::new(None));
                let s = code.welcome.code.0.clone();
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                ctx.set_contents(code.welcome.code.0.clone()).unwrap();
                println!("code {}", s);
                let hole= block_on(code.connect());
                Ok(Response {
                  message: s
                })
              },
              callback,
              error,
            ),
            RedeemCode { code, callback, error } => tauri::execute_promise(
              _webview, 
              move || {
                let code = block_on(dana::Code::new(Some(code)));
                let hole = block_on(code.connect());

                Ok(Response {
                  message: hex::encode(hole.key.0)
                })
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