use std::collections::HashSet;

/// Solution for part 1.
pub fn part1(input: &str) -> usize {
    let (galaxies, width, height) = parse_galaxies(input);
    let galaxies = expand_universe(galaxies, width, height);

    let mut distance_sum = 0;
    let galaxies = galaxies.as_slice();
    for (i, galaxy) in galaxies[..galaxies.len() - 1].iter().enumerate() {
        for other in &galaxies[(i + 1)..] {
            // Calculate taxicab/Manhattan distance.
            let dx = galaxy.x.max(other.x) - galaxy.x.min(other.x);
            let dy = galaxy.y.max(other.y) - galaxy.y.min(other.y);
            let distance = dx + dy;
            distance_sum += distance;
        }
    }

    distance_sum
}

/// Solution for part 2.
pub fn part2(input: &str) -> usize {
    todo!()
}

fn parse_galaxies(input: &str) -> (Vec<Galaxy>, usize, usize) {
    let mut galaxies = Vec::new();
    let mut height = 0;
    let mut width = 0;
    for (y, line) in input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .enumerate()
    {
        width = line.len();
        height = y;

        let start_id = galaxies.len();
        galaxies.extend(
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .enumerate()
                .map(|(id, (x, _))| Galaxy {
                    id: start_id + id + 1,
                    x,
                    y,
                }),
        );
    }

    galaxies.sort_unstable();
    (galaxies, width, height)
}

fn expand_universe(mut galaxies: Vec<Galaxy>, width: usize, height: usize) -> Vec<Galaxy> {
    let rows: HashSet<usize> = HashSet::from_iter(0..height);
    let columns: HashSet<usize> = HashSet::from_iter(0..width);
    let observed_rows = HashSet::from_iter(galaxies.iter().map(|g| g.y));
    let observed_columns = HashSet::from_iter(galaxies.iter().map(|g| g.x));

    // Find rows that contain no galaxies and double their width.
    let mut missing_rows: Vec<_> = rows.difference(&observed_rows).cloned().collect();
    missing_rows.sort_unstable();
    for row in missing_rows.into_iter().rev() {
        for galaxy in galaxies.iter_mut() {
            debug_assert_ne!(galaxy.y, row);
            if galaxy.y >= row {
                galaxy.y += 1;
            }
        }
    }

    // Find columns that contain no galaxies and double their height.
    let mut missing_columns: Vec<_> = columns.difference(&observed_columns).cloned().collect();
    missing_columns.sort_unstable();
    for column in missing_columns.into_iter().rev() {
        for galaxy in galaxies.iter_mut() {
            debug_assert_ne!(galaxy.x, column);
            if galaxy.x >= column {
                galaxy.x += 1;
            }
        }
    }

    galaxies
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Galaxy {
    id: usize,
    x: usize,
    y: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = "...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
            ";
        assert_eq!(part1(INPUT), 374);
    }

    #[test]
    fn test_part2() {
        todo!()
    }

    #[test]
    fn test_parse_galaxies() {
        const INPUT: &str = "...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
            ";
        let (mut galaxies, width, height) = parse_galaxies(INPUT);
        assert_eq!(width, 10);
        assert_eq!(height, 9);

        let mut galaxies = galaxies.into_iter();
        assert_eq!(galaxies.next(), Some(Galaxy { id: 1, x: 3, y: 0 }));
        assert_eq!(galaxies.next(), Some(Galaxy { id: 2, x: 7, y: 1 }));
        assert_eq!(galaxies.next(), Some(Galaxy { id: 3, x: 0, y: 2 }));
        assert_eq!(galaxies.next(), Some(Galaxy { id: 4, x: 6, y: 4 }));
        assert_eq!(galaxies.next(), Some(Galaxy { id: 5, x: 1, y: 5 }));
        assert_eq!(galaxies.next(), Some(Galaxy { id: 6, x: 9, y: 6 }));
        assert_eq!(galaxies.next(), Some(Galaxy { id: 7, x: 7, y: 8 }));
        assert_eq!(galaxies.next(), Some(Galaxy { id: 8, x: 0, y: 9 }));
        assert_eq!(galaxies.next(), Some(Galaxy { id: 9, x: 4, y: 9 }));
    }

    #[test]
    fn test_expand_universe() {
        const INPUT: &str = "...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
            ";
        let (galaxies, width, height) = parse_galaxies(INPUT);
        let galaxies = expand_universe(galaxies, width, height);

        let mut galaxies = galaxies.into_iter();
        assert_eq!(galaxies.next(), Some(Galaxy { id: 1, x: 4, y: 0 }));
        assert_eq!(galaxies.next(), Some(Galaxy { id: 2, x: 9, y: 1 }));
        assert_eq!(galaxies.next(), Some(Galaxy { id: 3, x: 0, y: 2 }));
        assert_eq!(galaxies.next(), Some(Galaxy { id: 4, x: 8, y: 5 }));
        assert_eq!(galaxies.next(), Some(Galaxy { id: 5, x: 1, y: 6 }));
        assert_eq!(galaxies.next(), Some(Galaxy { id: 6, x: 12, y: 7 }));
        assert_eq!(galaxies.next(), Some(Galaxy { id: 7, x: 9, y: 10 }));
        assert_eq!(galaxies.next(), Some(Galaxy { id: 8, x: 0, y: 11 }));
        assert_eq!(galaxies.next(), Some(Galaxy { id: 9, x: 5, y: 11 }));
    }
}
