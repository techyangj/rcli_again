mod cli;
mod process;
pub use cli::base64::Base64SubCommand;
pub use cli::{Opts, SubCommand};
pub use process::{process_csv, process_decode, process_encode, process_genpass};
