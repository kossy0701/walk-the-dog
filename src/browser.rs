use anyhow::{anyhow, Result};

// Straight taken from https://rustwasm.github.io/book/game-of-life/debugging.html
macro_rules! log {
  ( $( $t:tt )* ) => {
      web_sys::console::log_1(&format!( $( $t )* ).into());
  }
}