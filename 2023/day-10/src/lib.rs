use std::borrow::Borrow;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

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
pub fn part2(input: &str, print_map: bool) -> usize {
    let mut map = parse_tiles(input);

    // The start lies on a tile. We assume the surrounding tiles connect to it meaningfully
    // (i.e. the are no ambiguities). We can allow this assumption because we know the
    // starting position is on a loop, and therefore cannot branch into a dead end.
    let start = map.find_start();
    let tile = map.infer_tile(&start);

    // Replace the start tile.
    let start_tile_index = map.to_index(start);
    map.tiles[start_tile_index] = tile;

    // Widen the map.
    let map = map.widen();

    // Obtain the start position in the widened map.
    let start = Coordinate(start.x() * 2, start.y() * 2);

    // Get a starting direction.
    let (current, _) = tile.expand(start);
    let mut loop_map = prepare_loop_map(&map, start, current);

    // Flood-fill the outside
    flood_fill_outside(&map, &mut loop_map);

    // Reduce the map again.
    let small_loop_map = shrink_loop_map(&map, &loop_map);

    // Print the reduced map.
    if print_map {
        print_final_loop_map(&map, &small_loop_map);
    }

    // Count the number of remaining spots in the map.
    let num_in_loop = small_loop_map
        .iter()
        .filter(|&state| *state == MapState::None)
        .count();

    num_in_loop
}

fn prepare_loop_map(map: &WidenedMap, start: Coordinate, mut current: Coordinate) -> Vec<MapState> {
    let mut previous = start;

    // Create a map of all tiles that are on the loop.
    // We will later color it in such that all tiles inside the loop are marked.
    let mut loop_map: Vec<_> = map
        .tiles
        .iter()
        .map(|&tile| match tile {
            Tile::Widened => MapState::Widened,
            _ => MapState::None,
        })
        .collect();

    // Walk the loop, filling in the loop outline on the map.
    loop_map[map.to_index(start)] = MapState::Loop;
    while current != start {
        loop_map[map.to_index(current)] = MapState::Loop;
        let next = map.at(current).step(current, previous);
        (current, previous) = (next, current);
    }
    loop_map
}

fn flood_fill_outside(map: &WidenedMap, loop_map: &mut [MapState]) {
    let mut seeds = Vec::new();
    for x in 0..map.width {
        // Top row.
        let coordinate = Coordinate(x, 0);
        let tile = map.at(coordinate);
        if tile == Tile::None || tile == Tile::Widened {
            loop_map[map.to_index(coordinate)] = MapState::Outside;
            seeds.push(coordinate);
        }

        // Bottom row.
        let coordinate = Coordinate(x, map.height - 1);
        let tile = map.at(coordinate);
        if tile == Tile::None || tile == Tile::Widened {
            loop_map[map.to_index(coordinate)] = MapState::Outside;
            seeds.push(coordinate);
        }
    }

    for y in 1..map.height {
        // Top column.
        let coordinate = Coordinate(0, y);
        let tile = map.at(coordinate);
        if tile == Tile::None || tile == Tile::Widened {
            loop_map[map.to_index(coordinate)] = MapState::Outside;
            seeds.push(coordinate);
        }

        // Right column.
        let coordinate = Coordinate(map.width - 1, y);
        let tile = map.at(coordinate);
        if tile == Tile::None || tile == Tile::Widened {
            loop_map[map.to_index(coordinate)] = MapState::Outside;
            seeds.push(coordinate);
        }
    }

    seeds.reverse();
    while let Some(seed) = seeds.pop() {
        // Check north side.
        if let Some(coordinate) = seed.maybe_north() {
            let tile = &mut loop_map[map.to_index(coordinate)];
            if *tile == MapState::None || *tile == MapState::Widened {
                *tile = MapState::Outside;
                seeds.push(coordinate);
            }
        } else {
            let thingy = loop_map[map.to_index(seed)];
            debug_assert_eq!(thingy, MapState::Outside);
        }

        // Check east side.
        if let Some(coordinate) = seed.maybe_east(map) {
            let tile = &mut loop_map[map.to_index(coordinate)];
            if *tile == MapState::None || *tile == MapState::Widened {
                *tile = MapState::Outside;
                seeds.push(coordinate);
            }
        }

        // Check south side.
        if let Some(coordinate) = seed.maybe_south(map) {
            let tile = &mut loop_map[map.to_index(coordinate)];
            if *tile == MapState::None || *tile == MapState::Widened {
                *tile = MapState::Outside;
                seeds.push(coordinate);
            }
        }

        // Check west side.
        if let Some(coordinate) = seed.maybe_west() {
            let tile = &mut loop_map[map.to_index(coordinate)];
            if *tile == MapState::None || *tile == MapState::Widened {
                *tile = MapState::Outside;
                seeds.push(coordinate);
            }
        } else {
            let thingy = loop_map[map.to_index(seed)];
            debug_assert_eq!(thingy, MapState::Outside);
        }
    }
}

