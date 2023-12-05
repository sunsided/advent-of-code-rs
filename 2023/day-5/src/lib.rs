use aoc_utils::parse_whitespace_delimited;
use itertools::Itertools;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Range, Sub};
use std::str::FromStr;

mod macros;

pub trait AlmanacType:
    Copy
    + Clone
    + Eq
    + PartialEq
    + Ord
    + PartialOrd
    + FromStr
    + From<u64>
    + Into<u64>
    + Debug
    + Add<usize, Output = Self>
    + Add<u64, Output = Self>
    + Sub<Self, Output = usize>
{
}

pub struct Almanac {
    seeds: Vec<Seed>,
    seed_to_soil: MapRangeSet<Soil, Seed>,
    soil_to_fertilizer: MapRangeSet<Fertilizer, Soil>,
    fertilizer_to_water: MapRangeSet<Water, Fertilizer>,
    water_to_light: MapRangeSet<Light, Water>,
    light_to_temperature: MapRangeSet<Temperature, Light>,
    temperature_to_humidity: MapRangeSet<Humidity, Temperature>,
    humidity_to_location: MapRangeSet<Location, Humidity>,
}

struct MapRange<To, From> {
    /// The length of the range.
    length: usize,
    /// The destination range.
    destination: Range<To>,
    /// The source range.
    source: Range<From>,
    /// The smallest possible location reachable from this region. [`None`] if not yet determined.
    smallest_location: Option<Location>,
}

create_type!(Seed);
create_type!(Soil);
create_type!(Fertilizer);
create_type!(Water);
create_type!(Light);
create_type!(Temperature);
create_type!(Humidity);
create_type!(Location);

impl Almanac {
    /// Solution for the first part of the puzzle. Maps each loaded seed into a location.
    ///
    /// For each of the listed seeds we perform a lookup using [`map_seed`](Almanac::map_seed)
    /// and return the smallest location.
    pub fn map_smallest_from_seeds(&self) -> Option<(Seed, Location)> {
        self.seeds
            .iter()
            .map(|&seed| (seed, self.map_seed(seed)))
            .min_by(|(_, lhs), (_, rhs)| lhs.cmp(&rhs))
    }

    /// Solution for the second part of the puzzle. Treats each pair of seeds as a
    /// seed and a number of repetitions, then maps these.
    ///
    /// This solution works by back-propagating the possible locations up to the initial seeds.
    /// - First, all implied ranges ("Any source numbers that aren't mapped correspond to the same
    ///   destination number") are explicitly filled in.
    /// - Starting from the locations, all spots where a location slice change occurs in the
    ///   `humidity-to-location` map are marked in the previous map (`temperature-to-humidity`).
    ///   We know that locations are only sequentially growing within a range; by slicing the
    ///   ranges in the map above, we must re-evaluate the location for each slice when we search.
    /// - For each (now sliced) humidity entry in the `temperature-to-humidity` map, we slice
    ///   the `light-to-temperature` above.
    /// - We repeat these steps up until the `seeds-to-soil` map.
    ///
    /// These steps above already happen when parsing the [`Almanac`].
    ///
    /// - We now produce proper value ranges from the `seeds`.
    /// - For each range in the `seeds-to-soil` map we slice the seed ranges. For this
    ///   we detect all seed ranges that overlap with each individual `seed-to-soil` range.
    /// - The combination of these approaches guarantees that each individual slice of the seeds
    ///   now has growing location numbers; as a result, we only need to test the start of the
    ///   seed range using [`map_seed`](Almanac::map_seed).
    /// - The smallest location for each of these is the winner.
    pub fn map_smallest_from_seed_ranges(&self) -> Option<(Seed, Location)> {
        let mut seeds = Vec::new();
        for pair in &self.seeds.iter().chunks(2) {
            let pair = pair.collect::<Vec<_>>();
            let (&start, repetitions) = (pair[0], pair[1].value());
            seeds.push(start..start + repetitions)
        }
        seeds.sort_by_key(|range| range.start);

        // Slice the seeds according to the first map.
        let mut extra_slices = Vec::new();
        for range in &self.seed_to_soil.ranges {
            // If there is a seed range that contains a boundary, slice it.
            let positions: Vec<_> = seeds
                .iter()
                // find overlapping slices
                .filter(|seed| range.source.start < seed.end && seed.start < range.source.end)
                .enumerate()
                .map(|(pos, _)| pos)
                .collect();

            for pos in positions {
                let seed_range = &seeds[pos];

                // Don't slice direct matches.
                if seed_range.start == range.source.start {
                    continue;
                }

                let updated_range = seed_range.start..range.source.start;
                let sliced_range = range.source.start..seed_range.end;
                seeds[pos] = updated_range;
                extra_slices.push(sliced_range);
            }
        }

        seeds.extend(extra_slices);
        seeds.sort_by_key(|seed| seed.start);

        // Now iterate through all the seed ranges. The start index corresponds to the smallest
        // possible location.
        let mut best_location: Option<Location> = None;
        let mut best_seed: Option<Seed> = None;
        for seed in seeds {
            let better = self.map_seed(seed.start);

            if let Some(location) = best_location {
                if better >= location {
                    continue;
                }
            }

            best_location = Some(better);
            best_seed = Some(seed.start);

            // Sanity check that the end of the sliced seeds is indeed a larger location.
            let last = self.map_seed(Seed::from(seed.end.value() - 1));
            debug_assert!(last > better);
        }

        Some((
            best_seed.expect("found no location"),
            best_location.expect("found no location"),
        ))
    }

