use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
struct NodeId([char; 3], u16);

#[derive(Debug, Copy, Clone)]
struct Node {
    id: NodeId,
    left: NodeId,
    right: NodeId,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Directions(Vec<Direction>);

pub fn count_steps_to_destination(input: &str) -> usize {
    let (directions, nodes) = parse_input(input);
    count_until(&directions, &nodes, NodeId::START, NodeId::GOAL, 0)
}

pub fn count_ghost_steps_to_destination(input: &str) -> usize {
    let (directions, nodes) = parse_input(input);

    let node_ids: Vec<_> = nodes
        .keys()
        .filter(|id| id.is_ghost_start())
        .copied()
        .collect();

    let loop_lengths: Vec<usize> = node_ids
        .iter()
        .map(|&id| count_until_ghost_goal(&directions, &nodes, id))
        .collect();

    lcm_slice(&loop_lengths)
}

fn count_until(
    directions: &Directions,
    nodes: &HashMap<NodeId, Node>,
    mut node_id: NodeId,
    goal: NodeId,
    min_steps: usize,
) -> usize {
    for (steps_taken, direction) in directions.iter().enumerate() {
        if node_id == goal && steps_taken >= min_steps {
            return steps_taken;
        }

        node_id = nodes[&node_id].branch(direction);
    }

    unreachable!();
}

fn count_until_ghost_goal(
    directions: &Directions,
    nodes: &HashMap<NodeId, Node>,
    mut node_id: NodeId,
) -> usize {
    for (steps_taken, direction) in directions.iter().enumerate() {
        if node_id.is_ghost_goal() {
            return steps_taken;
        }

        node_id = nodes[&node_id].branch(direction);
    }

    unreachable!();
}

/// Calculate the greatest common divisor (GCD) of two numbers.
///
/// The GCD is the largest positive integer that divides both `a` and `b` without remainder.
/// This function uses the Euclidean algorithm to calculate the GCD.
///
/// # Arguments
///
/// * `a` - The first number.
/// * `b` - The second number.
///
/// # Returns
///
/// The GCD of `a` and `b`.
///
/// # Examples
///
/// ```
/// use aoc_2023_day_8::gcd;
///
/// let result = gcd(10, 15);
/// assert_eq!(result, 5);
///
/// let result = gcd(24, 36);
/// assert_eq!(result, 12);
/// ```
///
pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Calculates the least common multiple (LCM) of two numbers.
///
/// # Arguments
///
/// * `a` - A positive integer number.
/// * `b` - Another positive integer number.
///
/// # Returns
///
/// The LCM of `a` and `b`.
///
/// # Examples
///
/// ```
/// use aoc_2023_day_8::lcm;
///
/// let result = lcm(12, 18);
/// assert_eq!(result, 36);
/// ```
pub fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

/// Calculates the least common multiple (LCM) of a vector of numbers.
///
/// The function takes a vector of `usize` numbers and returns their
/// least common multiple (LCM).
///
/// # Arguments
///
/// * `numbers` - A vector of `usize` numbers.
///
/// # Returns
///
/// The LCM of the given numbers.
///
/// # Panics
///
/// The function will panic if called with an empty vector.
///
/// # Examples
///
/// ```
/// use std::iter::FromIterator;
/// use aoc_2023_day_8::lcm_slice;
///
/// let numbers = Vec::from_iter([2, 3, 4, 5]);
/// let lcm = lcm_slice(&numbers);
/// assert_eq!(lcm, 60);
/// ```
pub fn lcm_slice(numbers: &[usize]) -> usize {
    let mut iter = numbers.iter();
    let &first = iter.next().unwrap();
    iter.fold(first, |a, &b| lcm(a, b))
}

fn parse_input(input: &str) -> (Directions, HashMap<NodeId, Node>) {
    let mut lines = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());

    let directions: Directions = lines
        .next()
        .expect("empty input")
        .parse()
        .expect("failed to parse directions");

    let nodes: Vec<Node> = lines
        .map(Node::from_str)
        .collect::<Result<_, _>>()
        .expect("failed to parse nodes");