fn shrink_loop_map(map: &WidenedMap, loop_map: &[MapState]) -> Vec<MapState> {
    let mut small_loop_map = vec![MapState::None; loop_map.len() / 4];
    for y in (0..map.height).step_by(2) {
        for x in (0..map.width).step_by(2) {
            let index = x + y * map.width;
            let state = loop_map[index];

            let index = (x / 2) + (y * map.width) / 4;
            small_loop_map[index] = state;
        }
    }
    small_loop_map
}

fn print_final_loop_map(map: &Map, small_loop_map: &[MapState]) {
    let mut out = String::new();
    for l in 0..(map.height / 2) {
        let line = &small_loop_map[l * (map.width / 2)..(l + 1) * (map.width / 2)];
        let str = String::from_iter(line.iter().map(|&state| match state {
            MapState::None => 'I',
            MapState::Loop => '*',
            MapState::Outside => 'O',
            MapState::Widened => unreachable!(),
        }));
        out.push_str(&str);
        out.push('\n');
    }
    println!("{out}");
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum MapState {
    None,
    Loop,
    Outside,
    Widened,
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
    Widened,
}

struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

struct WidenedMap(Map);

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

    fn widen(&self) -> WidenedMap {
        self.into()
    }
}

impl WidenedMap {
    fn to_index(&self, coordinate: Coordinate) -> usize {
        coordinate.x() + coordinate.y() * self.width
    }

    fn upgrade(&mut self, coordinate: Coordinate, new: Tile) {
        let index = self.to_index(coordinate);
        let tile = &mut self.tiles[index];
        if *tile == Tile::Widened {
            *tile = new;
        }
    }

    fn connects_north(&self, coordinate: Coordinate) -> bool {
        if coordinate.1 < 2 {
            return false;
        }

        let tile = self.tiles[self.to_index(coordinate)];
        let other = self.tiles[self.to_index(Coordinate(coordinate.0, coordinate.1 - 2))];
        tile.connects_north() && other.connects_south()
    }

    fn connects_west(&self, coordinate: Coordinate) -> bool {
        if coordinate.0 < 2 {
            return false;
        }

        let tile = self.tiles[self.to_index(coordinate)];
        let other = self.tiles[self.to_index(Coordinate(coordinate.0 - 2, coordinate.1))];
        tile.connects_west() && other.connects_east()
    }

    fn connects_south(&self, coordinate: Coordinate) -> bool {
        if coordinate.1 >= self.height - 2 {
            return false;
        }

        let tile = self.tiles[self.to_index(coordinate)];
        let other = self.tiles[self.to_index(Coordinate(coordinate.0, coordinate.1 + 2))];
        tile.connects_south() && other.connects_north()
    }

