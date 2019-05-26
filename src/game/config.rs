#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub player_count: u8,
    pub points_to_win: u8,
    pub max_character_repetitions: u8,
    pub max_arsenal_items: u8,
}

impl Config {
    pub(super) fn deductions(&self, points: Vec<u8>) -> Vec<u8> {
        if let Some(max_points) = points.iter().max() {
            if max_points < &self.points_to_win {
                vec![0; points.len()]
            } else {
                let diff = max_points - self.points_to_win;
                let tie_exists = points.iter().filter(|p| p == &max_points).count() > 1;
                if tie_exists {
                    vec![diff + 1; points.len()]
                } else {
                    vec![diff; points.len()]
                }
            }
        } else {
            vec![]
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            player_count: 2,
            points_to_win: 5,
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

    #[test]
    fn deducts_nothing_if_all_players_points_less_than_winning_amount() {
        const PLAYER_COUNT: u8 = 3;
        let config = Config {
            player_count: PLAYER_COUNT,
            ..Config::default()
        };
        let one_less = config.points_to_win - 1;

        assert_eq!(
            vec![0; PLAYER_COUNT as usize],
            config.deductions(vec![one_less; PLAYER_COUNT as usize])
        );
    }

    #[test]
    fn deducts_diff_if_one_player_has_points_greater_than_winning_amount() {
        const PLAYER_COUNT: u8 = 3;
        let config = Config {
            player_count: PLAYER_COUNT,
            ..Config::default()
        };
        let winning = config.points_to_win;
        let diff = 1;
        let diff_more = winning + diff;

        assert_eq!(
            vec![diff; PLAYER_COUNT as usize],
            config.deductions(vec![winning, winning, diff_more])
        );
    }

    #[test]
    fn deducts_zero_if_one_player_has_points_equal_to_winning_amount() {
        const PLAYER_COUNT: u8 = 3;
        let config = Config {
            player_count: PLAYER_COUNT,
            ..Config::default()
        };
        let winning = config.points_to_win;
        let diff = 0;
        let one_less = winning - 1;

        assert_eq!(
            vec![diff; PLAYER_COUNT as usize],
            config.deductions(vec![winning, one_less, one_less])
        );
    }

    #[test]
    fn deducts_diff_plus_one_if_multiple_players_have_more_points_than_winning_amount() {
        const PLAYER_COUNT: u8 = 3;
        let config = Config {
            player_count: PLAYER_COUNT,
            ..Config::default()
        };
        let winning = config.points_to_win;
        let diff = 1;
        let diff_more = winning + diff;

        assert_eq!(
            vec![diff + 1; PLAYER_COUNT as usize],
            config.deductions(vec![winning, diff_more, diff_more])
        );
    }

    #[test]
    fn deducts_one_if_multiple_players_are_tied_for_first_and_have_points_equal_to_winning_amount()
    {
        const PLAYER_COUNT: u8 = 3;
        let config = Config {
            player_count: PLAYER_COUNT,
            ..Config::default()
        };
        let winning = config.points_to_win;
        let diff = 0;
        let one_less = winning - 1;

        assert_eq!(
            vec![diff + 1; PLAYER_COUNT as usize],
            config.deductions(vec![winning, winning, one_less])
        );
    }
}
