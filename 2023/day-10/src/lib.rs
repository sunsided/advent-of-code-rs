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
pub fn part2(input: &str) -> u64 {
    let map = parse_tiles(input);

    // The start lies on a tile. We assume the surrounding tiles connect to it meaningfully
    // (i.e. the are no ambiguities). We can allow this assumption because we know the
    // starting position is on a loop, and therefore cannot branch into a dead end.
    let start = map.find_start();
    let tile = map.infer_tile(&start);

    // Get a starting direction.
    let (mut current, _) = tile.expand(start);
    let mut previous = start;

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    enum MapState {
        None,
        Loop,
        Included,
    }

    // Create a map of all tiles that are on the loop.
    // We will later color it in such that all tiles inside the loop are marked.
    let mut loop_map = vec![MapState::None; map.tiles.len()];
    loop_map[map.to_index(start)] = MapState::Loop;

    // Walk the loop, filling in the loop outline on the map.
    while current != start {
        loop_map[map.to_index(current)] = MapState::Loop;
        let next = map.at(current).step(current, previous);
        (current, previous) = (next, current);
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    enum State {
        Outside,
        LeadingEdge,
        Inside,
        TrailingEdge,
    }

    // Fill in the loop map.
    let mut num_in_loop = 0;
    for l in 0..map.height {
        let line = &mut loop_map[l * map.width..(l + 1) * map.width];
        let mut state = State::Outside;

        for i in 0..line.len() {
            let is_loop_outline = line[i] == MapState::Loop;

            // Update state.
            state = match (state, is_loop_outline) {
                (State::Outside, false) => State::Outside,
                (State::Outside, true) => State::LeadingEdge,
                (State::LeadingEdge, true) => State::TrailingEdge,
                (State::LeadingEdge, false) => State::Inside,
                (State::Inside, false) => State::Inside,
                (State::Inside, true) => State::TrailingEdge,
                (State::TrailingEdge, true) => State::TrailingEdge,
                (State::TrailingEdge, false) => State::Outside,
            };

            // As long as we are in a loop outline, paint the map in.
            if state == State::Inside {
                num_in_loop += 1;
            }

            match state {
                State::Outside => line[i] = MapState::None,
                State::LeadingEdge => line[i] = MapState::Loop,
                State::Inside => line[i] = MapState::Included,
                State::TrailingEdge => line[i] = MapState::Loop,
            }
        }
    }

    let mut out = String::new();
    for l in 0..map.height {
        let line = &loop_map[l * map.width..(l + 1) * map.width];
        let str = String::from_iter(line.iter().map(|&state| match state {
            MapState::None => '.',
            MapState::Loop => '*',
            MapState::Included => 'I',
        }));
        out.push_str(&str);
        out.push('\n');
    }

    println!("{out}");

    num_in_loop
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
    height: usize,
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

    Map {
        tiles,
        width,
        height: num_lines,
    }
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

    fn to_index(&self, position: Coordinate) -> usize {
        position.x() + position.y() * self.width
    }

    fn at(&self, position: Coordinate) -> Tile {
        self.tiles[self.to_index(position)]
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
    fn test_part2_example1() {
        const TEST: &str = "...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........";

        assert_eq!(part2(TEST), 4);
    }

    #[test]
    fn test_part2_example2() {
        const TEST: &str = ".F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...";

        assert_eq!(part2(TEST), 8);
    }

    #[test]
    fn test_part2_example3() {
        const TEST: &str = "FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L";

        assert_eq!(part2(TEST), 10);
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