    fn connects_east(&self, coordinate: Coordinate) -> bool {
        if coordinate.0 >= self.width - 2 {
            return false;
        }

        let tile = self.tiles[self.to_index(coordinate)];
        let other = self.tiles[self.to_index(Coordinate(coordinate.0 + 2, coordinate.1))];
        tile.connects_east() && other.connects_west()
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

    pub fn has_south(&self, map: &Map) -> bool {
        self.1 < map.height - 1
    }

    pub fn has_east(&self, map: &Map) -> bool {
        self.0 < map.width - 1
    }

    pub fn maybe_north(&self) -> Option<Coordinate> {
        if self.has_north() {
            Some(self.north())
        } else {
            None
        }
    }

    pub fn maybe_west(&self) -> Option<Coordinate> {
        if self.has_west() {
            Some(self.west())
        } else {
            None
        }
    }

    pub fn maybe_east(&self, map: &Map) -> Option<Coordinate> {
        if self.has_east(map) {
            Some(self.east())
        } else {
            None
        }
    }

    pub fn maybe_south(&self, map: &Map) -> Option<Coordinate> {
        if self.has_south(map) {
            Some(self.south())
        } else {
            None
        }
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

    pub fn southeast(&self) -> Coordinate {
        Coordinate(self.0 + 1, self.1 + 1)
    }
}

impl Tile {
    pub fn expand<C: Borrow<Coordinate>>(&self, coordinate: C) -> (Coordinate, Coordinate) {
        let coordinate = coordinate.borrow();
        match self {
            Tile::None => panic!("Invalid call on a none-tile"),
            Tile::Widened => panic!("Invalid call on a none-tile"),
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
            Tile::Widened => false,
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
            Tile::Widened => false,
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
            Tile::Widened => false,
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
            Tile::Widened => false,
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
            Tile::Widened => panic!("can't call step on a none-tile"),
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

impl Deref for WidenedMap {
    type Target = Map;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WidenedMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<M> From<M> for WidenedMap
where
    M: Borrow<Map>,
{
    fn from(value: M) -> Self {
        let value = value.borrow();
        let mut map = WidenedMap(Map {
            tiles: vec![Tile::Widened; value.tiles.len() * 4],
            width: value.width * 2,
            height: value.height * 2,
        });

        // Fill in the base map.
        for y in 0..value.height {
            for x in 0..value.width {
                let coordinate = Coordinate(x, y);
                let tile = value.at(coordinate);

                // Place the regular tile.
                let base_coordinate = Coordinate(x * 2, y * 2);
                map.upgrade(base_coordinate, tile);
            }
        }

        // Fill in the gaps.
        for y in 0..value.height {
            for x in 0..value.width {
                let coordinate = Coordinate(x, y);
                let tile = value.at(coordinate);

                let base_coordinate = Coordinate(x * 2, y * 2);
                match tile {
                    Tile::None => {
                        // Place the tile east to it.
                        let new_coordinate = base_coordinate.east();
                        map.upgrade(new_coordinate, Tile::None);

                        // Place the tile south of it.
                        let new_coordinate = base_coordinate.south();
                        map.upgrade(new_coordinate, Tile::None);

                        // Place the tile southeast of it.
                        let new_coordinate = base_coordinate.southeast();
                        map.upgrade(new_coordinate, Tile::None);
                    }
                    Tile::Start => {
                        // nothing to do
                    }
                    Tile::NorthSouth => {
                        // Place the tile north to it.
                        if map.connects_north(base_coordinate) {
                            map.upgrade(base_coordinate.north(), Tile::NorthSouth);
                        }

                        // Place the tile south of it.
                        if map.connects_south(base_coordinate) {
                            map.upgrade(base_coordinate.south(), Tile::NorthSouth);
                        }
                    }
                    Tile::WestEast => {
                        // Place the tile west to it.
                        if map.connects_west(base_coordinate) {
                            map.upgrade(base_coordinate.west(), Tile::WestEast);
                        }

                        // Place the tile east to it.
                        if map.connects_east(base_coordinate) {
                            map.upgrade(base_coordinate.east(), Tile::WestEast);
                        }
                    }
                    Tile::NorthEast => {
                        // Place the tile north to it.
                        if map.connects_north(base_coordinate) {
                            map.upgrade(base_coordinate.north(), Tile::NorthSouth);
                        }

                        // Place the tile east to it.
                        if map.connects_east(base_coordinate) {
                            map.upgrade(base_coordinate.east(), Tile::WestEast);
                        }
                    }
                    Tile::NorthWest => {
                        // Place the tile north to it.
                        if map.connects_north(base_coordinate) {
                            map.upgrade(base_coordinate.north(), Tile::NorthSouth);
                        }

                        // Place the tile west to it.
                        if map.connects_west(base_coordinate) {
                            map.upgrade(base_coordinate.west(), Tile::WestEast);
                        }
                    }
                    Tile::SouthWest => {
                        // Place the tile west to it.
                        if map.connects_west(base_coordinate) {
                            map.upgrade(base_coordinate.west(), Tile::WestEast);
                        }

                        // Place the tile south of it.
                        if map.connects_south(base_coordinate) {
                            map.upgrade(base_coordinate.south(), Tile::NorthSouth);
                        }
                    }
                    Tile::SouthEast => {
                        // Place the tile east to it.
                        if map.connects_east(base_coordinate) {
                            map.upgrade(base_coordinate.east(), Tile::WestEast);
                        }

                        // Place the tile south of it.
                        if map.connects_south(base_coordinate) {
                            map.upgrade(base_coordinate.south(), Tile::NorthSouth);
                        }
                    }
                    Tile::Widened => unreachable!(),
                };
            }
        }

        map
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

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.at(Coordinate(x, y)))?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::None => write!(f, "."),
            Tile::Start => write!(f, "S"),
            Tile::NorthSouth => write!(f, "|"),
            Tile::WestEast => write!(f, "-"),
            Tile::NorthEast => write!(f, "L"),
            Tile::NorthWest => write!(f, "J"),
            Tile::SouthWest => write!(f, "7"),
            Tile::SouthEast => write!(f, "F"),
            Tile::Widened => write!(f, "*"),
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

        assert_eq!(part2(TEST, false), 4);
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

        assert_eq!(part2(TEST, false), 8);
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

        assert_eq!(part2(TEST, false), 10);
    }

    #[test]
    fn test_part2_real() {
        const TEST: &str = include_str!("../input.txt");
        assert_ne!(part2(TEST, false), 357);
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
