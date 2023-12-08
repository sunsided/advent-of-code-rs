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
    pub const START: NodeId = NodeId(['A', 'A', 'A']);
    pub const GOAL: NodeId = NodeId(['Z', 'Z', 'Z']);
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
}
