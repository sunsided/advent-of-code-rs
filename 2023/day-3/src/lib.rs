use std::borrow::Borrow;
use std::collections::Bound;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::{RangeBounds, RangeInclusive};
use std::str::FromStr;

/// The `Schematic` struct represents a schematic with valid and invalid part numbers.
#[derive(Debug)]
#[allow(dead_code)]
pub struct Schematic {
    /// A vector of `PartNumber` instances representing the valid part numbers.
    valid: Vec<PartNumber>,
    /// A vector of `PartNumber` instances representing the invalid part numbers.
    invalid: Vec<PartNumber>,
    /// The symbol map, used for gear detection.
    symbol_map: SymbolMap,
}

/// Represents a part number
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PartNumber {
    row: usize,
    pos: usize,
    len: usize,
    number: u32,
}

/// `SymbolMap` is a struct that represents a grid of symbols, where each symbol can be either true or false.
/// It is used to keep track of the state of symbols in a grid, such as the state of pixels in an image.
#[derive(Debug, Clone)]
struct SymbolMap {
    /// The number of lines.
    num_lines: usize,
    /// The length of each line.
    line_length: usize,
    /// The map of symbols.
    map: Vec<SymbolType>,
    /// The set of potential gear symbols.
    potential_gears: Vec<SymbolPosition>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum SymbolType {
    /// Not a symbol.
    None,
    /// An arbitrary symbol.
    Generic,
    /// A potential gear. To be a proper gear, this symbol must be next to two part numbers.
    GearCandidate,
}

#[derive(Debug, Clone)]
pub struct SymbolPosition {
    x: usize,
    y: usize,
}

impl SymbolType {
    /// Determines if the current value represents a symbol.
    fn is_symbol(&self) -> bool {
        match self {
            SymbolType::None => false,
            SymbolType::Generic => true,
            SymbolType::GearCandidate => true,
        }
    }

    /// Determines if the current value represents a gear.
    fn is_potential_gear(&self) -> bool {
        match self {
            SymbolType::None => false,
            SymbolType::Generic => false,
            SymbolType::GearCandidate => true,
        }
    }
}

impl Schematic {
    /// Returns the number of valid items in the collection.
    pub fn num_valid(&self) -> usize {
        self.valid.len()
    }

    /// Returns the sum of the values in the valid parts.
    pub fn sum_valid_parts(&self) -> u32 {
        self.valid.iter().fold(0, |sum, part| sum + part.number)
    }

    /// Sums up all the gear ratios.
    pub fn sum_gear_ratios(&self) -> u32 {
        let mut sum = 0;
        for potential_gear in self.symbol_map.potential_gears() {
            // Select only those numbers that fall into the relevant line range.
            let lower = self.valid.partition_point(|p| p.row < potential_gear.y - 1);
            let upper = self
                .valid
                .partition_point(|p| p.row <= potential_gear.y + 1);

            // Find all adjacent numbers. This is beautifully shitty as it goes through all numbers
            // in the relevant line range, even though we could limit them by x offset.
            let values: Vec<_> = self.valid[lower..upper]
                .iter()
                .filter(|&part| part.is_adjacent(potential_gear))
                .map(|part| part.number)
                .collect();

            // "A gear is any * symbol that is adjacent to exactly two part numbers."
            debug_assert!(values.len() <= 2);
            if values.len() != 2 {
                continue;
            }

            let gear_ratio: u32 = values.iter().product();
            sum += gear_ratio;
        }

        sum
    }
}

impl SymbolPosition {
    /// Creates a new symbol position.
    ///
    /// # Example
    ///
    /// ```
    /// use aoc_2023_day_3::SymbolPosition;
    ///
    /// let pos = SymbolPosition::new(42, 1337);
    /// assert_eq!(pos.x(), 42);
    /// assert_eq!(pos.y(), 1337);
    /// ```
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Gets the x coordinate.
    ///
    /// # Example
    ///
    /// ```
    /// use aoc_2023_day_3::SymbolPosition;
    ///
    /// let pos = SymbolPosition::new(42, 1337);
    /// assert_eq!(pos.x(), 42)
    /// ```
    pub fn x(&self) -> usize {
        self.x
    }

