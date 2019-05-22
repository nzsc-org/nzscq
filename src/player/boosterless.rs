use super::draineeless::DraineelessPlayer;
use crate::choices::{ArsenalItem, Booster, CanChoose, Character, Choose};
use crate::counters::Queue;
use crate::game::GameConfig;

#[derive(Debug, Clone)]
pub struct BoosterlessPlayer {
    pub(super) game_config: GameConfig,
    pub(super) points: u8,
    pub(super) character: Character,
    pub(super) pending_booster: Option<Booster>,
}

impl BoosterlessPlayer {
    pub fn into_draineeless(self) -> Result<DraineelessPlayer, ()> {
        if let Some(booster) = self.pending_booster {
            let arsenal = self.initial_arsenal(booster);

            Ok(DraineelessPlayer {
                game_config: self.game_config,
                points: self.points,
                character: self.character,
                booster,
                arsenal,
                queue: Queue::new(),
                choice: None,
            })
        } else {
            Err(())
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
    fn choices(&self) -> Option<Vec<Booster>> {
        if self.has_chosen() {
            None
        } else {
            Some(self.character.boosters())
        }
    }

    fn choose(&mut self, booster: Booster) -> Result<(), ()> {
        if self.can_choose(&booster) {
            self.pending_booster = Some(booster);
            Ok(())
        } else {
            Err(())
        }
    }

    fn choice(&self) -> Option<Booster> {
        self.pending_booster
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::CharacterlessPlayer;

    fn ninja() -> BoosterlessPlayer {
        let mut player = CharacterlessPlayer::from_game_config(GameConfig::default());
        player.choose(Character::Ninja).unwrap();
        player.into_boosterless().unwrap()
    }

    fn samurai() -> BoosterlessPlayer {
        let mut player = CharacterlessPlayer::from_game_config(GameConfig::default());
        player.choose(Character::Samurai).unwrap();
        player.into_boosterless().unwrap()
    }

    #[test]
    fn ninja_can_choose_shadow() {
        let mut player = ninja();
        assert_eq!(player.choices(), Some(Character::Ninja.boosters()));
        assert!(player.choose(Booster::Shadow).is_ok());
        assert_eq!(player.choice(), Some(Booster::Shadow));
    }

    #[test]
    fn ninja_cannot_choose_atlas() {
        let mut player = ninja();
        assert_eq!(player.choices(), Some(Character::Ninja.boosters()));
        assert!(player.choose(Booster::Atlas).is_err());
        assert_eq!(player.choice(), None);
    }

    #[test]
    fn cannot_choose_if_has_already_choose() {
        let mut player = ninja();
        player.choose(Booster::Shadow).unwrap();
        assert_eq!(player.choices(), None);
        assert!(player.choose(Booster::Speedy).is_err());
        assert_eq!(player.choice(), Some(Booster::Shadow));
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
    fn into_draineeless_works_if_player_has_chosen() {
        let mut player = ninja();
        player.choose(Booster::Shadow).unwrap();
        assert!(player.into_draineeless().is_ok());
    }

    #[test]
    fn into_draineeless_fails_if_player_has_not_chosen() {
        let player = ninja();
        assert!(player.into_draineeless().is_err());
    }
}
