use std::borrow::Borrow;

/// Solution for part 1.
pub fn part1(input: &str) -> u64 {
    let map = parse_tiles(input);

    // The start lies on a tile. We assume the surrounding tiles connect to it meaningfully
    // (i.e. the are no ambiguities). We can allow this assumption because we know the
    // starting position is on a loop, and therefore cannot branch into a dead end.
    let start = map.find_start();
    let tile = map.infer_tile(&start);

    // Get the starting directions.
    let (mut first, mut second) = tile.expand(start);
    let mut previous_first = start;
    let mut previous_second = start;

    // Loop around until we meet again ...
    let mut num_steps = 1;
    while first != second {
        // Move the first pointer.
        let next = map.at(first).step(first, previous_first);
        (first, previous_first) = (next, first);

        // Move the second pointer.
        let next = map.at(second).step(second, previous_second);
        (second, previous_second) = (next, second);

        num_steps += 1;
    }

    num_steps
}

/// Solution for part 2.
pub fn part2(_input: &str) -> u64 {
    todo!()
}

/// A 2D coordinate of x an y.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Coordinate(usize, usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    None,
    Start,
    NorthSouth,
    WestEast,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

struct Map {
    tiles: Vec<Tile>,
    width: usize,
}

fn parse_tiles(input: &str) -> Map {
    let mut tiles = Vec::with_capacity(input.len());
    let mut num_lines = 0;
    for line in input
        .lines()
        .map(|line| line.trim())
        .filter(|&line| !line.is_empty())
    {
        tiles.extend(line.chars().map(Tile::from));
        num_lines += 1;
    }

    // Ensure proper line format.
    let width = tiles.len() / num_lines;
    assert_eq!(width * num_lines, tiles.len());

    Map { tiles, width }
}

impl Map {
    fn find_start(&self) -> Coordinate {
        let pos = self
            .tiles
            .iter()
            .position(|&tile| tile == Tile::Start)
            .expect("map contains no starting position");
        Coordinate(pos % self.width, pos / self.width)
    }

    fn at(&self, position: Coordinate) -> Tile {
        self.tiles[position.x() + position.y() * self.width]
    }

    fn infer_tile(&self, position: &Coordinate) -> Tile {
        if position.has_north() && self.at(position.north()).connects_south() {
            if self.at(position.south()).connects_north() {
                return Tile::NorthSouth;
            }

            if position.has_west() && self.at(position.west()).connects_east() {
                return Tile::NorthWest;
            }

            if self.at(position.east()).connects_west() {
                return Tile::NorthEast;
            }
        }

        if self.at(position.south()).connects_north() {
            if position.has_west() && self.at(position.west()).connects_east() {
                return Tile::SouthWest;
            }

            if self.at(position.east()).connects_west() {
                return Tile::SouthEast;
            }
        }

        if position.has_west()
            && self.at(position.west()).connects_east()
            && self.at(position.east()).connects_west()
        {
            return Tile::WestEast;
        }

        panic!("Unexpected combination of tiles")
    }
}

impl Coordinate {
    pub fn x(&self) -> usize {
        self.0
    }

    pub fn y(&self) -> usize {
        self.1
    }

    pub fn has_north(&self) -> bool {
        self.1 > 0
    }

    pub fn has_west(&self) -> bool {
        self.0 > 0
    }

    pub fn is_north_of(&self, other: &Coordinate) -> bool {
        self.1 < other.1
    }

    pub fn is_south_of(&self, other: &Coordinate) -> bool {
        self.1 > other.1
    }

    pub fn is_west_of(&self, other: &Coordinate) -> bool {
        self.0 < other.0
    }

    pub fn is_east_of(&self, other: &Coordinate) -> bool {
        self.0 > other.0
    }

    pub fn north(&self) -> Coordinate {
        Coordinate(self.0, self.1 - 1)
    }

    pub fn south(&self) -> Coordinate {
        Coordinate(self.0, self.1 + 1)
    }

    pub fn west(&self) -> Coordinate {
        Coordinate(self.0 - 1, self.1)
    }

    pub fn east(&self) -> Coordinate {
        Coordinate(self.0 + 1, self.1)
    }
}

impl Tile {
    pub fn expand<C: Borrow<Coordinate>>(&self, coordinate: C) -> (Coordinate, Coordinate) {
        let coordinate = coordinate.borrow();
        match self {
            Tile::None => panic!("Invalid call on a none-tile"),
            Tile::Start => panic!("invalid call on a start tile"),
            Tile::NorthSouth => (coordinate.north(), coordinate.south()),
            Tile::WestEast => (coordinate.west(), coordinate.east()),
            Tile::NorthEast => (coordinate.north(), coordinate.east()),
            Tile::NorthWest => (coordinate.north(), coordinate.west()),
            Tile::SouthWest => (coordinate.south(), coordinate.west()),
            Tile::SouthEast => (coordinate.south(), coordinate.east()),
        }
    }

    pub fn connects_north(&self) -> bool {
        match self {
            Tile::None => false,
            Tile::Start => panic!("invalid call on a start tile"),
            Tile::NorthSouth => true,
            Tile::WestEast => false,
            Tile::NorthEast => true,
            Tile::NorthWest => true,
            Tile::SouthWest => false,
            Tile::SouthEast => false,
        }
    }

    pub fn connects_south(&self) -> bool {
        match self {
            Tile::None => false,
            Tile::Start => panic!("invalid call on a start tile"),
            Tile::NorthSouth => true,
            Tile::WestEast => false,
            Tile::NorthEast => false,
            Tile::NorthWest => false,
            Tile::SouthWest => true,
            Tile::SouthEast => true,
        }
    }