    /// Gets the y coordinate.
    ///
    /// # Example
    ///
    /// ```
    /// use aoc_2023_day_3::SymbolPosition;
    ///
    /// let pos = SymbolPosition::new(42, 1337);
    /// assert_eq!(pos.y(), 1337)
    /// ```
    pub fn y(&self) -> usize {
        self.y
    }
}

impl PartNumber {
    /// Creates a new [`PartNumber`].
    pub fn new(pos: usize, row: usize, len: usize, number: u32) -> Self {
        debug_assert_eq!(format!("{number}").len(), len);
        Self {
            pos,
            row,
            len,
            number,
        }
    }

    /// Checks if the given position is adjacent to the current symbol position.
    ///
    /// # Arguments
    ///
    /// - `position`: The position to check for adjacency. Must implement the `Borrow<SymbolPosition>` trait.
    ///
    /// # Returns
    ///
    /// - `true` if the given position is adjacent to the current symbol position.
    /// - `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc_2023_day_3::{PartNumber, SymbolPosition};
    ///
    /// let part = PartNumber::new(2, 4, 5, 12345);
    ///
    /// let adjacent_position = SymbolPosition::new(3, 3);
    /// let nonadjacent_position = SymbolPosition::new(1, 2);
    ///
    /// assert!(part.is_adjacent(&adjacent_position));
    /// assert!(!part.is_adjacent(&nonadjacent_position));
    /// ```
    pub fn is_adjacent<P: Borrow<SymbolPosition>>(&self, position: P) -> bool {
        let position = position.borrow();
        let x = position.x as isize;
        let y = position.y as isize;

        let range_top = self.row as isize - 1;
        let range_bottom = self.row as isize + 1;

        let range_left = self.pos as isize - 1;
        let range_right = self.pos as isize + self.len as isize;

        x >= range_left && x <= range_right && y >= range_top && y <= range_bottom
    }
}

impl SymbolMap {
    pub fn potential_gears(&self) -> std::slice::Iter<SymbolPosition> {
        self.potential_gears.iter()
    }

    /// Checks if the specified address represents a symbol in the map.
    ///
    /// # Arguments
    ///
    /// * `x` - The column index of the address.
    /// * `y` - The row index of the address.
    ///
    /// # Returns
    ///
    /// Returns a boolean indicating whether the address represents a symbol in the map.
    ///
    /// # Errors
    ///
    /// Returns an `InvalidAddressError` if the specified address is out of bounds.
    #[allow(dead_code)]
    fn is_symbol(&self, x: usize, y: usize) -> Result<bool, InvalidAddressError> {
        if x >= self.line_length || y >= self.num_lines {
            return Err(InvalidAddressError(x, y));
        }

        Ok(self.map[y * self.line_length + x]).map(|s| s.is_symbol())
    }

    /// Checks if there is a symbol adjacent to the given row and range of columns.
    ///
    /// # Arguments
    ///
    /// * `columns` - The range of columns to check.
    /// * `row` - The row to check.
    ///
    /// # Returns
    ///
    /// Returns `true` if there is a symbol adjacent to the given row and range of columns,
    /// otherwise returns `false`.
    pub fn is_next_to_symbol(&self, columns: RangeInclusive<isize>, row: isize) -> bool {
        let symbol_on_top = self.contains_symbol(columns.clone(), row - 1);
        let symbol_on_bottom = self.contains_symbol(columns.clone(), row + 1);
        let symbol_on_left = self.contains_symbol(columns.clone(), row);
        let symbol_on_right = self.contains_symbol(columns, row);
        symbol_on_top || symbol_on_bottom || symbol_on_left || symbol_on_right
    }

