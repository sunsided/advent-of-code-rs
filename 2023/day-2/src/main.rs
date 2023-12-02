use std::borrow::Borrow;
use std::collections::Bound;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::RangeBounds;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");
const GIVEN: SetOfCubes = SetOfCubes::rgb(12, 13, 14);

fn main() {
    let games: Vec<_> = iter_games(INPUT.lines())
        .map(|g| g.expect("found invalid game"))
        .collect();
    let sum_of_possible_game_ids: u32 = filter_playable_games(games.iter(), &GIVEN)
        .map(|g| g.game_no)
        .sum();
    println!("The sum of all possible game IDs is: {sum_of_possible_game_ids}");

    let power_of_smallest: u32 = games
        .iter()
        .map(|g| g.smallest_set_needed())
        .map(|g| g.power())
        .sum();
    println!("The total power of all smallest sets is: {power_of_smallest}");
}

/// A game.
#[derive(Debug, Eq, PartialEq)]
struct Game {
    /// The number of the game.
    game_no: u32,
    /// The sets of cubes drawn from the bag.
    draws: Vec<SetOfCubes>,
}

/// A number of colored cubes drawn from the bag.
#[derive(Debug, Eq, PartialEq, Default)]
struct SetOfCubes {
    /// The number of red cubes drawn.
    red: u32,
    /// The number of green cubes drawn.
    green: u32,
    /// The number of blue cubes drawn.
    blue: u32,
}

impl Game {
    /// Checks if the given draw is available in the list of cube sets.
    ///
    /// # Arguments
    ///
    /// * `given` - A reference to the draw to be checked.
    ///
    /// # Returns
    ///
    /// * `true` if the given draw is present in the list of draws, `false` otherwise.
    ///
    /// # Notes
    ///
    /// This function uses the `iter` method to iterate over the list of draws and
    /// check if any of the draws are greater than or equal to the given draw. It
    /// returns `true` if at least one draw satisfies this condition, `false` otherwise.
    pub fn is_possible(&self, given: &SetOfCubes) -> bool {
        self.draws.iter().all(|game| {
            game.red <= given.red && game.green <= given.green && game.blue <= given.blue
        })
    }

    /// Returns the smallest `SetOfCubes` needed based on the `draws` contained in the current object.
    ///
    /// The `draws` are iterated over and a fold operation is performed to find the smallest `SetOfCubes`.
    /// The `SetOfCubes` returned will have the maximum values for `red`, `green`, and `blue` from all the `draws`.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::SetOfCubes;
    ///
    /// let game = Game {
    ///     game_no: 0,
    ///     draws: vec![
    ///         SetOfCubes::rgb(1, 2, 9),
    ///         SetOfCubes::rgb(4, 8, 6),
    ///         SetOfCubes::rgb(7, 5, 3),
    ///     ],
    /// };
    ///
    /// let smallest_set = game.smallest_set_needed();
    /// assert_eq!(smallest_set, SetOfCubes::rgb(7, 8, 9));
    /// ```
    ///
    /// # Returns
    ///
    /// The smallest `SetOfCubes` needed based on the `draws` contained in the current object.
    pub fn smallest_set_needed(&self) -> SetOfCubes {
        self.draws
            .iter()
            .fold(SetOfCubes::default(), |smallest, set| {
                SetOfCubes::rgb(
                    smallest.red.max(set.red),
                    smallest.green.max(set.green),
                    smallest.blue.max(set.blue),
                )
            })
    }
}

