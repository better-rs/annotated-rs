//! Substrate Node Template CLI library.
#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;
mod rpc;

////////////////////////////////////////////////////////////////////////////////

///
/// todo x:
///
fn main() -> sc_cli::Result<()> {
	///
	/// todo x: 入口
	///
	command::run()
}
