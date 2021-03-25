use anyhow;

#[derive(Debug)]
pub enum DanaError {
  Wormhole { error: anyhow::Error },
}

impl std::error::Error for DanaError {}

impl std::fmt::Display for DanaError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      DanaError::Wormhole { error } => write!(f, "{}", error),
    }
  }
}
