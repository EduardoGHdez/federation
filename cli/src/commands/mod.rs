pub mod print;
pub mod setup;
pub mod update;

pub use print::Print;
pub use setup::Setup;
pub use update::Update;

use crate::errors::{ExitCode, Fallible};
use crate::telemetry::Session;

pub trait Command {
    /// Executes the command. Returns `Ok(true)` if the process should return 0,
    /// `Ok(false)` if the process should return 1, and `Err(e)` if the process
    /// should return `e.exit_code()`.
    fn run(&self, session: &mut Session) -> Fallible<ExitCode>;
}