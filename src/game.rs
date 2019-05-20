use crate::{
    boosters::Booster,
    characters::Character,
    helpers,
    phase::{
        outcomes::{
            ActionPhaseOutcome, ActionPoints, BoosterPhaseOutcome, CharacterHeadstart,
            CharacterPhaseOutcome, DraineePhaseOutcome,
        },
        Phase, PhaseComplete,
    },
    player::{Action, CharacterlessPlayer, Choose, FinishedPlayer},
    queue::DequeueChoice,
};
use std::mem;

#[derive(Debug, Clone)]
pub struct Game {
    config: GameConfig,
    previous_phase: Option<Phase>,
    phase: Phase,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GameConfig {
    pub player_count: u8,
    pub max_points: u8,
    pub max_character_repetitions: u8,
    pub max_arsenal_items: u8,
}

impl Default for GameConfig {
    fn default() -> GameConfig {
        GameConfig {
            player_count: 2,
            max_points: 5,
            max_character_repetitions: 3,
            max_arsenal_items: 2,
        }
    }
}

impl Game {
    pub fn new(player_count: u8) -> Self {
        let config = GameConfig {
            player_count,
            max_points: 5,
            max_character_repetitions: 3,
            max_arsenal_items: 2,
        };

        Self {
            config: config.clone(),
            previous_phase: None,
            phase: Phase::Character(Self::initial_players(player_count, &config)),
        }
    }

    fn initial_players(player_count: u8, config: &GameConfig) -> Vec<CharacterlessPlayer> {
        let mut players: Vec<CharacterlessPlayer> = vec![];
        for _ in 0..player_count {
            players.push(CharacterlessPlayer::from_game_config(config.clone()))
        }
        players
    }

    pub fn choices(&self) -> Choices {
        match &self.phase {
            Phase::Character(players) => {
                Choices::Character(players.iter().map(|p| p.choices()).collect())
            }

            Phase::Booster(players) => {
                Choices::Booster(players.iter().map(|p| p.choices()).collect())
            }

            Phase::DrainedMove(players) => Choices::DrainedMove(
                players
                    .iter()
                    .map(|p| p.choices().map(|choices| choices))
                    .collect(),
            ),

            Phase::Action(players) => {
                Choices::Action(players.iter().map(|p| p.choices()).collect())
            }

            Phase::Final(_) => Choices::None,
        }
    }

