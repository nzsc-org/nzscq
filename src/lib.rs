pub mod choices;
pub mod game;
pub mod outcomes;

pub mod prelude {
    pub use crate::{choices::*, game::*, outcomes::*};
}

mod counters;
mod helpers;
mod players;
