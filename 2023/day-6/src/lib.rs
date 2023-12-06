use aoc_utils::parse_whitespace_delimited;
use std::ops::RangeInclusive;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct RaceDuration(u64);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct ChargeTime(u64);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct BoatDistance(u64);

/// Solution to part 1.
pub fn product_of_winning_conditions(input: &str) -> u64 {
    let mut lines = input.lines();

    let first_line = lines.next().expect("input is empty");
    if &first_line[..5] != "Time:" {
        panic!("Invalid input: Missing time")
    }
    let first_line = first_line[5..].trim();
    let times: Vec<u64> = parse_whitespace_delimited(first_line).expect("unable to parse times");

    let second_line = lines.next().expect("input is toos hort");
    if &second_line[..9] != "Distance:" {
        panic!("Invalid input: Missing distnances")
    }
    let second_line = second_line[9..].trim();
    let distances: Vec<u64> =
        parse_whitespace_delimited(second_line).expect("unable to parse distances");

    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| (RaceDuration(time), BoatDistance(distance)))
        .map(|(time, distance)| num_winning_conditions(time, distance))
        .product()
}

/// Determines the number of winning conditions.
fn num_winning_conditions(race_duration: RaceDuration, best_distance: BoatDistance) -> u64 {
    let range =
        winning_condition(race_duration, best_distance).expect("found no winning conditions");

    // The number of winnings conditions is the range length (plus one since the end is inclusive).
    range.end().0 - range.start().0 + 1
}

/// Checks for the winning condition based on race duration and best distance.
///
/// Unlike [`winning_condition_bf`], this function uses analysis to obtain the conditions directly.
/// Uses [`find_quadratic_roots`] to solve the quadratic equation.
///
/// # Arguments
///
/// * `race_duration` - The duration of the race.
/// * `best_distance` - The best distance achieved by the boat.
///
/// # Returns
///
/// An `Option` containing the range of `ChargeTime` values that satisfy the winning condition.
/// If no range is found, `None` is returned.
fn winning_condition(
    RaceDuration(race_duration): RaceDuration,
    BoatDistance(best_distance): BoatDistance,
) -> Option<RangeInclusive<ChargeTime>> {
    // Find the winning conditions using analysis. We add `0.5` to the best distance to account
    // for the fact that we want to exclude the winning condition itself; since the actual solutions
    // are integer, adding `0.5` gives us some wiggle room.
    let (first, second) = find_quadratic_roots(race_duration as _, 0.5 + best_distance as f64);
    if first.is_nan() || second.is_nan() {
        return None;
    }

    // Ensure integral solutions. The start must be larger than the best winning conditions,
    // the end must be less than the best winning condition.
    let start = ChargeTime(first.ceil() as u64);
    let end = ChargeTime(second.floor() as u64);
    Some(start..=end)
}

/// Calculates the distance a boat can travel during a race.
///
/// # Arguments
///
/// * `charge_time` - The time it takes to charge the boat's batteries.
/// * `race_duration` - The total duration of the race.
///
/// # Returns
///
/// The distance the boat can travel during the race.
#[cfg(test)]
fn boat_distance(
    ChargeTime(charge_time): ChargeTime,
    RaceDuration(race_duration): RaceDuration,
) -> BoatDistance {
    debug_assert!(charge_time <= race_duration);
    // Charging takes time during the race.
    let travel_time = race_duration - charge_time;
    // The travel speed is equal to the charge time.
    let travel_speed = charge_time;
    // Travel distance is trivial then.
    BoatDistance(travel_speed * travel_time)
}