    fn map_seed(&self, seed: Seed) -> Location {
        let soil = self.seed_to_soil.map(seed);
        let fertilizer = self.soil_to_fertilizer.map(soil);
        let water = self.fertilizer_to_water.map(fertilizer);
        let light = self.water_to_light.map(water);
        let temperature = self.light_to_temperature.map(light);
        let humidity = self.temperature_to_humidity.map(temperature);
        self.humidity_to_location.map(humidity)
    }

    fn parse_section<Destination, Source>(
        section: &str,
        name: &str,
    ) -> Result<MapRangeSet<Destination, Source>, ParseAlmanacError>
    where
        Destination: AlmanacType,
        Source: AlmanacType,
    {
        let mut lines = section.lines().map(|line| line.trim());
        if let Some(heading) = lines.next() {
            if !heading.starts_with(name) || !heading.ends_with(" map:") {
                return Err(ParseAlmanacError("invalid section header"));
            }
        }

        let maps: Vec<_> = lines
            .map(MapRange::<Destination, Source>::from_str)
            .collect::<Result<_, _>>()
            .map_err(|_| ParseAlmanacError("unable to parse map range"))?;

        Ok(MapRangeSet::from(maps))
    }

    /// Patches the almanac, ensuring that the optimal
    fn optimize_after_construction(&mut self) {
        self.humidity_to_location.sort();

        // For the last map (humidity to location), the lowest possible location for
        // each entry is the destination itself.
        for entry in &mut self.humidity_to_location.ranges {
            entry.smallest_location = Some(entry.destination.start);

            // While we iterate, we can create slices in the map right before us, the
            // temperature to humidity map.
            self.temperature_to_humidity.slice(entry.source.start);
            // self.temperature_to_humidity.slice(entry.source.end);
        }
        self.temperature_to_humidity.sort();

        // Slice the light to temperature map.
        for entry in &self.temperature_to_humidity.ranges {
            self.light_to_temperature.slice(entry.source.start);
            // self.light_to_temperature.slice(entry.source.end);
        }
        self.light_to_temperature.sort();

        // Slice the water to light map.
        for entry in &self.light_to_temperature.ranges {
            self.water_to_light.slice(entry.source.start);
            // self.water_to_light.slice(entry.source.end);
        }
        self.water_to_light.sort();

        // Slice the fertilizer to water map.
        for entry in &self.water_to_light.ranges {
            self.fertilizer_to_water.slice(entry.source.start);
            // self.fertilizer_to_water.slice(entry.source.end);
        }
        self.fertilizer_to_water.sort();

        // Slice the soil to fertilizer map.
        for entry in &self.fertilizer_to_water.ranges {
            self.soil_to_fertilizer.slice(entry.source.start);
            // self.soil_to_fertilizer.slice(entry.source.end);
        }
        self.soil_to_fertilizer.sort();

        // Slice the seed to soil map.
        for entry in &self.soil_to_fertilizer.ranges {
            self.seed_to_soil.slice(entry.source.start);
            // self.seed_to_soil.slice(entry.source.end);
        }
        self.seed_to_soil.sort();

        // For seed to soil segment, determine the smallest reachable location.
        let smallest_locations: Vec<_> = self
            .seed_to_soil
            .ranges
            .iter()
            .map(|map| map.source.start)
            .map(|seed| self.map_seed(seed))
            .collect();

        // Determine the location at the section end for testing purposes.
        let largest_locations: Vec<_> = self
            .seed_to_soil
            .ranges
            .iter()
            .map(|map| map.source.end - 1.into())
            .map(|num| Seed::from(num as u64))
            .map(|seed| self.map_seed(seed))
            .collect();

        for ((map, location_at_section_start), location_at_section_end) in self
            .seed_to_soil
            .ranges
            .iter_mut()
            .zip(smallest_locations)
            .zip(largest_locations)
        {
            map.smallest_location = Some(location_at_section_start);
            debug_assert!(location_at_section_start < location_at_section_end);
        }
    }
}

