use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum PlaceFields {
    ContainedWithin,
    Country,
    CountryCode,
    FullName,
    Geo,
    Id,
    Name,
    PlaceType,
}

impl PlaceFields {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::ContainedWithin);
        result.insert(Self::Country);
        result.insert(Self::CountryCode);
        result.insert(Self::FullName);
        result.insert(Self::Geo);
        result.insert(Self::Id);
        result.insert(Self::Name);
        result.insert(Self::PlaceType);
        result
    }
}

impl std::fmt::Display for PlaceFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ContainedWithin => write!(f, "contained_within"),
            Self::Country => write!(f, "country"),
            Self::CountryCode => write!(f, "country_code"),
            Self::FullName => write!(f, "full_name"),
            Self::Geo => write!(f, "geo"),
            Self::Id => write!(f, "id"),
            Self::Name => write!(f, "name"),
            Self::PlaceType => write!(f, "place_type"),
        }
    }
}

impl Default for PlaceFields {
    fn default() -> Self {
        Self::ContainedWithin
    }
}
