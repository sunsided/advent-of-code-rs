use aoc_utils::parse_whitespace_delimited;
use itertools::Itertools;
use rayon::prelude::*;
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
    + From<u32>
    + Into<u32>
    + Debug
    + Add<usize, Output = Self>
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
    length: usize,
    destination: Range<To>,
    source: Range<From>,
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
    pub fn map_smallest_from_seeds(&self) -> Option<(Seed, Location)> {
        self.seeds
            .iter()
            .map(|&seed| (seed, self.map_seed(seed)))
            .min_by(|(_, lhs), (_, rhs)| lhs.cmp(&rhs))
    }

    /// Solution for the second part of the puzzle. Treats each pair of seeds as a
    /// seed and a number of repetitions, then maps these.
    pub fn map_smallest_from_seed_ranges(&self) -> Option<(Seed, Location)> {
        self.seeds
            .iter()
            .tuple_windows()
            .flat_map(|(&seed, &repetitions)| {
                (0..repetitions.value() as usize).map(move |i| seed + i)
            })
            .par_bridge()
            .map(|seed| (seed, self.map_seed(seed)))
            .min_by_key(|(_, loc)| loc.value())
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
        // let partition = self.ranges.partition_point(|map| map.source.end < source);
        // let ranges = &self.ranges[partition..];
        if let Some(entry) = self
            .ranges
            .iter()
            .filter(|&map| map.source.start <= source)
            .filter(|&map| map.source.end > source)
            .find_map(|map| map.map(source))
        {
            entry
        } else {
            // "Any source numbers that aren't mapped correspond to the same destination number."
            Destination::from(source.into())
        }
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
                plugs.push(MapRange {
                    source: Source::from(next_start)..Source::from(range_start),
                    destination: Destination::from(next_start)
                        ..Destination::from(next_start) + (length as usize),
                    length: length as _,
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
        let last_range_start = (ranges[ranges.len() - 1].source.end).into();
        ranges.push(MapRange {
            source: Source::from(last_range_start)..Source::from(u32::MAX),
            destination: Destination::from(next_start)..Destination::from(u32::MAX),
            length: (u32::MAX - last_range_start) as usize,
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

        Ok(Almanac {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        })
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
        assert_eq!(almanac.seed_to_soil.len(), 4);
        assert_eq!(almanac.soil_to_fertilizer.len(), 4);
        assert_eq!(almanac.fertilizer_to_water.len(), 5);
        assert_eq!(almanac.water_to_light.len(), 4);
        assert_eq!(almanac.light_to_temperature.len(), 5);
        assert_eq!(almanac.temperature_to_humidity.len(), 3);
        assert_eq!(almanac.humidity_to_location.len(), 4);

        assert_eq!(almanac.map_seed(Seed(79)), Location(82));
        assert_eq!(almanac.map_seed(Seed(14)), Location(43));
        assert_eq!(almanac.map_seed(Seed(55)), Location(86));
        assert_eq!(almanac.map_seed(Seed(13)), Location(35));
    }
}