struct MapRangeSet<Destination, Source> {
    ranges: Vec<MapRange<Destination, Source>>,
}

impl<Destination, Source> MapRangeSet<Destination, Source>
where
    Destination: AlmanacType,
    Source: AlmanacType,
{
    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.ranges.len()
    }

    fn map(&self, source: Source) -> Destination {
        self.ranges
            .iter()
            .filter(|&map| map.source.start <= source)
            .filter(|&map| map.source.end > source)
            .find_map(|map| map.map(source))
            .expect("not all ranges are covered")
    }

    /// Sorts the set, e.g. after a call to [`slice`](MapRangeSet::slice).
    fn sort(&mut self) {
        self.ranges.sort_by_key(|r| r.source.start);
    }

    /// Slices the map set so that the [`MapRange`] containing the destination index is split
    /// across that index, such that the left part does not contain it and the right part start
    /// with it.
    ///
    /// After this, the segment list is unsorted and should be [sorted](MapRangeSet::sort) again for proper use.
    fn slice(&mut self, index: Destination) {
        let pos = match self
            .ranges
            .iter_mut()
            .position(|map| map.destination.start <= index && map.destination.end > index)
        {
            // It's possible that the destination range is unmapped in the current set.
            None => return,
            Some(pos) => pos,
        };

        // Don't slice if it's an exact boundary.
        if self.ranges[pos].destination.start == index {
            return;
        }

        let sliced_range = self.ranges[pos].slice(index);
        self.ranges.push(sliced_range);
    }
}

