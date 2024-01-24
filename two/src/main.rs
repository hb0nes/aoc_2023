use aoclib::parse_input_lines;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    combinator::map_res,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use std::{
    fs::File,
    io::{BufReader, Read},
};

const GAME_PARAMS: GameColors = GameColors {
    red: 12,
    green: 13,
    blue: 14,
};

const COLOR_RED: &str = "red";
const COLOR_GREEN: &str = "green";
const COLOR_BLUE: &str = "blue";

#[derive(Debug)]
struct GameLine {
    game_id: usize,
    game_colors: Vec<GameColors>,
}

#[derive(Debug)]
struct GameColors {
    red: usize,
    green: usize,
    blue: usize,
}

impl Default for GameColors {
    fn default() -> Self {
        GameColors {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

fn solution_one(input: &str) -> anyhow::Result<usize> {
    let (_, games) = parse_input_lines(input, parse_gameline).unwrap();
    let mut game_id_sum = 0;
    for game in games {
        if check_game(&game) {
            game_id_sum += game.game_id
        }
    }
    Ok(game_id_sum)
}

fn solution_two(input: &str) -> anyhow::Result<usize> {
    let (_, games) = parse_input_lines(input, parse_gameline).unwrap();
    let mut power_sum = 0;
    for game in games.iter() {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for game_color in game.game_colors.iter() {
            if game_color.red > max_red {
                max_red = game_color.red
            }
            if game_color.green > max_green {
                max_green = game_color.green
            }
            if game_color.blue > max_blue {
                max_blue = game_color.blue
            }
        }
        power_sum += max_red * max_green * max_blue;
    }
    Ok(power_sum)
}

fn check_game(game: &GameLine) -> bool {
    for game_colors in game.game_colors.iter() {
        if game_colors.red > GAME_PARAMS.red
            || game_colors.green > GAME_PARAMS.green
            || game_colors.blue > GAME_PARAMS.blue
        {
            return false;
        }
    }
    true
}

fn parse_gameline(input: &str) -> IResult<&str, GameLine> {
    let (remaining, (game_num, game_colors)) = tuple((game_num, game_colors_all))(input)?;
    Ok((
        remaining,
        GameLine {
            game_id: game_num,
            game_colors,
        },
    ))
}

fn game_num(input: &str) -> IResult<&str, usize> {
    map_res(
        tuple((tag("Game"), multispace1, digit1, tag(": "))),
        |(_, _, num_str, _): (_, _, &str, _)| num_str.parse::<usize>(),
    )(input)
}

fn game_colors_all(input: &str) -> IResult<&str, Vec<GameColors>> {
    let (remainder, game_colors_all) =
        separated_list1(tag("; "), game_colors_single)(input).unwrap();
    Ok((remainder, game_colors_all))
}

fn game_colors_single(input: &str) -> IResult<&str, GameColors> {
    let (remainder, color_counts) = separated_list1(tag(", "), color_count)(input).unwrap();
    let mut game_colors = GameColors::default();
    color_counts.iter().for_each(|(count, color)| match color {
        &COLOR_RED => game_colors.red = *count,
        &COLOR_GREEN => game_colors.green = *count,
        &COLOR_BLUE => game_colors.blue = *count,
        _ => (),
    });
    Ok((remainder, game_colors))
}

fn color_count(input: &str) -> IResult<&str, (usize, &str)> {
    map_res(
        tuple((digit1, multispace1, color)),
        |(digit_str, _, color_str)| digit_str.parse::<usize>().map(|digit| (digit, color_str)),
    )(input)
}

fn color(input: &str) -> IResult<&str, &str> {
    alt((tag("red"), tag("green"), tag("blue")))(input)
}

fn main() -> anyhow::Result<()> {
    let mut buf = String::new();
    BufReader::new(File::open("input.txt").unwrap())
        .read_to_string(&mut buf)
        .unwrap();
    let one = solution_one(&buf).unwrap();
    let two = solution_two(&buf).unwrap();
    println!("one: {}", one);
    println!("two: {}", two);
    Ok(())
}
