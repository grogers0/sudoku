mod solver;
mod technique;
mod techniques;

pub use technique::Technique;
pub use techniques::{ALL, FAST};
pub use solver::{solve, SolveOpts, SolveResult};

