use crate::common;

#[derive(Debug)]
struct BagConfiguration {
    red: i32,
    green: i32,
    blue: i32
}

impl BagConfiguration {
    fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

const BAG_CONFIG: BagConfiguration = BagConfiguration {
    red: 12,
    green: 13,
    blue: 14,
};

fn get_color(color: &str, bag_configuration: &BagConfiguration) -> i32 {
    match color {
        "red" => bag_configuration.red,
        "green" => bag_configuration.green,
        "blue" => bag_configuration.blue,
        _ => panic!("failed match of {} by color!", color)
    }
}

fn set_color(color: &str, bag_configuration: &mut BagConfiguration, n: i32) -> () {
    match color {
        "red" => { bag_configuration.red = n; },
        "green" => { bag_configuration.green = n; },
        "blue" => { bag_configuration.blue = n; },
        _ => panic!("failed match of {} by color!", color)
    }
}

pub(crate) fn solve() {
    if let Ok(lines) = common::read_lines("src/day2/input.txt") {
        let mut game_n_sum = 0;
        let mut min_power_sum = 0;

        for line in lines {
            if let Ok(content) = line {
                let mut min_bag_config = BagConfiguration {
                    red: 0,
                    green: 0,
                    blue: 0,
                };

                let mut game_reveal_sets = content
                    .split(&[':', ';'][..])
                    .map(|s| s.trim());

                let game_n = game_reveal_sets.next()
                    .expect("game number information should exist!")
                    .split_whitespace()
                    .last()
                    .map(|s| s.parse::<i32>())
                    .expect("game number information should have last character!")
                    .expect("game number information should end in a number!");

                let mut is_possible_game = true;
                for cube_sets in game_reveal_sets {
                    for cube_set in cube_sets.split(", ") {
                        let mut iter = cube_set.split_whitespace();
                        let n_cubes = iter.next()
                            .map(|n| n.parse::<i32>())
                            .expect("cube set should have n cubes count!")
                            .expect("cube set n cubes count should be a number!");
                        let cube_color = iter.next()
                            .expect("cube set should have cube color!");

                        let min_cubes = get_color(cube_color, &min_bag_config);
                        if n_cubes > min_cubes {
                            set_color(cube_color, &mut min_bag_config, n_cubes)
                        }

                        let max_cubes= get_color(cube_color, &BAG_CONFIG);
                        if n_cubes > max_cubes && is_possible_game {
                            is_possible_game = false;
                        }
                    }
                }

                if is_possible_game {
                    game_n_sum += game_n;
                }

                min_power_sum += min_bag_config.power();
            }
        }

        println!("2.1: {}", game_n_sum);
        println!("2.2: {}", min_power_sum)
    }
}