    pub fn connects_east(&self) -> bool {
        match self {
            Tile::None => false,
            Tile::Start => panic!("invalid call on a start tile"),
            Tile::NorthSouth => false,
            Tile::WestEast => true,
            Tile::NorthEast => true,
            Tile::NorthWest => false,
            Tile::SouthWest => false,
            Tile::SouthEast => true,
        }
    }

    pub fn connects_west(&self) -> bool {
        match self {
            Tile::None => false,
            Tile::Start => panic!("invalid call on a start tile"),
            Tile::NorthSouth => false,
            Tile::WestEast => true,
            Tile::NorthEast => false,
            Tile::NorthWest => true,
            Tile::SouthWest => true,
            Tile::SouthEast => false,
        }
    }

    pub fn step<C: Borrow<Coordinate>, P: Borrow<Coordinate>>(
        &self,
        current: C,
        previous: P,
    ) -> Coordinate {
        let current = current.borrow();
        let previous = previous.borrow();

        match self {
            Tile::None => panic!("can't call step on a none-tile"),
            Tile::Start => panic!("can't call step on the start node"),
            Tile::NorthSouth => {
                debug_assert!(previous.is_north_of(current) || previous.is_south_of(current));
                if previous.is_south_of(current) {
                    // if we came from the south, move further north
                    current.north()
                } else {
                    // if we came from the north, move further south
                    current.south()
                }
            }
            Tile::WestEast => {
                debug_assert!(previous.is_east_of(current) || previous.is_west_of(current));
                if previous.is_east_of(current) {
                    // if we came from the east, move further west
                    current.west()
                } else {
                    // if we came from the west, move further east
                    current.east()
                }
            }
            Tile::NorthEast => {
                debug_assert!(previous.is_east_of(current) || previous.is_north_of(current));
                if previous.is_east_of(current) {
                    // if we came from the east, move north
                    current.north()
                } else {
                    // if we came from the north, move east
                    current.east()
                }
            }
            Tile::NorthWest => {
                debug_assert!(previous.is_west_of(current) || previous.is_north_of(current));
                if previous.is_west_of(current) {
                    // if we came from the west, move north
                    current.north()
                } else {
                    // if we came from the north, move west
                    current.west()
                }
            }
            Tile::SouthWest => {
                debug_assert!(previous.is_west_of(current) || previous.is_south_of(current));
                if previous.is_west_of(current) {
                    // if we came from the west, move south
                    current.south()
                } else {
                    // if we came from the south, move west
                    current.west()
                }
            }
            Tile::SouthEast => {
                debug_assert!(previous.is_east_of(current) || previous.is_south_of(current));
                if previous.is_east_of(current) {
                    // if we came from the east, move south
                    current.south()
                } else {
                    // if we came from the south, move east
                    current.east()
                }
            }
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::NorthSouth,
            '-' => Self::WestEast,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            'S' => Self::Start,
            '.' => Self::None,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        const TEST: &str = ".....
            .S-7.
            .|.|.
            .L-J.
            .....";
        assert_eq!(part1(TEST), 4);
    }

    #[test]
    fn test_part1_example2() {
        const TEST: &str = "..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...";
        assert_eq!(part1(TEST), 8);
    }

    #[test]
    fn test_part2() {
        // const TEST: &str = "...";

        todo!()
        // assert_eq!(part2(TEST), todo!());
    }

    #[test]
    fn test_parse_map() {
        const TEST1: &str = ".....
            .S-7.
            .|.|.
            .L-J.
            .....";
        let map = parse_tiles(TEST1);
        assert_eq!(map.find_start(), Coordinate(1, 1));

        const TEST2: &str = "..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...";
        let map = parse_tiles(TEST2);
        assert_eq!(map.find_start(), Coordinate(0, 2));
    }

    #[test]
    fn test_infer_tile() {
        const TEST1: &str = ".....
            .S-7.
            .|.|.
            .L-J.
            .....";
        let map = parse_tiles(TEST1);
        let start = map.find_start();
        assert_eq!(map.infer_tile(&start), Tile::SouthEast);

        const TEST2: &str = "..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...";
        let map = parse_tiles(TEST2);
        let start = map.find_start();
        assert_eq!(map.infer_tile(&start), Tile::SouthEast);
    }

    #[test]
    fn test_steps() {
        let current = Coordinate(10, 10);
        assert_eq!(
            Tile::NorthSouth.step(current, current.north()),
            current.south()
        );
        assert_eq!(
            Tile::NorthSouth.step(current, current.south()),
            current.north()
        );

        assert_eq!(Tile::WestEast.step(current, current.west()), current.east());
        assert_eq!(Tile::WestEast.step(current, current.east()), current.west());

        assert_eq!(
            Tile::NorthWest.step(current, current.west()),
            current.north()
        );
        assert_eq!(
            Tile::NorthWest.step(current, current.north()),
            current.west()
        );

        assert_eq!(
            Tile::NorthEast.step(current, current.north()),
            current.east()
        );
        assert_eq!(
            Tile::NorthEast.step(current, current.east()),
            current.north()
        );

        assert_eq!(
            Tile::SouthWest.step(current, current.west()),
            current.south()
        );
        assert_eq!(
            Tile::SouthWest.step(current, current.south()),
            current.west()
        );

        assert_eq!(
            Tile::SouthEast.step(current, current.south()),
            current.east()
        );
        assert_eq!(
            Tile::SouthEast.step(current, current.east()),
            current.south()
        );
    }
}
