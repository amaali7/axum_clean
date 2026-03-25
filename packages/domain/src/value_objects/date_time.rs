#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Weekday {
    Saturday = 0,
    Sunday = 1,
    Monday = 2,
    Tuesday = 3,
    Wednesday = 4,
    Thursday = 5,
    Friday = 6,
}

impl Weekday {
    pub fn number(&self) -> u8 {
        match self {
            Weekday::Saturday => 6,
            Weekday::Sunday => 0,
            Weekday::Monday => 1,
            Weekday::Tuesday => 2,
            Weekday::Wednesday => 3,
            Weekday::Thursday => 4,
            Weekday::Friday => 5,
        }
    }
    pub fn all() -> [Weekday; 7] {
        [
            Weekday::Saturday,
            Weekday::Sunday,
            Weekday::Monday,
            Weekday::Tuesday,
            Weekday::Wednesday,
            Weekday::Thursday,
            Weekday::Friday,
        ]
    }
}

#[derive(Debug, Clone, Copy, Hash, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTime(i64);

impl DateTime {
    pub fn new(datetime: i64) -> Self {
        Self(datetime)
    }

    pub fn datetime(&self) -> i64 {
        self.0
    }
    pub fn is_before(&self, other: &DateTime) -> bool {
        self.0 < other.0
    }

    pub fn is_after(&self, other: &DateTime) -> bool {
        self.0 > other.0
    }

    pub fn between(&self, start: &DateTime, end: &DateTime) -> bool {
        self.0 >= start.0 && self.0 <= end.0
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