impl<To, From> MapRange<To, From> {
    pub fn new(destination: To, source: From, count: usize) -> Self
    where
        From: Add<usize, Output = From> + Copy,
        To: Add<usize, Output = To> + Copy,
    {
        Self {
            length: count,
            destination: destination..(destination + count),
            source: source..(source + count),
            smallest_location: None,
        }
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn map(&self, source: From) -> Option<To>
    where
        From: AlmanacType,
        To: AlmanacType,
    {
        if source < self.source.start || source >= self.source.end {
            return None;
        }

        let offset = source - self.source.start;
        Some(self.destination.start + offset)
    }

    /// Slices the map set so that the [`MapRange`] containing the destination index is split
    /// across that index, such that the left part does not contain it and the right part start
    /// with it.
    ///
    /// After this, the segment list is unsorted and should be sorted again for proper use.
    fn slice(&mut self, index: To) -> MapRange<To, From>
    where
        To: AlmanacType,
        From: AlmanacType,
    {
        debug_assert!(self.destination.contains(&index));

        // The offset within the range at which to cut.
        let offset = index - self.destination.start;

        // The length prior to cutting.
        let current_length = self.length;

        let new_range = MapRange {
            source: self.source.start + offset..self.source.end,
            destination: self.destination.start + offset..self.destination.end,
            length: current_length - offset,
            smallest_location: None,
        };

        *self = MapRange {
            source: self.source.start..self.source.start + offset,
            destination: self.destination.start..self.destination.start + offset,
            length: offset,
            smallest_location: None,
        };

        new_range
    }
}

impl<Destination, Source> From<Vec<MapRange<Destination, Source>>>
    for MapRangeSet<Destination, Source>
where
    Destination: AlmanacType,
    Source: AlmanacType,
{
    fn from(mut ranges: Vec<MapRange<Destination, Source>>) -> Self {
        ranges.sort_by_key(|r| r.source.start);

        // Find holes and plug them. This provides full coverage of the entire value space.
        let mut next_start = 0;
        let mut plugs = Vec::new();
        for range in &ranges {
            let range_start = range.source.start.into();
            if range_start > next_start {
                let length = range_start - next_start;
                debug_assert!(next_start < range_start);
                plugs.push(MapRange {
                    source: Source::from(next_start)..Source::from(range_start),
                    destination: Destination::from(next_start)
                        ..Destination::from(next_start) + (length as usize),
                    length: length as _,
                    smallest_location: None,
                })
            }
            next_start = range.source.end.into();
        }

        // Merge and sort.
        if !plugs.is_empty() {
            ranges.extend(plugs.into_iter());
            ranges.sort_by_key(|r| r.source.start);
        }

        // Fill in the last range. We do this after sorting because it always goes last anyway.
        let last_range = &ranges[ranges.len() - 1];
        debug_assert!(last_range.source.end > 0.into());

        let last_range_start = last_range.source.end.into();
        ranges.push(MapRange {
            source: Source::from(last_range_start)..Source::from(u64::MAX),
            destination: Destination::from(next_start)..Destination::from(u64::MAX),
            length: (u64::MAX - last_range_start) as usize,
            smallest_location: None,
        });

        Self { ranges }
    }
}

impl<Destination, Source> FromIterator<MapRange<Destination, Source>>
    for MapRangeSet<Destination, Source>
where
    Destination: AlmanacType,
    Source: AlmanacType,
{
    fn from_iter<T: IntoIterator<Item = MapRange<Destination, Source>>>(iter: T) -> Self {
        let ranges: Vec<MapRange<Destination, Source>> = iter.into_iter().collect();
        ranges.into()
    }
}

impl FromStr for Almanac {
    type Err = ParseAlmanacError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = s
            .split_terminator("\n\n")
            .map(|line| line.trim())
            .filter(|&line| !line.is_empty());

        // The seeds.
        let seeds = if let Some(section) = sections.next() {
            if !section.starts_with("seeds:") {
                return Err(ParseAlmanacError("invalid seeds section"));
            }

            parse_seeds(section[6..].trim()).map_err(|_| ParseAlmanacError("invalid seeds"))?
        } else {
            return Err(ParseAlmanacError("Missing seeds section"));
        };

        // The seed-to-soil map.
        let seed_to_soil = if let Some(section) = sections.next() {
            Self::parse_section::<Soil, Seed>(section, "seed-to-soil")?
        } else {
            return Err(ParseAlmanacError("Missing seed-to-soil map section"));
        };

        // The soil-to-fertilizer map.
        let soil_to_fertilizer = if let Some(section) = sections.next() {
            Self::parse_section::<Fertilizer, Soil>(section, "soil-to-fertilizer")?
        } else {
            return Err(ParseAlmanacError("Missing soil-to-fertilizer map section"));
        };

        // The fertilizer-to-water map.
        let fertilizer_to_water = if let Some(section) = sections.next() {
            Self::parse_section::<Water, Fertilizer>(section, "fertilizer-to-water")?
        } else {
            return Err(ParseAlmanacError("Missing fertilizer-to-water map section"));
        };

        // The water-to-light map.
        let water_to_light = if let Some(section) = sections.next() {
            Self::parse_section::<Light, Water>(section, "water-to-light")?
        } else {
            return Err(ParseAlmanacError("Missing water-to-light map section"));
        };

        // The light-to-temperature map.
        let light_to_temperature = if let Some(section) = sections.next() {
            Self::parse_section::<Temperature, Light>(section, "light-to-temperature")?
        } else {
            return Err(ParseAlmanacError(
                "Missing light-to-temperature map section",
            ));
        };

