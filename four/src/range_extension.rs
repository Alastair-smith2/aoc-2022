use std::ops::RangeInclusive;

// See https://fasterthanli.me/series/advent-of-code-2022/part-4 for more detail on extensions and the overlap example!
// Simpler solution than what's currently in main.
pub trait InclusiveRangeOverlap {
    fn contains_range(&self, other: &Self) -> bool;

    fn contains_or_is_contained(&self, other: &Self) -> bool {
        self.contains_range(other) || other.contains_range(self)
    }
}

impl<T> InclusiveRangeOverlap for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }
}
