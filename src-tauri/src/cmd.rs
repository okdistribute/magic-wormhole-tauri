use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  // your custom commands
  // multiple arguments are allowed
  // note that rename_all = "camelCase": you need to use "myCustomCommand" on JS
  GenerateCode {
    filename: String,
    callback: String,
    error: String,
  },

  RedeemCode {
    code: String,
    callback: String,
    error: String,
  },

  Heartbeat {
    callback: String,
    error: String,
  },
}
