#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use futures::executor::block_on;
use hex;
use serde::Serialize;

mod cmd;
mod dana;
mod errors;

#[derive(Serialize)]
struct Response {
  message: String,
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
            GenerateCode {
              filename,
              callback,
              error,
            } => tauri::execute_promise(
              _webview,
              move || {
                let peer = block_on(dana::create_code()).unwrap();
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                let code = peer.code;
                println!("code {}", &code);
                ctx.set_contents(code.clone()).unwrap();
                let hole = block_on(peer.connector.connect_to_client()).unwrap();
                let crypto_key = hex::encode(hole.key.0);
                Ok(Response {
                  message: crypto_key,
                })
              },
              callback,
              error,
            ),
            RedeemCode {
              code,
              callback,
              error,
            } => tauri::execute_promise(
              _webview,
              move || {
                let peer = block_on(dana::redeem_code(code)).unwrap();
                let hole = block_on(peer.connector.connect_to_client()).unwrap();
                let crypto_key = hex::encode(hole.key.0);

                Ok(Response {
                  message: crypto_key,
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
