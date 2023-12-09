use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
struct NodeId([char; 3]);

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

    let mut node_id = NodeId::START;
    for (steps_taken, direction) in directions.iter().enumerate() {
        if node_id == NodeId::GOAL {
            return steps_taken;
        }

        node_id = nodes[&node_id].branch(direction);
    }

    unreachable!()
}

pub fn count_ghost_steps_to_destination(input: &str) -> usize {
    let (directions, nodes) = parse_input(input);

    let mut node_ids: Vec<_> = nodes
        .keys()
        .filter(|id| id.is_ghost_start())
        .copied()
        .collect();

    // TODO: Use (t)racers: Move the first one as far as possible, count the max distance.
    //       Then move the next one until it reaches that distance. If it overshoots, set as max distance.
    //       Repeat with the next one until all have reached the max distance.

    // TODO: Instead of a HashMap convert NodeId to index. "ZZZ" becomes 26 * 100 + 26 * 10 + 26 = 2886.

    for (steps_taken, direction) in directions.iter().enumerate() {
        // Step simultaneously.
        let mut goal_reached = true;
        for node_id in node_ids.iter_mut() {
            *node_id = nodes[node_id].branch(direction);
            goal_reached &= node_id.is_ghost_goal();
        }

        // If all simultaneous steps reached a goal node, finish.
        if goal_reached {
            return steps_taken + 1;
        }
    }

    unreachable!()
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
}

impl NodeId {
    /// Marks a start node according to part 1.
    pub const START: NodeId = NodeId(['A', 'A', 'A']);

    /// Marks a goal node according to part 1.
    pub const GOAL: NodeId = NodeId(['Z', 'Z', 'Z']);

    /// Identifies a start node according to part 2.
    pub fn is_ghost_start(&self) -> bool {
        self.0[2] == 'A'
    }

    /// Identifies a goal node according to part 2.
    pub fn is_ghost_goal(&self) -> bool {
        self.0[2] == 'Z'
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
        Ok(Self([
            chars.next().expect("invalid iterator"),
            chars.next().expect("invalid iterator"),
            chars.next().expect("invalid iterator"),
        ]))
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

    #[test]
    fn test_parse_node_id() {
        let id: NodeId = "ABC".parse().expect("failed to parse node ID");
        assert_eq!(id, NodeId(['A', 'B', 'C']))
    }

    #[test]
    fn test_parse_node() {
        let node: Node = "AAA = (BBB, CCC)".parse().expect("failed to parse node ID");
        assert_eq!(node.id, NodeId(['A', 'A', 'A']));
        assert_eq!(node.left, NodeId(['B', 'B', 'B']));
        assert_eq!(node.right, NodeId(['C', 'C', 'C']));
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

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)";

        assert_eq!(count_ghost_steps_to_destination(INPUT), 6);
    }
}