        // The temperature-to-humidity map.
        let temperature_to_humidity = if let Some(section) = sections.next() {
            Self::parse_section::<Humidity, Temperature>(section, "temperature-to-humidity")?
        } else {
            return Err(ParseAlmanacError(
                "Missing temperature-to-humidity map section",
            ));
        };

        // The humidity-to-location map.
        let humidity_to_location = if let Some(section) = sections.next() {
            Self::parse_section::<Location, Humidity>(section, "humidity-to-location")?
        } else {
            return Err(ParseAlmanacError(
                "Missing humidity-to-location map section",
            ));
        };

        let mut almanac = Almanac {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        };

        almanac.optimize_after_construction();

        Ok(almanac)
    }
}

impl<To, From> FromStr for MapRange<To, From>
where
    From: AlmanacType,
    To: AlmanacType,
{
    type Err = ParseMapRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut destination = None;
        let mut source = None;
        let mut count = None;
        for item in s.split_whitespace() {
            if destination.is_none() {
                destination = Some(
                    To::from_str(item)
                        .map_err(|_| ParseMapRangeError("failed to parse destination type"))?,
                );
            } else if source.is_none() {
                source = Some(
                    From::from_str(item)
                        .map_err(|_| ParseMapRangeError("failed to parse source type"))?,
                );
            } else if count.is_none() {
                count = Some(
                    usize::from_str(item)
                        .map_err(|_| ParseMapRangeError("failed to parse length"))?,
                );
            } else {
                return Err(ParseMapRangeError("sequence is longer than expected"));
            }
        }

        let destination = destination.ok_or(ParseMapRangeError("no destination value provided"))?;
        let source = source.ok_or(ParseMapRangeError("no source value provided"))?;
        let count = count.ok_or(ParseMapRangeError("no count provided"))?;

        Ok(Self::new(destination, source, count))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct ParseMapRangeError(&'static str);

impl Display for ParseMapRangeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse range: {}", self.0)
    }
}

impl Error for ParseMapRangeError {}

#[derive(Debug, Eq, PartialEq)]
pub struct ParseAlmanacError(&'static str);

impl Display for ParseAlmanacError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse almanac: {}", self.0)
    }
}

impl Error for ParseAlmanacError {}

