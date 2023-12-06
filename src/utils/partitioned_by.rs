use std::ops::Range;

pub struct Partitions<T> {
    pub lower: Option<T>,
    pub middle: Option<T>,
    pub upper: Option<T>,
}

pub trait PartitionedBy<T = Self>: Sized {
    fn partitioned_by(&self, other: &T) -> Partitions<Self>;
}

impl<T: PartialOrd + Copy> PartitionedBy for Range<T> {
    fn partitioned_by(&self, other: &Self) -> Partitions<Self> {
        // Other range is entirely larger than this one
        if other.start >= self.end {
            return Partitions {
                lower: Some(self.clone()),
                middle: None,
                upper: None,
            };
        }
        // Other range is entirely less than this one
        if other.end <= self.start {
            return Partitions {
                lower: None,
                middle: None,
                upper: Some(self.clone()),
            };
        }

        // This range entirely contains the other one
        if other.start >= self.start && other.end <= self.end {
            return Partitions {
                lower: Some(self.start..other.start),
                middle: Some(other.clone()),
                upper: Some(other.end..self.end),
            };
        }

        // This range is entirely contained in this one
        if self.start >= other.start && self.end <= other.end {
            return Partitions {
                lower: None,
                middle: Some(self.clone()),
                upper: None,
            };
        }

        // Other range contains the start of this one
        if other.contains(&self.start) {
            return Partitions {
                lower: None,
                middle: Some(self.start..other.end),
                upper: Some(other.end..self.end),
            };
        }

        Partitions {
            lower: Some(self.start..other.start),
            middle: Some(other.start..self.end),
            upper: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn self_fully_contained() {
        let s = 5..7;
        let o = 1..10;
        let partitions = s.partitioned_by(&o);
        assert_eq!(partitions.lower, None);
        assert_eq!(partitions.middle, Some(s));
        assert_eq!(partitions.upper, None);
    }

    #[test]
    fn other_fully_contained() {
        let s = 1..10;
        let o = 5..7;
        let partitions = s.partitioned_by(&o);
        assert_eq!(partitions.lower, Some(1..5));
        assert_eq!(partitions.middle, Some(5..7));
        assert_eq!(partitions.upper, Some(7..10));
    }

    #[test]
    fn self_entirely_smaller() {
        let s = 1..5;
        let o = 6..10;
        let partitions = s.partitioned_by(&o);
        assert_eq!(partitions.lower, Some(s));
        assert_eq!(partitions.middle, None);
        assert_eq!(partitions.upper, None);
    }

    #[test]
    fn self_entirely_larger() {
        let s = 6..10;
        let o = 1..5;
        let partitions = s.partitioned_by(&o);
        assert_eq!(partitions.lower, None);
        assert_eq!(partitions.middle, None);
        assert_eq!(partitions.upper, Some(s));
    }

    #[test]
    fn self_upper_half_intersects() {
        let s = 1..10;
        let o = 5..15;
        let partitions = s.partitioned_by(&o);
        assert_eq!(partitions.lower, Some(1..5));
        assert_eq!(partitions.middle, Some(5..10));
        assert_eq!(partitions.upper, None);
    }

    #[test]
    fn self_lower_half_intersects() {
        let s = 5..15;
        let o = 1..10;
        let partitions = s.partitioned_by(&o);
        assert_eq!(partitions.lower, None);
        assert_eq!(partitions.middle, Some(5..10));
        assert_eq!(partitions.upper, Some(10..15));
    }

    #[test]
    fn end_corner_case() {
        let s = 1..5;
        let o = 5..10;
        let partitions = s.partitioned_by(&o);
        assert_eq!(partitions.lower, Some(s));
        assert_eq!(partitions.middle, None);
        assert_eq!(partitions.upper, None);
    }

    #[test]
    fn start_corner_case() {
        let s = 5..10;
        let o = 1..5;
        let partitions = s.partitioned_by(&o);
        assert_eq!(partitions.lower, None);
        assert_eq!(partitions.middle, None);
        assert_eq!(partitions.upper, Some(s));
    }
}
