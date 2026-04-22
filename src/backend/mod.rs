#[cfg(not(any(feature = "tokio", feature = "async-io")))]
compile_error!("'tokio' or 'async-io' feature has to be enabled in order to use one backend");

#[cfg(feature = "crossterm")]
mod crossterm;
#[cfg(feature = "crossterm")]
pub use crossterm::*;