    let map: HashMap<NodeId, Node> =
        HashMap::from_iter(nodes.into_iter().map(|node| (node.id, node)));

    (directions, map)
}

impl Node {
    pub fn branch(&self, direction: Direction) -> NodeId {
        match direction {
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
}

impl Directions {
    pub fn iter(&self) -> impl Iterator<Item = Direction> + '_ {
        self.0.iter().copied().cycle()
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl NodeId {
    /// Marks a start node according to part 1.
    pub const START: NodeId = NodeId(['A', 'A', 'A'], 0);

    /// Marks a goal node according to part 1.
    pub const GOAL: NodeId = NodeId(['Z', 'Z', 'Z'], 25 * 100 + 25 * 10 + 25);

    pub fn new(first: char, second: char, third: char) -> Self {
        let hash = (first as usize - 'A' as usize) * 100
            + (second as usize - 'A' as usize) * 10
            + (third as usize - 'A' as usize);
        let hash = hash as u16;
        Self([first, second, third], hash)
    }

    /// Identifies a start node according to part 2.
    pub fn is_ghost_start(&self) -> bool {
        self.0[2] == 'A'
    }

    /// Identifies a goal node according to part 2.
    pub fn is_ghost_goal(&self) -> bool {
        self.0[2] == 'Z'
    }
}

impl Hash for NodeId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.1.hash(state)
    }
}

impl FromStr for Node {
    type Err = ParseNodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.len() != 16 {
            return Err(ParseNodeError("Invalid length"));
        }

        let id = NodeId::from_str(&s[..3]).map_err(|_| ParseNodeError("Invalid node ID"))?;
        let left = NodeId::from_str(&s[7..10]).map_err(|_| ParseNodeError("Invalid node ID"))?;
        let right = NodeId::from_str(&s[12..15]).map_err(|_| ParseNodeError("Invalid node ID"))?;

        Ok(Self { id, left, right })
    }
}

impl FromStr for NodeId {
    type Err = ParseNodeIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.len() != 3 {
            return Err(ParseNodeIdError("Invalid length"));
        }

        let mut chars = s.chars();
        Ok(Self::new(
            chars.next().expect("invalid iterator"),
            chars.next().expect("invalid iterator"),
            chars.next().expect("invalid iterator"),
        ))
    }
}

impl FromStr for Directions {
    type Err = ParseDirectionsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err(ParseDirectionsError("Empty input"));
        }

        let directions: Vec<_> = s
            .chars()
            .map(|c| match c {
                'L' => Ok(Direction::Left),
                'R' => Ok(Direction::Right),
                _ => Err(ParseDirectionsError("Invalid input in sequence")),
            })
            .collect::<Result<_, _>>()?;

        Ok(Self(directions))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ParseDirectionsError(&'static str);

impl Display for ParseDirectionsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse directions: {}", self.0)
    }
}

impl Error for ParseDirectionsError {}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ParseNodeError(&'static str);

impl Display for ParseNodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse node: {}", self.0)
    }
}

impl Error for ParseNodeError {}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ParseNodeIdError(&'static str);

impl Display for ParseNodeIdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse Node ID: {}", self.0)
    }
}

