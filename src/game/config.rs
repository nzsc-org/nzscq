use crate::players::ActionlessPlayer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub player_count: u8,
    pub max_points: u8,
    pub max_character_repetitions: u8,
    pub max_arsenal_items: u8,
}

impl Config {
    pub(super) fn clamp_points(&self, players: &mut Vec<ActionlessPlayer>) -> bool {
        let max = players.iter().map(|p| p.points()).max().unwrap();
        if max >= self.max_points {
            let players_with_max = players.iter().filter(|p| p.points() == max).count();
            if players_with_max > 1 {
                for p in players {
                    p.deduct_points(max - self.max_points - 1);
                }

                false
            } else {
                for p in players {
                    p.deduct_points(max - self.max_points);
                }

                true
            }
        } else {
            false
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            player_count: 2,
            max_points: 5,
            max_character_repetitions: 3,
            max_arsenal_items: 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_default_works() {
        let _ = Config::default();
    }
}
