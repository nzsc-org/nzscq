use super::dequeue_choiceless::DequeueChoicelessPlayer;
use crate::choices::{ArsenalItem, Booster, Character, Choose};
use crate::counters::Queue;
use crate::game::Config;
use crate::scoreboard::transparent;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoosterlessPlayer {
    pub(super) game_config: Config,
    pub(super) points: u8,
    pub(super) character: Character,
}

impl BoosterlessPlayer {
    pub fn into_dequeue_choiceless(self, booster: Booster) -> DequeueChoicelessPlayer {
        let arsenal = self.initial_arsenal(booster);

        DequeueChoicelessPlayer {
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
        let arsenal: Vec<ArsenalItem> = moves.into_iter().map(ArsenalItem::Move).collect();

        arsenal
    }
}

impl Choose<Booster> for BoosterlessPlayer {
    fn choices(&self) -> Vec<Booster> {
        self.character.boosters()
    }
}

impl Into<transparent::BoosterlessPlayer> for BoosterlessPlayer {
    fn into(self) -> transparent::BoosterlessPlayer {
        transparent::BoosterlessPlayer {
            points: self.points,
            character: self.character,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::players::CharacterlessPlayer;

    fn ninja() -> BoosterlessPlayer {
        use crate::outcomes::CharacterHeadstart;

        let player = CharacterlessPlayer::from_game_config(Config::default());
        player.into_boosterless(CharacterHeadstart(Character::Ninja, 0))
    }

    fn samurai() -> BoosterlessPlayer {
        use crate::outcomes::CharacterHeadstart;

        let player = CharacterlessPlayer::from_game_config(Config::default());
        player.into_boosterless(CharacterHeadstart(Character::Samurai, 0))
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
            player.into_dequeue_choiceless(Booster::Shadow).booster
        );
    }

    #[test]
    fn into_transparent_works() {
        let original = ninja();
        let transparent: transparent::BoosterlessPlayer = original.clone().into();

        assert_eq!(original.points, transparent.points);
        assert_eq!(original.character, transparent.character);
    }
}