    fn contains_symbol<R>(&self, columns: R, row: isize) -> bool
    where
        R: RangeBounds<isize>,
    {
        let columns = columns.borrow();
        if row < 0 || row as usize >= self.num_lines {
            return false;
        }

        let row = row as usize;

        let start = match columns.start_bound() {
            Bound::Included(&idx) => idx.max(0),
            Bound::Excluded(&idx) => (idx + 1).max(0),
            Bound::Unbounded => 0,
        };

        let end = match columns.end_bound() {
            Bound::Included(&idx) => idx.min(self.line_length as isize - 1),
            Bound::Excluded(&idx) => (idx - 1).max(0).min(self.line_length as isize - 1),
            Bound::Unbounded => (self.line_length - 1) as _,
        };

        if start > end {
            return false;
        }

        if end as usize >= self.line_length {
            return false;
        }

        let start = start as usize;
        let end = end as usize;

        let line_start = row * self.line_length;
        let line_end = line_start + self.line_length;
        let line = &self.map[line_start..line_end];

        let segment = &line[start..=end];
        segment.iter().any(|&is_symbol| is_symbol.is_symbol())
    }
}

impl From<char> for SymbolType {
    fn from(value: char) -> Self {
        if value == '*' {
            Self::GearCandidate
        } else if !value.is_ascii_digit() && value != '.' {
            Self::Generic
        } else {
            Self::None
        }
    }
}

impl FromStr for Schematic {
    type Err = ParseSchematicError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let symbol_map = SymbolMap::from_str(s)?;
        let line_len = symbol_map.line_length;

        let mut valid = Vec::new();
        let mut invalid = Vec::new();

        // We trim whitespace to make test input easier.
        'line: for (line_no, line) in s.lines().map(|l| l.trim()).enumerate() {
            if line.is_empty() {
                continue;
            }

            let mut start_pos = 0;
            while start_pos < line_len {
                // Find the position of the first digit in the line or skip to the next line.
                let first_digit = start_pos
                    + match line[start_pos..].bytes().position(|c| c.is_ascii_digit()) {
                        None => continue 'line,
                        Some(digit) => digit,
                    };

                // Find the position of the first non-digit after the specified position; if none
                // is found, return the line length.
                let first_non_digit = first_digit
                    + line[first_digit..]
                        .bytes()
                        .position(|c| !c.is_ascii_digit())
                        .unwrap_or(line.len() - first_digit);

                // Register start position for the next number.
                start_pos = first_non_digit;

                // Extract region containing numbers.
                debug_assert!(first_non_digit <= line_len);
                let digit = &line[first_digit..first_non_digit];

                // Test if we are surrounded by a symbol.
                let range = (first_digit as isize - 1)..=(first_non_digit as isize);
                let next_to_symbol = symbol_map.is_next_to_symbol(range, line_no as _);

                let part = PartNumber {
                    row: line_no,
                    pos: first_digit,
                    len: digit.len(),
                    number: u32::from_str(digit).map_err(|_| {
                        ParseSchematicError::Line(line_no, "Failed to parse part number")
                    })?,
                };

                if next_to_symbol {
                    valid.push(part);
                } else {
                    invalid.push(part);
                }
            }
        }

        Ok(Self {
            valid,
            invalid,
            symbol_map,
        })
    }
}

impl FromStr for SymbolMap {
    type Err = ParseSchematicError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(ParseSchematicError::NotAscii);
        }

        // We trim whitespace to make test input easier.
        let mut lines = s.lines().map(|l| l.trim()).peekable();
        let first_line = *lines.peek().ok_or(ParseSchematicError::InputEmpty)?;
        let line_length = first_line.len();
        if line_length >= isize::MAX as usize {
            return Err(ParseSchematicError::Line(0, "Input line too long"));
        }

        // We reserve capacity for the entire input length - this is typically oversized
        // as we do not need to keep the space for the newline characters. It is, however,
        // a safe upper bound that's not excessively large.
        let mut map = Vec::with_capacity(s.len());
        let mut potential_gears = Vec::new();

        let mut num_lines = 0;
        for (line_no, line) in lines.enumerate() {
            num_lines += 1;
            if line.len() != line_length {
                return Err(ParseSchematicError::Line(line_no, "Line length mismatch"));
            }

            // Convert every character into a boolean. true implies the character was a symbol,
            // false implies it was not. Dots do not count as a character as per the problem description.
            let symbol_detection = Vec::from_iter(line.chars().map(SymbolType::from));

            // Register all potential gear positions.
            potential_gears.extend(
                symbol_detection
                    .iter()
                    .enumerate()
                    .filter(|(_, &sym)| sym.is_potential_gear())
                    .map(|(x, _)| SymbolPosition { x, y: line_no }),
            );

            // Register all symbols.
            map.extend(symbol_detection);
        }

        map.shrink_to_fit();
        Ok(SymbolMap {
            num_lines,
            line_length,
            map,
            potential_gears,
        })
    }
}

