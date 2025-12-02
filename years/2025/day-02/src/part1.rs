#[derive(PartialEq, Debug)]
pub struct IdRange {
    pub start: u64,
    pub end: u64,
}

impl IdRange {
    pub fn contains(&self, id: u64) -> bool {
        id >= self.start && id <= self.end
    }
}

pub fn parse_range(input: &str) -> Vec<IdRange> {
    input
        .replace('\n', "")
        .split(',')
        .filter_map(|s| {
            let parts: Vec<&str> = s.trim().split('-').collect();
            if parts.len() == 2 {
                let start = parts[0].parse::<u64>().ok()?;
                let end = parts[1].parse::<u64>().ok()?;
                Some(IdRange { start, end })
            } else {
                None
            }
        })
        .collect()
}

fn generate_mirrored_id(seed: u64) -> Option<u64> {
    let s = seed.to_string();
    format!("{}{}", s, s).parse::<u64>().ok()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let ranges = parse_range(input);

    if ranges.is_empty() {
        return Ok("0".to_string());
    }

    let max_bound = ranges.iter().map(|r| r.end).max().unwrap_or(0);
    let mut total_sum: u64 = 0;
    let mut seed: u64 = 1;

    while let Some(id) = generate_mirrored_id(seed) {
        let candidate = id;

        if candidate > max_bound {
            break;
        }

        if ranges.iter().any(|range| range.contains(candidate)) {
            total_sum += candidate;
        }

        seed += 1;
    }

    Ok(total_sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_range() {
        let input = "11-22";
        assert_eq!(vec![IdRange { start: 11, end: 22 }], parse_range(input));
    }

    #[test]
    fn test_parse_multiple_range() {
        let input = "11-22,95-115";
        assert_eq!(
            vec![
                IdRange { start: 11, end: 22 },
                IdRange {
                    start: 95,
                    end: 115
                }
            ],
            parse_range(input)
        );
    }

    #[test]
    fn test_generate_mirror() {
        assert_eq!(Some(6464), generate_mirrored_id(64));
        assert_eq!(Some(55), generate_mirrored_id(5));
        assert_eq!(Some(1212), generate_mirrored_id(12));
        assert_eq!(Some(123123), generate_mirrored_id(123));
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
        assert_eq!("1227775554", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_empty_input() -> miette::Result<()> {
        let input = "";
        assert_eq!("0", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_no_matches() -> miette::Result<()> {
        let input = "1-2";
        assert_eq!("0", process(input)?);
        Ok(())
    }

    #[test]
    fn test_generate_mirror_overflow() {
        // Test that overflow is handled gracefully
        // u64::MAX is 18_446_744_073_709_551_615
        // Seeds >= 4_294_967_296 will cause overflow when mirrored
        let large_seed = 10_000_000_000_u64;
        assert_eq!(None, generate_mirrored_id(large_seed));
    }

    #[test]
    fn test_id_range_contains() {
        let range = IdRange { start: 10, end: 20 };
        assert!(range.contains(10));
        assert!(range.contains(15));
        assert!(range.contains(20));
        assert!(!range.contains(9));
        assert!(!range.contains(21));
    }
}