impl SetOfCubes {
    pub const fn rgb(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    pub const fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Ensure there are no multi-byte characters so we can fiddle with the bytes directly.
        if !s.is_ascii() {
            return Err(ParseGameError("found non-ASCII characters"));
        }

        let s = s.trim_start();
        if &s[..5] != "Game " {
            return Err(ParseGameError("preamble missing"));
        }

        // Parse the game number.
        let game_separator = find_in_range(s, 5.., ':').ok_or(ParseGameError("missing colon"))?;
        let game_no: u32 = s[5..game_separator]
            .parse()
            .map_err(|_e| ParseGameError("invalid game number"))?;

        // Parse the game draws.
        let mut draws = Vec::new();
        let mut section_begin = game_separator + 1;
        while section_begin < s.len() {
            let section_end = find_in_range(s, section_begin.., ';').unwrap_or(s.len());
            let draw_section = s[section_begin..section_end].trim();

            let mut draw = SetOfCubes {
                red: 0,
                green: 0,
                blue: 0,
            };

            // Parse all color counts.
            let mut color_begin = 0;
            while color_begin < draw_section.len() {
                let color_end =
                    find_in_range(draw_section, color_begin.., ',').unwrap_or(draw_section.len());
                let color_section = draw_section[color_begin..color_end].trim();

                let count_end = find_in_range(color_section, 0.., ' ')
                    .ok_or(ParseGameError("invalid draw definition"))?;
                let num_cubes_drawn: u32 = color_section[..count_end]
                    .parse()
                    .map_err(|_e| ParseGameError("invalid draw count definition"))?;

                match &color_section[(count_end + 1)..] {
                    "red" => draw.red += num_cubes_drawn,
                    "green" => draw.green += num_cubes_drawn,
                    "blue" => draw.blue += num_cubes_drawn,
                    _ => return Err(ParseGameError("Invalid color name")),
                }

                color_begin = color_end + 1;
            }

            draws.push(draw);
            section_begin = section_end + 1;
        }

        Ok(Self { game_no, draws })
    }
}

/// Finds the index of the first occurrence of a given `pattern` character in the `input` string.
/// The search is restricted to the given `search_range` bounds, represented by a `RangeBounds<usize>` object.
///
/// # Arguments
///
/// * `input` - The input string to search in.
/// * `search_range` - The range within which to search for the pattern.
/// * `pattern` - The character to search for.
///
/// # Returns
///
/// * If the pattern is found within the search range, returns the index of the first occurrence of the pattern character.
/// * If the search range is empty or the pattern is not found, returns `None`.
///
/// # Examples
///
/// ```
/// use std::ops::Bound;
///
/// let input = "Hello, world!";
/// let search_range = 0..5; // Search only in the first 5 characters
/// let pattern = 'o';
///
/// let result = find_index(input, search_range, pattern);
/// assert_eq!(result, Some(4));
/// ```
fn find_in_range<R: RangeBounds<usize>>(
    input: &str,
    search_range: R,
    pattern: char,
) -> Option<usize> {
    let start = match search_range.start_bound() {
        Bound::Included(x) => *x,
        Bound::Excluded(_) => unreachable!(),
        Bound::Unbounded => 0,
    };
    let end = match search_range.end_bound() {
        Bound::Included(x) => *x,
        Bound::Excluded(x) => *x - 1,
        Bound::Unbounded => input.len() - 1,
    };
    if start >= input.len() {
        return None;
    }
    input[start..=end].find(pattern).map(|idx| idx + start)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct ParseGameError(&'static str);

impl Display for ParseGameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid game definition: {}", self.0)
    }
}

impl Error for ParseGameError {}

/// Iterates over a collection of lines and returns an iterator that yields `Result<Game, ParseGameError>`.
///
/// # Arguments
///
/// * `lines` - An iterator over lines that represents games.
///
/// # Returns
///
/// An iterator that yields `Result<Game, ParseGameError>`, where `Game` is a struct representing a parsed game
/// and `ParseGameError` is an error that occurs during game parsing.
fn iter_games<'a, I: Iterator<Item = &'a str>>(
    lines: I,
) -> impl Iterator<Item = Result<Game, ParseGameError>> {
    lines.map(Game::from_str)
}