    pub fn choose_character(
        &mut self,
        index: usize,
        character: Character,
    ) -> Result<CharacterPhaseOutcome, ()> {
        if let Phase::Character(players) = &mut self.phase {
            if index < players.len() {
                let player = &mut players[index];
                let result = player.choose(character);

                match result {
                    Err(_) => Err(()),
                    Ok(_) => Ok(if players.complete() {
                        let mut character_codes: Vec<u8> =
                            players.iter().map(|p| p.choice().unwrap() as u8).collect();
                        if helpers::has_duplicates(&mut character_codes) {
                            for p in players {
                                p.clear_choice().unwrap();
                            }
                            CharacterPhaseOutcome::Rechoose
                        } else {
                            let all_characters: Vec<Character> =
                                players.iter().map(|p| p.choice().unwrap()).collect();
                            let mut character_headstarts: Vec<CharacterHeadstart> = all_characters
                                .iter()
                                .map(|c| CharacterHeadstart(*c, 0))
                                .collect();
                            for (index, player) in players.iter_mut().enumerate() {
                                let mut other_characters = all_characters.clone();
                                let own_character = other_characters.remove(index);
                                for other_character in other_characters {
                                    let points = own_character.points_against(other_character);
                                    character_headstarts[index].1 += points;
                                    player.add_points(points);
                                }
                            }

                            let dummy: Vec<CharacterlessPlayer> = vec![];
                            let players = mem::replace(players, dummy);
                            self.phase = Phase::Booster(
                                players
                                    .into_iter()
                                    .map(|p| p.into_boosterless().unwrap())
                                    .collect(),
                            );

                            CharacterPhaseOutcome::Done(character_headstarts)
                        }
                    } else {
                        CharacterPhaseOutcome::Pending
                    }),
                }
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    pub fn choose_booster(
        &mut self,
        index: usize,
        booster: Booster,
    ) -> Result<BoosterPhaseOutcome, ()> {
        if let Phase::Booster(players) = &mut self.phase {
            if index < players.len() {
                let player = &mut players[index];
                let result = player.choose(booster);

                match result {
                    Err(_) => Err(()),
                    Ok(_) => Ok(if players.complete() {
                        let boosters: Vec<Booster> =
                            players.iter().map(|p| p.choice().unwrap()).collect();
                        let dummy = vec![];
                        let players = mem::replace(players, dummy);
                        self.phase = Phase::DrainedMove(
                            players
                                .into_iter()
                                .map(|p| p.into_draineeless().unwrap())
                                .collect(),
                        );

                        BoosterPhaseOutcome::Done(boosters)
                    } else {
                        BoosterPhaseOutcome::Pending
                    }),
                }
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    pub fn choose_drainee(
        &mut self,
        index: usize,
        drainee: DequeueChoice,
    ) -> Result<DraineePhaseOutcome, ()> {
        if let Phase::DrainedMove(players) = &mut self.phase {
            if index < players.len() {
                let player = &mut players[index];
                let result = player.choose(drainee);

                match result {
                    Err(_) => Err(()),
                    Ok(_) => Ok(if players.complete() {
                        let dequeue_choices: Vec<DequeueChoice> = players
                            .iter()
                            .map(|p| p.choice().unwrap().clone())
                            .collect();

                        let dummy = vec![];
                        let players = mem::replace(players, dummy);
                        self.phase = Phase::Action(
                            players
                                .into_iter()
                                .map(|p| p.into_actionless().unwrap())
                                .collect(),
                        );

                        DraineePhaseOutcome::Done(dequeue_choices)
                    } else {
                        DraineePhaseOutcome::Pending
                    }),
                }
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    pub fn choose_action(
        &mut self,
        index: usize,
        action: Action,
    ) -> Result<ActionPhaseOutcome, ()> {
        if let Phase::Action(players) = &mut self.phase {
            if index < players.len() {
                let player = &mut players[index];
                let result = player.choose(action);

                match result {
                    Err(_) => Err(()),
                    Ok(_) => Ok(if players.complete() {
                        let all_actions: Vec<Action> = players
                            .iter()
                            .map(|p| p.choice().unwrap().clone())
                            .collect();
                        let mut action_points: Vec<ActionPoints> = all_actions
                            .clone()
                            .into_iter()
                            .map(|a| ActionPoints(a, 0))
                            .collect();
                        for (index, player) in players.iter_mut().enumerate() {
                            let mut other_actions = all_actions.clone();
                            let action = other_actions.remove(index);
                            for other in other_actions {
                                let points = action.points_against(&other);
                                action_points[index].1 += points;
                                player.add_points(points);
                            }
                        }

                        if players.iter().all(|p| p.needs_points_to_win()) {
                            let dummy = vec![];
                            let players = mem::replace(players, dummy);
                            self.phase = Phase::DrainedMove(
                                players
                                    .into_iter()
                                    .map(|p| p.into_draineeless().unwrap())
                                    .collect(),
                            );
                            ActionPhaseOutcome::Done(action_points)
                        } else {
                            // TODO Handle multiple players having the needed points to win
                            let dummy = vec![];
                            let players = mem::replace(players, dummy);
                            let finished_players: Vec<FinishedPlayer> =
                                players.into_iter().map(|p| p.into_finished()).collect();
                            self.phase = Phase::Final(finished_players.clone());

                            ActionPhaseOutcome::GameOver(finished_players)
                        }
                    } else {
                        ActionPhaseOutcome::Pending
                    }),
                }
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    pub fn winner_index(&self) -> Option<usize> {
        match &self.phase {
            Phase::Final(players) => players
                .iter()
                .position(|p| p.points() >= self.config.max_points),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Choices {
    Character(Vec<Option<Vec<Character>>>),
    Booster(Vec<Option<Vec<Booster>>>),
    DrainedMove(Vec<Option<Vec<DequeueChoice>>>),
    Action(Vec<Option<Vec<Action>>>),
    None,
}

// TODO Handle destructives and single-use moves.
