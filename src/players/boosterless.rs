use super::draineeless::DraineelessPlayer;
use crate::choices::{ArsenalItem, Booster, Character, Choose};
use crate::counters::Queue;
use crate::game::Config;

#[derive(Debug, Clone)]
pub struct BoosterlessPlayer {
    pub(super) game_config: Config,
    pub(super) points: u8,
    pub(super) character: Character,
}

impl BoosterlessPlayer {
    pub fn into_draineeless(self, booster: Booster) -> DraineelessPlayer {
        let arsenal = self.initial_arsenal(booster);

        DraineelessPlayer {
            game_config: self.game_config,
            points: self.points,
            character: self.character,
            booster,
            arsenal,
            queue: Queue::new(),
        }
    }

    fn initial_arsenal(&self, booster: Booster) -> Vec<ArsenalItem> {
        let mut moves = self.character.moves();
        moves.extend(booster.moves());
        booster.replace_moves(&mut moves);
        let arsenal: Vec<ArsenalItem> = moves.into_iter().map(|m| ArsenalItem::Move(m)).collect();

        arsenal
    }
}

impl Choose<Booster> for BoosterlessPlayer {
    fn choices(&self) -> Vec<Booster> {
        self.character.boosters()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::players::CharacterlessPlayer;

    fn ninja() -> BoosterlessPlayer {
        let player = CharacterlessPlayer::from_game_config(Config::default());
        player.into_boosterless(Character::Ninja)
    }

    fn samurai() -> BoosterlessPlayer {
        let player = CharacterlessPlayer::from_game_config(Config::default());
        player.into_boosterless(Character::Samurai)
    }

    #[test]
    fn ninja_can_choose_shadow() {
        let player = ninja();
        assert_eq!(Character::Ninja.boosters(), player.choices());
    }

    #[test]
    fn ninja_cannot_choose_atlas() {
        let player = ninja();
        assert!(!player.choices().contains(&Booster::Atlas));
    }

    #[test]
    fn shadow_initial_arsenal_is_correct() {
        use crate::choices::Move;

        let player = ninja();
        let arsenal = player.initial_arsenal(Booster::Shadow);
        assert_eq!(
            arsenal,
            vec![
                ArsenalItem::Move(Move::Kick),
                ArsenalItem::Move(Move::NinjaSword),
                ArsenalItem::Move(Move::Nunchucks),
                ArsenalItem::Move(Move::ShadowFireball),
                ArsenalItem::Move(Move::ShadowSlip)
            ]
        );
    }

    #[test]
    fn atlas_initial_arsenal_is_correct() {
        use crate::choices::Move;

        let player = samurai();
        let arsenal = player.initial_arsenal(Booster::Atlas);
        assert_eq!(
            arsenal,
            vec![
                ArsenalItem::Move(Move::SamuraiSword),
                ArsenalItem::Move(Move::Helmet),
                ArsenalItem::Move(Move::Smash),
                ArsenalItem::Move(Move::Lightning),
                ArsenalItem::Move(Move::Earthquake)
            ]
        );
    }

    #[test]
    fn strong_initial_arsenal_is_correct() {
        use crate::choices::Move;

        let player = samurai();
        let arsenal = player.initial_arsenal(Booster::Strong);
        assert_eq!(
            arsenal,
            vec![
                ArsenalItem::Move(Move::SamuraiSword),
                ArsenalItem::Move(Move::Helmet),
                ArsenalItem::Move(Move::Twist),
                ArsenalItem::Move(Move::Bend),
                ArsenalItem::Move(Move::StrongSmash),
            ]
        );
    }

    #[test]
    fn into_draineeless_works() {
        let player = ninja();
        assert_eq!(
            Booster::Shadow,
            player.into_draineeless(Booster::Shadow).booster
        );
    }
}