/// Filter playable games from an iterator based on a given draw.
///
/// This function takes an iterator of `Result<Game, ParseGameError>` and a reference to a `Draw`,
/// and returns a new iterator that only contains the games that are playable based on the given draw.
///
/// # Arguments
///
/// * `games` - An iterator of `Result<Game, ParseGameError>` representing a collection of games.
/// * `given` - A reference to a `Draw` representing the given draw.
///
/// # Returns
///
/// An iterator of `Result<Game, ParseGameError>` that only contains playable games.
fn filter_playable_games<'a, I, G>(games: I, given: &'a SetOfCubes) -> impl Iterator<Item = G> + 'a
where
    I: Iterator<Item = G> + 'a,
    G: Borrow<Game>,
{
    games.filter(|game| game.borrow().is_possible(given))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(
        input,
        game_no,
        num_draws,
        total_red,
        total_green,
        total_blue,
        case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 1, 3, 4 + 1, 2 + 2, 3 + 6),
        case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red", 2, 2, 1, 2 + 3, 1 + 4),
        case(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            3,
            3,
            20 + 4 + 1,
            8 + 13 + 5,
            6 + 5
        ),
        case(
            "Game 100: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            100,
            3,
            3 + 6 + 14,
            1 + 3 + 3,
            6 + 15
        )
    )]
    fn test_parse_game(
        input: &str,
        game_no: u32,
        num_draws: usize,
        total_red: u32,
        total_green: u32,
        total_blue: u32,
    ) {
        let result = Game::from_str(input);
        assert!(!result.is_err(), "Parsing failed with error {result:?}");

        let game = result.unwrap();
        assert_eq!(
            game.game_no, game_no,
            "Game number is incorrect: Expected {game_no}, got {}",
            game.game_no
        );

        assert_eq!(
            game.draws.len(),
            num_draws,
            "Number of draws is incorrect: Expected {num_draws}, got {}",
            game.draws.len()
        );

        let sum = game
            .draws
            .iter()
            .fold(SetOfCubes::default(), |sum, item| SetOfCubes {
                red: sum.red + item.red,
                green: sum.green + item.green,
                blue: sum.blue + item.blue,
            });

        assert_eq!(
            sum.red, total_red,
            "Number of total red draws is incorrect: Expected {total_red}, got {}",
            sum.red
        );

        assert_eq!(
            sum.green, total_green,
            "Number of total green draws is incorrect: Expected {total_green}, got {}",
            sum.green
        );

        assert_eq!(
            sum.blue, total_blue,
            "Number of total blue draws is incorrect: Expected {total_blue}, got {}",
            sum.blue
        );
    }

    #[rstest(
        input,
        given_red,
        given_green,
        given_blue,
        expected_possibility,
        case(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            12,
            13,
            14,
            true
        ),
        case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red", 12, 13, 14, true),
        case(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            12,
            13,
            14,
            false
        ),
        case(
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            12,
            13,
            14,
            false
        ),
        case(
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            12,
            13,
            14,
            true
        )
    )]
    fn test_is_possible(
        input: &str,
        given_red: u32,
        given_green: u32,
        given_blue: u32,
        expected_possibility: bool,
    ) {
        let result = Game::from_str(input).expect("failed to parse game");
        let given = SetOfCubes::rgb(given_red, given_green, given_blue);
        assert_eq!(result.is_possible(&given), expected_possibility)
    }

    #[test]
    fn test_find_index() {
        assert_eq!(find_in_range("abcdef", 0.., 'c'), Some(2));
        assert_eq!(find_in_range("abcdef", 2.., 'c'), Some(2));
        assert_eq!(find_in_range("abcdef", 3.., 'c'), None);
    }

    #[test]
    fn test_filter_playable() {
        const EXAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
             Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
             Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
             Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
             Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        const GIVEN: SetOfCubes = SetOfCubes::rgb(12, 13, 14);

        let games = iter_games(EXAMPLE.lines()).map(|g| g.expect("found invalid game"));
        let possible_games: Vec<_> = filter_playable_games(games, &GIVEN).collect();
        assert_eq!(possible_games.len(), 3);
        assert!(possible_games.iter().any(|g| g.game_no == 1));
        assert!(possible_games.iter().any(|g| g.game_no == 2));
        assert!(possible_games.iter().any(|g| g.game_no == 5));
    }

    #[test]
    fn test_smallest_needed() {
        let game = Game {
            game_no: 0,
            draws: vec![
                SetOfCubes::rgb(1, 2, 9),
                SetOfCubes::rgb(4, 8, 6),
                SetOfCubes::rgb(7, 5, 3),
            ],
        };

        let smallest_set = game.smallest_set_needed();
        assert_eq!(smallest_set, SetOfCubes::rgb(7, 8, 9));
        assert_eq!(smallest_set.power(), 7 * 8 * 9);
    }

    #[test]
    fn test_power_of_smallest() {
        const EXAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
             Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
             Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
             Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
             Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        const GIVEN: SetOfCubes = SetOfCubes::rgb(12, 13, 14);

        let games = iter_games(EXAMPLE.lines());
        let power_of_smallest: u32 = games
            .map(|g| g.expect("found invalid game"))
            .map(|g| g.smallest_set_needed())
            .map(|g| g.power())
            .sum();
        assert_eq!(power_of_smallest, 2286);
    }
}
