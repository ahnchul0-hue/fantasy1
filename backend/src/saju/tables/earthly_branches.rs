//! 지지 (Earthly Branches) - 12 Terrestrial Branches with 오행 and 시진 mapping
//!
//! The 12 Earthly Branches (지지/地支) represent the 12 two-hour periods of the day,
//! the 12 months, and the 12-year cycle. Each branch maps to an element and a time period.

use super::heavenly_stems::{Element, Polarity};
use std::fmt;

/// A single Earthly Branch (지지)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EarthlyBranch {
    pub index: u8,
    pub korean: &'static str,
    pub hanja: char,
    pub animal: &'static str,
    pub animal_korean: &'static str,
    pub element: Element,
    pub polarity: Polarity,
    /// Start hour of the 시진 (two-hour period), in 24h format
    pub hour_start: u8,
    /// End hour of the 시진
    pub hour_end: u8,
    /// Romanized name for API use
    pub romanized: &'static str,
}

impl fmt::Display for EarthlyBranch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.korean, self.hanja)
    }
}

/// All 12 Earthly Branches in canonical order
pub static EARTHLY_BRANCHES: [EarthlyBranch; 12] = [
    EarthlyBranch {
        index: 0, korean: "자", hanja: '子', animal: "Rat", animal_korean: "쥐",
        element: Element::Water, polarity: Polarity::Yang,
        hour_start: 23, hour_end: 1, romanized: "ja",
    },
    EarthlyBranch {
        index: 1, korean: "축", hanja: '丑', animal: "Ox", animal_korean: "소",
        element: Element::Earth, polarity: Polarity::Yin,
        hour_start: 1, hour_end: 3, romanized: "chuk",
    },
    EarthlyBranch {
        index: 2, korean: "인", hanja: '寅', animal: "Tiger", animal_korean: "호랑이",
        element: Element::Wood, polarity: Polarity::Yang,
        hour_start: 3, hour_end: 5, romanized: "in",
    },
    EarthlyBranch {
        index: 3, korean: "묘", hanja: '卯', animal: "Rabbit", animal_korean: "토끼",
        element: Element::Wood, polarity: Polarity::Yin,
        hour_start: 5, hour_end: 7, romanized: "myo",
    },
    EarthlyBranch {
        index: 4, korean: "진", hanja: '辰', animal: "Dragon", animal_korean: "용",
        element: Element::Earth, polarity: Polarity::Yang,
        hour_start: 7, hour_end: 9, romanized: "jin",
    },
    EarthlyBranch {
        index: 5, korean: "사", hanja: '巳', animal: "Snake", animal_korean: "뱀",
        element: Element::Fire, polarity: Polarity::Yin,
        hour_start: 9, hour_end: 11, romanized: "sa",
    },
    EarthlyBranch {
        index: 6, korean: "오", hanja: '午', animal: "Horse", animal_korean: "말",
        element: Element::Fire, polarity: Polarity::Yang,
        hour_start: 11, hour_end: 13, romanized: "o",
    },
    EarthlyBranch {
        index: 7, korean: "미", hanja: '未', animal: "Goat", animal_korean: "양",
        element: Element::Earth, polarity: Polarity::Yin,
        hour_start: 13, hour_end: 15, romanized: "mi",
    },
    EarthlyBranch {
        index: 8, korean: "신", hanja: '申', animal: "Monkey", animal_korean: "원숭이",
        element: Element::Metal, polarity: Polarity::Yang,
        hour_start: 15, hour_end: 17, romanized: "sin",
    },
    EarthlyBranch {
        index: 9, korean: "유", hanja: '酉', animal: "Rooster", animal_korean: "닭",
        element: Element::Metal, polarity: Polarity::Yin,
        hour_start: 17, hour_end: 19, romanized: "yu",
    },
    EarthlyBranch {
        index: 10, korean: "술", hanja: '戌', animal: "Dog", animal_korean: "개",
        element: Element::Earth, polarity: Polarity::Yang,
        hour_start: 19, hour_end: 21, romanized: "sul",
    },
    EarthlyBranch {
        index: 11, korean: "해", hanja: '亥', animal: "Pig", animal_korean: "돼지",
        element: Element::Water, polarity: Polarity::Yin,
        hour_start: 21, hour_end: 23, romanized: "hae",
    },
];

/// Look up an Earthly Branch by its 0-based index
pub fn branch_by_index(index: u8) -> Option<&'static EarthlyBranch> {
    EARTHLY_BRANCHES.get(index as usize)
}

/// Look up an Earthly Branch by its Korean name
pub fn branch_by_korean(name: &str) -> Option<&'static EarthlyBranch> {
    EARTHLY_BRANCHES.iter().find(|b| b.korean == name)
}

/// Look up an Earthly Branch by its Hanja character
pub fn branch_by_hanja(hanja: char) -> Option<&'static EarthlyBranch> {
    EARTHLY_BRANCHES.iter().find(|b| b.hanja == hanja)
}

/// Look up an Earthly Branch by its romanized API name (e.g., "ja", "chuk")
pub fn branch_by_romanized(name: &str) -> Option<&'static EarthlyBranch> {
    EARTHLY_BRANCHES.iter().find(|b| b.romanized == name)
}

/// Get the Earthly Branch for a given year.
/// Formula: (year - 4) % 12 maps to the branch index.
pub fn branch_for_year(year: i32) -> &'static EarthlyBranch {
    let index = ((year - 4) % 12 + 12) % 12;
    &EARTHLY_BRANCHES[index as usize]
}

/// Get the Earthly Branch for a given hour (0-23).
/// 子時 spans 23:00-01:00, so hours 23 and 0 both map to 子.
pub fn branch_for_hour(hour: u8) -> &'static EarthlyBranch {
    let index = match hour {
        23 | 0 => 0,
        1 | 2 => 1,
        3 | 4 => 2,
        5 | 6 => 3,
        7 | 8 => 4,
        9 | 10 => 5,
        11 | 12 => 6,
        13 | 14 => 7,
        15 | 16 => 8,
        17 | 18 => 9,
        19 | 20 => 10,
        21 | 22 => 11,
        _ => 0, // Invalid hour defaults to 子
    };
    &EARTHLY_BRANCHES[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_branch_count() {
        assert_eq!(EARTHLY_BRANCHES.len(), 12);
    }

    #[test]
    fn test_zodiac_cycle() {
        // 2024 = 甲辰年 (Dragon)
        let branch = branch_for_year(2024);
        assert_eq!(branch.korean, "진");
        assert_eq!(branch.animal, "Dragon");
    }

    #[test]
    fn test_hour_mapping() {
        assert_eq!(branch_for_hour(23).korean, "자");
        assert_eq!(branch_for_hour(0).korean, "자");
        assert_eq!(branch_for_hour(12).korean, "오");
    }

    #[test]
    fn test_romanized_lookup() {
        let branch = branch_by_romanized("hae").unwrap();
        assert_eq!(branch.korean, "해");
        assert_eq!(branch.animal, "Pig");
    }

    #[test]
    fn test_all_romanized_names_match_api_schema() {
        // These must match the BirthInput.birth_hour enum in api-schema.json
        let api_values = [
            "ja", "chuk", "in", "myo", "jin", "sa",
            "o", "mi", "sin", "yu", "sul", "hae",
        ];
        for (i, name) in api_values.iter().enumerate() {
            assert_eq!(EARTHLY_BRANCHES[i].romanized, *name);
        }
    }
}
