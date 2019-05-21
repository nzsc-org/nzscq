pub mod game;
pub mod prelude {
    pub use crate::game::{Game, GameConfig};
}

pub mod choices;
pub mod counters;
pub mod outcomes;
pub mod player;

mod helpers;