impl Error for ParseNodeIdError {}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_parse_node_id() {
        let id: NodeId = "ABC".parse().expect("failed to parse node ID");
        assert_eq!(id, NodeId::new('A', 'B', 'C'))
    }

    #[test]
    fn test_parse_node() {
        let node: Node = "AAA = (BBB, CCC)".parse().expect("failed to parse node ID");
        assert_eq!(node.id, NodeId::new('A', 'A', 'A'));
        assert_eq!(node.left, NodeId::new('B', 'B', 'B'));
        assert_eq!(node.right, NodeId::new('C', 'C', 'C'));
    }

    #[test]
    fn test_parse_directions() {
        let directions: Directions = "LLR".parse().expect("failed to parse directions");
        assert_eq!(
            directions.0,
            [Direction::Left, Direction::Left, Direction::Right]
        );
    }

    #[test]
    fn test_directions_iter() {
        let directions: Directions = "LLR".parse().expect("failed to parse directions");

        let mut directions = directions.iter();
        assert_eq!(directions.next(), Some(Direction::Left));
        assert_eq!(directions.next(), Some(Direction::Left));
        assert_eq!(directions.next(), Some(Direction::Right));

        // The iterator cycles.
        assert_eq!(directions.next(), Some(Direction::Left));
        assert_eq!(directions.next(), Some(Direction::Left));
        assert_eq!(directions.next(), Some(Direction::Right));
    }

    #[test]
    fn test_parse_input() {
        const INPUT: &str = "LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)";

        let (directions, nodes) = parse_input(INPUT);
        assert_eq!(directions.0.len(), 3);
        assert_eq!(nodes.len(), 3);
    }

    #[test]
    fn test_part_1() {
        const INPUT: &str = "RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
            ";

        assert_eq!(count_steps_to_destination(INPUT), 2);
    }

    #[test]
    fn test_part_2() {
        const INPUT: &str = "LR

            FFA = (FFB, XXX)
            FFB = (XXX, FFZ)
            FFZ = (FFB, XXX)
            GGA = (GGB, XXX)
            GGB = (GGC, GGC)
            GGC = (GGZ, GGZ)
            GGZ = (GGB, GGB)
            XXX = (XXX, XXX)";

        assert_eq!(count_ghost_steps_to_destination(INPUT), 6);
    }

    #[test]
    fn test_loop_from_start() {
        let (directions, nodes) = parse_input(INPUT);

        // Find all start nodes.
        let mut node_ids: Vec<_> = nodes
            .keys()
            .filter(|id| id.is_ghost_start())
            .copied()
            .collect();
        node_ids.sort();
        assert_eq!(node_ids.len(), 6);

        // Determine the length of a cycle from a goal node to its next occurrence.
        let cycle_lengths: Vec<usize> = node_ids
            .iter()
            .map(|&id| count_until_ghost_goal(&directions, &nodes, id))
            .collect();

        assert_eq!(cycle_lengths[0], 22199); // AAA -> ZZZ
        assert_eq!(cycle_lengths[1], 13207); // DVA -> XDZ
        assert_eq!(cycle_lengths[2], 18827); // JHA -> FRZ
        assert_eq!(cycle_lengths[3], 17141); // NMA -> JFZ
        assert_eq!(cycle_lengths[4], 14893); // PXA -> LHZ
        assert_eq!(cycle_lengths[5], 16579); // VXA -> TNZ

        // Path lengths are evenly divisible by the direction length.
        let direction_length = directions.len();
        assert_eq!(cycle_lengths[0] % direction_length, 0);
        assert_eq!(cycle_lengths[1] % direction_length, 0);
        assert_eq!(cycle_lengths[2] % direction_length, 0);
        assert_eq!(cycle_lengths[3] % direction_length, 0);
        assert_eq!(cycle_lengths[4] % direction_length, 0);
        assert_eq!(cycle_lengths[5] % direction_length, 0);

        assert_eq!(lcm_slice(&cycle_lengths), 13334102464297);
    }

    #[test]
    fn test_loop_from_goal() {
        let (directions, nodes) = parse_input(INPUT);

        // Find all goal nodes.
        let mut node_ids: Vec<_> = nodes
            .keys()
            .filter(|id| id.is_ghost_goal())
            .copied()
            .collect();
        node_ids.sort();
        assert_eq!(node_ids.len(), 6);

        // Determine the length of a cycle from a goal node to its next occurrence.
        let cycle_lengths: Vec<usize> = node_ids
            .iter()
            .map(|&id| count_until(&directions, &nodes, id, id, 1))
            .collect();

        assert_eq!(cycle_lengths[0], 18827); // FRZ -> FRZ
        assert_eq!(cycle_lengths[1], 17141); // JFZ -> JFZ
        assert_eq!(cycle_lengths[2], 14893); // LHZ -> LHZ
        assert_eq!(cycle_lengths[3], 16579); // TNZ -> TNZ
        assert_eq!(cycle_lengths[4], 13207); // XDZ -> XDZ
        assert_eq!(cycle_lengths[5], 22199); // ZZZ -> ZZZ

        assert_eq!(lcm_slice(&cycle_lengths), 13334102464297);
    }
}
