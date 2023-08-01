use std::{collections::HashMap, fs};

use crate::solutions::day_2::rock_paper_scissors::{Action, Game, GamePlay, Result, Round};
use crate::solutions::traits::Solution;

pub struct Day2<'a> {
    data: Vec<Vec<&'a str>>,
}

impl<'a> Day2<'a> {
    pub fn new(data: Vec<Vec<&'a str>>) -> Day2<'a> {
        return Day2 { data };
    }

    fn part_1(&mut self) -> i32 {
        let mut game = Game::new("one", "two");

        for (index, group) in self.data.clone().into_iter().enumerate() {
            let round = group.into_iter().collect::<Vec<&str>>();

            let player_1_game_play = GamePlay {
                player_name: game.player_1.to_string(),
                action: Day2::convert_string_to_action(round[0]),
            };

            let player_2_game_play = GamePlay {
                player_name: game.player_2.to_string(),
                action: Day2::convert_string_to_action(round[1]),
            };

            game.new_round(Round {
                round: index as i32,
                player_1: player_1_game_play,
                player_2: player_2_game_play,
            });
        }

        return Day2::get_total_score(&game);
    }

    fn part_2(&mut self) -> i32 {
        let mut game = Game::new("one", "two");

        for (index, item) in self.data.clone().into_iter().enumerate() {
            let player_1_action = Day2::convert_string_to_action(item[0]);

            if item[1] == "X" {
                // LOSE round
                let action_to_loose: Action = Game::action_to_loose(player_1_action.clone());

                let player_1_game_play = GamePlay {
                    player_name: game.player_1.to_string(),
                    action: player_1_action.clone(),
                };

                let player_2_game_play = GamePlay {
                    player_name: game.player_2.to_string(),
                    action: action_to_loose,
                };

                game.new_round(Round {
                    round: index as i32,
                    player_1: player_1_game_play,
                    player_2: player_2_game_play,
                });
            } else if item[1] == "Y" {
                // DRAW round
                let action_to_draw: Action = player_1_action.clone();

                let player_1_game_play = GamePlay {
                    player_name: game.player_1.to_string(),
                    action: player_1_action.clone(),
                };

                let player_2_game_play = GamePlay {
                    player_name: game.player_2.to_string(),
                    action: action_to_draw,
                };

                game.new_round(Round {
                    round: index as i32,
                    player_1: player_1_game_play,
                    player_2: player_2_game_play,
                })
            } else {
                // WIN round
                let action_to_win: Action = Game::action_to_win(player_1_action.clone());

                let player_1_game_play = GamePlay {
                    player_name: game.player_1.to_string(),
                    action: player_1_action.clone(),
                };

                let player_2_game_play = GamePlay {
                    player_name: game.player_2.to_string(),
                    action: action_to_win,
                };

                game.new_round(Round {
                    round: index as i32,
                    player_1: player_1_game_play,
                    player_2: player_2_game_play,
                });
            }
        }

        return Day2::get_total_score(&game);
    }

    fn convert_string_to_action(string_action: &str) -> Action {
        if string_action == "A" || string_action == "X" {
            return Action::ROCK;
        } else if string_action == "B" || string_action == "Y" {
            return Action::PAPER;
        } else {
            return Action::SCISSORS;
        }
    }

    fn get_total_score(game: &Game) -> i32 {
        let result_map: &HashMap<i32, Vec<Result>> = &game.results;
        let mut total_score: i32 = 0;

        for (_key, &ref value) in result_map {
            if value[0].player_name == game.player_2.to_string() {
                total_score += value[0].score;
            } else if value[1].player_name == game.player_2.to_string() {
                total_score += value[1].score;
            }
        }

        return total_score;
    }
}

impl<'a> Solution for Day2<'a> {
    type PartOne = u32;
    type PartTwo = u32;

    fn solution() -> (Self::PartOne, Self::PartTwo) {
        let contents = fs::read_to_string("src/inputs/day_2_input.txt").unwrap();

        let groups = contents.split("\n").map(|group| group.split(" "));

        let data = groups
            .map(|group| group.into_iter().collect::<Vec<&str>>())
            .into_iter()
            .collect::<Vec<Vec<&str>>>();

        let mut day_2 = Day2::new(data);

        let part_1_soln = day_2.part_1() as u32;
        let part_2_soln = day_2.part_2() as u32;

        (part_1_soln, part_2_soln)
    }
}