fn parse_seeds<S>(input: S) -> Result<Vec<Seed>, ParseSeedError>
where
    S: AsRef<str>,
{
    parse_whitespace_delimited::<Seed>(input.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_seeds() {
        assert_eq!(
            parse_seeds("59 42 3").expect("parsing failed"),
            [Seed(59), Seed(42), Seed(3)]
        );
        assert_eq!(
            parse_seeds("59a")
                .expect_err("parsing did not fail")
                .to_string(),
            "Failed to parse a Seed: invalid digit found in string"
        );
    }

    #[test]
    fn test_map_range() {
        let range = MapRange::new(Soil(50), Seed(98), 2);
        assert_eq!(range.len(), 2);
        assert_eq!(range.map(Seed(97)), None);
        assert_eq!(range.map(Seed(98)), Some(Soil(50)));
        assert_eq!(range.map(Seed(99)), Some(Soil(51)));
        assert_eq!(range.map(Seed(100)), None);

        // End is exclusive.
        assert!(range.source.contains(&Seed(99)));
        assert!(!range.source.contains(&Seed(100)));
    }

    #[test]
    fn test_parse_range() {
        let range = MapRange::<Soil, Seed>::from_str("50 98 2").expect("failed to parse range");
        assert_eq!(range.len(), 2);
    }

    #[test]
    fn test_parse_almanac() {
        const EXAMPLE: &str = "seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4";

        let almanac = Almanac::from_str(EXAMPLE).expect("failed to parse almanac");
        assert_eq!(almanac.seeds.len(), 4);
        /*
        assert_eq!(almanac.seed_to_soil.len(), 4);
        assert_eq!(almanac.soil_to_fertilizer.len(), 4);
        assert_eq!(almanac.fertilizer_to_water.len(), 5);
        assert_eq!(almanac.water_to_light.len(), 4);
        assert_eq!(almanac.light_to_temperature.len(), 5);
        assert_eq!(almanac.temperature_to_humidity.len(), 3);
        assert_eq!(almanac.humidity_to_location.len(), 4);
        */

        assert_eq!(almanac.map_seed(Seed(79)), Location(82));
        assert_eq!(almanac.map_seed(Seed(14)), Location(43));
        assert_eq!(almanac.map_seed(Seed(55)), Location(86));
        assert_eq!(almanac.map_seed(Seed(13)), Location(35));
    }

    #[test]
    fn test_slice_range() {
        let mut range = MapRange::<Soil, Seed>::from_str("50 98 3").expect("failed to parse range");
        let sliced = range.slice(Soil(51));

        assert_eq!(range.len(), 1);
        assert_eq!(sliced.len(), 2);

        // The original range starts where it started before.
        assert_eq!(range.source.start, Seed(98));
        assert_eq!(range.destination.start, Soil(50));

        // The end is exclusive.
        assert_eq!(range.source.end, Seed(99));
        assert_eq!(range.destination.end, Soil(51));

        // The sliced range starts at the (exclusive) end of the previous slice.
        assert_eq!(sliced.source.start, Seed(99));
        assert_eq!(sliced.destination.start, Soil(51));

        // The end is exclusive.
        assert_eq!(sliced.source.end, Seed(101));
        assert_eq!(sliced.destination.end, Soil(53));
    }

    #[test]
    fn test_slice_range_set() {
        let mut set = MapRangeSet::from(vec![
            MapRange::<Soil, Seed>::from_str("50 98 3").expect("failed to parse range"),
            MapRange::<Soil, Seed>::from_str("52 50 48").expect("failed to parse range"),
        ]);

        assert_eq!(set.len(), 4);
        assert_eq!(set.ranges[0].source.start, Seed(0));
        assert_eq!(set.ranges[0].destination.start, Soil(0));

        assert_eq!(set.ranges[1].source.start, Seed(50));
        assert_eq!(set.ranges[1].destination.start, Soil(52));

        assert_eq!(set.ranges[2].source.start, Seed(98));
        assert_eq!(set.ranges[2].destination.start, Soil(50));

        assert_eq!(set.ranges[3].source.start, Seed(101));
        assert_eq!(set.ranges[3].destination.start, Soil(101));

        set.slice(Soil(51));
        set.sort();

        assert_eq!(set.len(), 5);
        assert_eq!(set.ranges[0].source.start, Seed(0));
        assert_eq!(set.ranges[0].destination.start, Soil(0));

        assert_eq!(set.ranges[1].source.start, Seed(50));
        assert_eq!(set.ranges[1].destination.start, Soil(52));

        assert_eq!(set.ranges[2].source.start, Seed(98));
        assert_eq!(set.ranges[2].destination.start, Soil(50));

        assert_eq!(set.ranges[3].source.start, Seed(99)); // the sliced one
        assert_eq!(set.ranges[3].destination.start, Soil(51));

        assert_eq!(set.ranges[4].source.start, Seed(101));
        assert_eq!(set.ranges[4].destination.start, Soil(101));
    }

    #[test]
    fn test_slice_range_set_noop() {
        let mut set = MapRangeSet::from(vec![
            MapRange::<Soil, Seed>::from_str("50 98 3").expect("failed to parse range"),
            MapRange::<Soil, Seed>::from_str("52 50 48").expect("failed to parse range"),
        ]);

        assert_eq!(set.len(), 4);
        assert_eq!(set.ranges[0].source.start, Seed(0));
        assert_eq!(set.ranges[1].source.start, Seed(50));
        assert_eq!(set.ranges[2].source.start, Seed(98));
        assert_eq!(set.ranges[3].source.start, Seed(101));

        // This slice should be no-op because it's on an exact destination boundary.
        set.slice(Soil(50));
        set.sort();

        assert_eq!(set.len(), 4);
        assert_eq!(set.ranges[0].source.start, Seed(0));
        assert_eq!(set.ranges[1].source.start, Seed(50));
        assert_eq!(set.ranges[2].source.start, Seed(98));
        assert_eq!(set.ranges[3].source.start, Seed(101));
    }
}
