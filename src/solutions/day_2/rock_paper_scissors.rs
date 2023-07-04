// https://adventofcode.com/2022/day/2

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Action {
    ROCK,
    PAPER,
    SCISSORS,
}

impl Action {
    fn point(&self) -> i32 {
        let point = match self {
            Action::ROCK => 1,
            Action::PAPER => 2,
            Action::SCISSORS => 3,
        };

        return point;
    }
}

pub struct Round {
    pub round: i32,
    pub player_1: GamePlay,
    pub player_2: GamePlay,
}

#[derive(Debug)]
pub struct GamePlay {
    pub action: Action,
    pub player_name: String,
}

#[derive(Debug)]
pub struct Result {
    pub player_name: String,
    pub score: i32,
    pub action: Action,
}

pub struct Game {
    pub player_1: String,
    pub player_2: String,
    pub results: HashMap<i32, Vec<Result>>,
}

impl Game {
    pub fn new(player_1_name: &str, player_2_name: &str) -> Self {
        return Game {
            player_1: player_1_name.to_string(),
            player_2: player_2_name.to_string(),
            results: HashMap::new(),
        };
    }

    fn record_gameplay(&mut self, round: i32, player_name: String, score: i32, action: Action) {
        let entry = self.results.entry(round).or_insert_with(Vec::new);

        entry.push(Result {
            player_name: player_name,
            score,
            action,
        });
    }

    pub fn new_round(&mut self, round: Round) {
        match (&round.player_1.action, &round.player_2.action) {
            (Action::ROCK, Action::SCISSORS) | (Action::SCISSORS, Action::ROCK) => {
                if round.player_1.action == Action::ROCK {
                    self.record_gameplay(
                        round.round,
                        round.player_1.player_name,
                        round.player_1.action.point() + 6,
                        round.player_1.action,
                    );

                    self.record_gameplay(
                        round.round,
                        round.player_2.player_name,
                        round.player_2.action.point() + 0,
                        round.player_2.action,
                    );
                } else if round.player_2.action == Action::ROCK {
                    self.record_gameplay(
                        round.round,
                        round.player_2.player_name,
                        round.player_2.action.point() + 6,
                        round.player_2.action,
                    );

                    self.record_gameplay(
                        round.round,
                        round.player_1.player_name,
                        round.player_1.action.point() + 0,
                        round.player_1.action,
                    );
                }
            }
            (Action::SCISSORS, Action::PAPER) | (Action::PAPER, Action::SCISSORS) => {
                if round.player_1.action == Action::SCISSORS {
                    self.record_gameplay(
                        round.round,
                        round.player_1.player_name,
                        round.player_1.action.point() + 6,
                        round.player_1.action,
                    );

                    self.record_gameplay(
                        round.round,
                        round.player_2.player_name,
                        round.player_2.action.point() + 0,
                        round.player_2.action,
                    );
                } else if round.player_2.action == Action::SCISSORS {
                    self.record_gameplay(
                        round.round,
                        round.player_2.player_name,
                        round.player_2.action.point() + 6,
                        round.player_2.action,
                    );

                    self.record_gameplay(
                        round.round,
                        round.player_1.player_name,
                        round.player_1.action.point() + 0,
                        round.player_1.action,
                    );
                }
            }
            (Action::PAPER, Action::ROCK) | (Action::ROCK, Action::PAPER) => {
                if round.player_1.action == Action::PAPER {
                    self.record_gameplay(
                        round.round,
                        round.player_1.player_name,
                        round.player_1.action.point() + 6,
                        round.player_1.action,
                    );

                    self.record_gameplay(
                        round.round,
                        round.player_2.player_name,
                        round.player_2.action.point() + 0,
                        round.player_2.action,
                    );
                } else if round.player_2.action == Action::PAPER {
                    self.record_gameplay(
                        round.round,
                        round.player_2.player_name,
                        round.player_2.action.point() + 6,
                        round.player_2.action,
                    );

                    self.record_gameplay(
                        round.round,
                        round.player_1.player_name,
                        round.player_1.action.point() + 0,
                        round.player_1.action,
                    );
                }
            }
            _ => {
                self.record_gameplay(
                    round.round,
                    round.player_1.player_name,
                    round.player_1.action.point() + 3,
                    round.player_1.action,
                );

                self.record_gameplay(
                    round.round,
                    round.player_2.player_name,
                    round.player_2.action.point() + 3,
                    round.player_2.action,
                );
            }
        };
    }

    pub fn action_to_win(open_action: Action) -> Action {
        if open_action == Action::ROCK {
            return Action::PAPER;
        } else if open_action == Action::PAPER {
            return Action::SCISSORS;
        } else {
            return Action::ROCK;
        }
    }

    pub fn action_to_loose(open_action: Action) -> Action {
        if open_action == Action::ROCK {
            return Action::SCISSORS;
        } else if open_action == Action::PAPER {
            return Action::ROCK;
        } else {
            return Action::PAPER;
        }
    }
}