/// Represents an error that can occur during parsing of a schematic.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ParseSchematicError {
    NotAscii,
    InputEmpty,
    Line(usize, &'static str),
}

impl Display for ParseSchematicError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseSchematicError::NotAscii => write!(f, "The input is not proper ASCII"),
            ParseSchematicError::InputEmpty => write!(f, "The input is empty"),
            ParseSchematicError::Line(line_no, message) => {
                write!(f, "Error in line {line_no}: {message}")
            }
        }
    }
}

impl Error for ParseSchematicError {}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct InvalidAddressError(usize, usize);

impl Display for InvalidAddressError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "The specified address is invalid: {}, {}",
            self.0, self.1
        )
    }
}

impl Error for InvalidAddressError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schematic_from_string() {
        const EXAMPLE: &str = "467..114..
                               ...*......
                               ..35..633.
                               ......#...
                               617*......
                               .....+.58.
                               ..592.....
                               ......755.
                               ...$.*....
                               .664.598..
                               ......*997";
        let schematic = Schematic::from_str(EXAMPLE).expect("failed to parse schematic");

        assert_eq!(
            schematic.valid.len(),
            9,
            "More valid numbers found than expected"
        );
        assert!(schematic.valid.iter().any(|p| p.number == 467));
        assert!(schematic.valid.iter().any(|p| p.number == 35));
        assert!(schematic.valid.iter().any(|p| p.number == 633));
        assert!(schematic.valid.iter().any(|p| p.number == 617));
        assert!(schematic.valid.iter().any(|p| p.number == 592));
        assert!(schematic.valid.iter().any(|p| p.number == 755));
        assert!(schematic.valid.iter().any(|p| p.number == 664));
        assert!(schematic.valid.iter().any(|p| p.number == 598));
        assert!(schematic.valid.iter().any(|p| p.number == 997));