/// Checks for the winning condition based on race duration and best distance.
///
/// This is a brute-force implementation of the winning condition and it does not account
/// for the following observations:
///
/// - The curve of distances traveled is symmetric. If the input range is scanned, only
///   half of the range needs to be scanned to begin with.
/// - The curve can be described mathematically, no search is required.
///
/// # Arguments
///
/// * `race_duration` - The duration of the race.
/// * `best_distance` - The best distance achieved by the boat.
///
/// # Returns
///
/// An `Option` containing the range of `ChargeTime` values that satisfy the winning condition.
/// If no range is found, `None` is returned.
#[cfg(test)]
fn winning_condition_bf(
    race_duration: RaceDuration,
    best_distance: BoatDistance,
) -> Option<RangeInclusive<ChargeTime>> {
    debug_assert!(race_duration.0 > 0);

    // Find the first winning condition.
    let start_condition = (1..race_duration.0)
        .map(ChargeTime)
        .map(|t| (t, boat_distance(t, race_duration)))
        .filter(|(_, d)| *d > best_distance)
        .map(|(t, _)| t)
        .next();
    let start_condition = match start_condition {
        None => return None,
        Some(duration) => duration,
    };

    // Find the first non-winning condition after the known start condition.
    // When a non-winning condition is found, the time before that must be the last
    // winning condition. If none is found, the time just before the race ends is taken.
    // Note that the boat's button cannot be held for the entire duration of the race
    // is the boat would then have no time to move.
    let end_condition = (start_condition.0 + 1..race_duration.0)
        .map(ChargeTime)
        .map(|t| (t, boat_distance(t, race_duration)))
        .filter(|(_, d)| *d <= best_distance)
        .map(|(t, _)| ChargeTime(t.0 - 1))
        .next()
        .unwrap_or(ChargeTime(race_duration.0 - 1));

    debug_assert!(start_condition <= end_condition);
    Some(start_condition..=end_condition)
}

/// Finds zero crossings for the quadratic formula `f(c, d, b) = -c^2 + dc + b` where
/// - `c` is our charge time,
/// - `d` is the race duration and
/// - `b` is the best game we want to beat.
fn find_quadratic_roots(duration: f64, best: f64) -> (f64, f64) {
    let discriminant = duration.powi(2) + 4.0 * (-best);
    if discriminant >= 0.0 {
        let root1 = (duration - discriminant.sqrt()) / 2.0;
        let root2 = (duration + discriminant.sqrt()) / 2.0;

        debug_assert!(root1 < root2);
        (root1, root2)
    } else {
        (f64::NAN, f64::NAN)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boat_distance() {
        assert_eq!(
            boat_distance(ChargeTime(0), RaceDuration(7)),
            BoatDistance(0)
        );
        assert_eq!(
            boat_distance(ChargeTime(1), RaceDuration(7)),
            BoatDistance(6)
        );
        assert_eq!(
            boat_distance(ChargeTime(2), RaceDuration(7)),
            BoatDistance(10)
        );
        assert_eq!(
            boat_distance(ChargeTime(3), RaceDuration(7)),
            BoatDistance(12)
        );
        assert_eq!(
            boat_distance(ChargeTime(4), RaceDuration(7)),
            BoatDistance(12)
        );
        assert_eq!(
            boat_distance(ChargeTime(5), RaceDuration(7)),
            BoatDistance(10)
        );
        assert_eq!(
            boat_distance(ChargeTime(6), RaceDuration(7)),
            BoatDistance(6)
        );
        assert_eq!(
            boat_distance(ChargeTime(7), RaceDuration(7)),
            BoatDistance(0)
        );
    }

    #[test]
    fn test_winning_condition_brute_force() {
        assert_eq!(
            winning_condition_bf(RaceDuration(7), BoatDistance(9)),
            Some(ChargeTime(2)..=ChargeTime(5))
        );
        assert_eq!(
            winning_condition_bf(RaceDuration(15), BoatDistance(40)),
            Some(ChargeTime(4)..=ChargeTime(11))
        );
        assert_eq!(
            winning_condition_bf(RaceDuration(30), BoatDistance(200)),
            Some(ChargeTime(11)..=ChargeTime(19))
        );
    }

    #[test]
    fn test_winning_condition() {
        let _result = find_quadratic_roots(7.0, 9.0);

        assert_eq!(
            winning_condition(RaceDuration(7), BoatDistance(9)),
            Some(ChargeTime(2)..=ChargeTime(5))
        );
        assert_eq!(
            winning_condition(RaceDuration(15), BoatDistance(40)),
            Some(ChargeTime(4)..=ChargeTime(11))
        );
        assert_eq!(
            winning_condition(RaceDuration(30), BoatDistance(200)),
            Some(ChargeTime(11)..=ChargeTime(19))
        );
    }

    #[test]
    fn test_num_winning_conditions() {
        assert_eq!(num_winning_conditions(RaceDuration(7), BoatDistance(9)), 4);
        assert_eq!(
            num_winning_conditions(RaceDuration(15), BoatDistance(40)),
            8
        );
        assert_eq!(
            num_winning_conditions(RaceDuration(30), BoatDistance(200)),
            9
        );
    }
}