        assert_eq!(
            schematic.invalid.len(),
            2,
            "More invalid numbers found than expected"
        );
        assert!(schematic.invalid.iter().any(|p| p.number == 114));
        assert!(schematic.invalid.iter().any(|p| p.number == 58));
    }

    #[test]
    fn test_sum_valid_parts() {
        const EXAMPLE: &str = "467..114..
                               ...*......
                               ..35..633.
                               ......#...
                               617*......
                               .....+.58.
                               ..592.....
                               ......755.
                               ...$.*....
                               .664.598..
                               ......*997";
        let schematic = Schematic::from_str(EXAMPLE).expect("failed to parse schematic");
        assert_eq!(schematic.sum_valid_parts(), 4361 + 997);
    }

    #[test]
    fn test_sum_gear_ratios() {
        const EXAMPLE: &str = "467..114..
                               ...*......
                               ..35..633.
                               ......#...
                               617*......
                               .....+.58.
                               ..592.....
                               ......755.
                               ...$.*....
                               .664.598..
                               ......*997";
        let schematic = Schematic::from_str(EXAMPLE).expect("failed to parse schematic");
        assert_eq!(schematic.sum_gear_ratios(), 467835 + 598 * 997);
    }

    #[test]
    fn test_symbol_map_from_string_single_line() {
        let map = SymbolMap::from_str("...$.*....").expect("failed to parse input");
        assert_eq!(map.num_lines, 1);
        assert_eq!(map.line_length, 10);

        assert_eq!(map.is_symbol(3, 0), Ok(true));
        assert_eq!(map.is_symbol(5, 0), Ok(true));

        assert_eq!(map.is_symbol(0, 0), Ok(false));
        assert_eq!(map.is_symbol(1, 0), Ok(false));
        assert_eq!(map.is_symbol(2, 0), Ok(false));
        assert_eq!(map.is_symbol(4, 0), Ok(false));
        assert_eq!(map.is_symbol(6, 0), Ok(false));
        assert_eq!(map.is_symbol(7, 0), Ok(false));
        assert_eq!(map.is_symbol(8, 0), Ok(false));
        assert_eq!(map.is_symbol(9, 0), Ok(false));

        assert_eq!(map.is_symbol(10, 0), Err(InvalidAddressError(10, 0)));
        assert_eq!(map.is_symbol(9, 1), Err(InvalidAddressError(9, 1)));
    }

    #[test]
    fn test_symbol_map_from_string_multi_line() {
        let map = SymbolMap::from_str("...$.*....\n.....+.58.").expect("failed to parse input");
        assert_eq!(map.num_lines, 2);
        assert_eq!(map.line_length, 10);

        assert_eq!(map.is_symbol(3, 0), Ok(true));
        assert_eq!(map.is_symbol(5, 0), Ok(true));
        assert_eq!(map.is_symbol(5, 1), Ok(true));

        assert_eq!(map.is_symbol(0, 0), Ok(false));
        assert_eq!(map.is_symbol(1, 0), Ok(false));
        assert_eq!(map.is_symbol(2, 0), Ok(false));
        assert_eq!(map.is_symbol(4, 0), Ok(false));
        assert_eq!(map.is_symbol(6, 0), Ok(false));
        assert_eq!(map.is_symbol(7, 0), Ok(false));
        assert_eq!(map.is_symbol(8, 0), Ok(false));
        assert_eq!(map.is_symbol(9, 0), Ok(false));

        assert_eq!(map.is_symbol(0, 1), Ok(false));
        assert_eq!(map.is_symbol(1, 1), Ok(false));
        assert_eq!(map.is_symbol(2, 1), Ok(false));
        assert_eq!(map.is_symbol(3, 1), Ok(false));
        assert_eq!(map.is_symbol(4, 1), Ok(false));
        assert_eq!(map.is_symbol(6, 1), Ok(false));
        assert_eq!(map.is_symbol(7, 1), Ok(false));
        assert_eq!(map.is_symbol(8, 1), Ok(false));
        assert_eq!(map.is_symbol(9, 1), Ok(false));

        assert_eq!(map.is_symbol(10, 0), Err(InvalidAddressError(10, 0)));
        assert_eq!(map.is_symbol(9, 2), Err(InvalidAddressError(9, 2)));

        assert_eq!(map.potential_gears.len(), 1);
    }

    #[test]
    fn test_contains_symbol() {
        let map = SymbolMap::from_str("...$.*....\n.....*.58.").expect("failed to parse input");
        assert!(map.contains_symbol(0.., 0));
        assert!(map.contains_symbol(3..=3, 0));
        assert!(!map.contains_symbol(3..=3, 1));
        assert_eq!(map.potential_gears.len(), 2);
    }

    #[test]
    fn test_symbol_type_from_char() {
        assert_eq!(SymbolType::from('*'), SymbolType::GearCandidate);
        assert_eq!(SymbolType::from('%'), SymbolType::Generic);
        assert_eq!(SymbolType::from('0'), SymbolType::None);
        assert_eq!(SymbolType::from('.'), SymbolType::None);
    }

    #[test]
    fn test_part_number_is_adjacent() {
        let part = PartNumber {
            row: 17,
            pos: 4,
            len: 4,
            number: 1234,
        };
        assert!(part.is_adjacent(SymbolPosition { x: 3, y: 16 }));
        assert!(part.is_adjacent(SymbolPosition { x: 3, y: 17 }));
        assert!(part.is_adjacent(SymbolPosition { x: 3, y: 18 }));

        assert!(part.is_adjacent(SymbolPosition { x: 8, y: 16 }));
        assert!(part.is_adjacent(SymbolPosition { x: 8, y: 17 }));
        assert!(part.is_adjacent(SymbolPosition { x: 8, y: 18 }));

        // ....1234....

        assert!(!part.is_adjacent(SymbolPosition { x: 2, y: 17 }));
        assert!(!part.is_adjacent(SymbolPosition { x: 9, y: 17 }));
        assert!(!part.is_adjacent(SymbolPosition { x: 8, y: 15 }));
        assert!(!part.is_adjacent(SymbolPosition { x: 9, y: 16 }));
    }
}
