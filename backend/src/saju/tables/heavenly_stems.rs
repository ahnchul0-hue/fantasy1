//! 천간 (Heavenly Stems) - 10 Celestial Stems with 오행 (Five Elements) mapping
//!
//! The 10 Heavenly Stems (천간/天干) are fundamental to the Four Pillars system.
//! Each stem maps to one of the Five Elements (오행) with a Yin/Yang polarity.

use std::fmt;

/// The Five Elements (오행/五行)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Element {
    Wood,  // 목(木)
    Fire,  // 화(火)
    Earth, // 토(土)
    Metal, // 금(金)
    Water, // 수(水)
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::Wood => write!(f, "목(木)"),
            Element::Fire => write!(f, "화(火)"),
            Element::Earth => write!(f, "토(土)"),
            Element::Metal => write!(f, "금(金)"),
            Element::Water => write!(f, "수(水)"),
        }
    }
}

/// Yin-Yang polarity (음양)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Polarity {
    Yang, // 양(陽)
    Yin,  // 음(陰)
}

impl fmt::Display for Polarity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Polarity::Yang => write!(f, "양(陽)"),
            Polarity::Yin => write!(f, "음(陰)"),
        }
    }
}

/// A single Heavenly Stem (천간)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HeavenlyStem {
    pub index: u8,
    pub korean: &'static str,
    pub hanja: char,
    pub element: Element,
    pub polarity: Polarity,
}

impl fmt::Display for HeavenlyStem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.korean, self.hanja)
    }
}

/// All 10 Heavenly Stems in canonical order
pub static HEAVENLY_STEMS: [HeavenlyStem; 10] = [
    HeavenlyStem { index: 0, korean: "갑", hanja: '甲', element: Element::Wood,  polarity: Polarity::Yang },
    HeavenlyStem { index: 1, korean: "을", hanja: '乙', element: Element::Wood,  polarity: Polarity::Yin  },
    HeavenlyStem { index: 2, korean: "병", hanja: '丙', element: Element::Fire,  polarity: Polarity::Yang },
    HeavenlyStem { index: 3, korean: "정", hanja: '丁', element: Element::Fire,  polarity: Polarity::Yin  },
    HeavenlyStem { index: 4, korean: "무", hanja: '戊', element: Element::Earth, polarity: Polarity::Yang },
    HeavenlyStem { index: 5, korean: "기", hanja: '己', element: Element::Earth, polarity: Polarity::Yin  },
    HeavenlyStem { index: 6, korean: "경", hanja: '庚', element: Element::Metal, polarity: Polarity::Yang },
    HeavenlyStem { index: 7, korean: "신", hanja: '辛', element: Element::Metal, polarity: Polarity::Yin  },
    HeavenlyStem { index: 8, korean: "임", hanja: '壬', element: Element::Water, polarity: Polarity::Yang },
    HeavenlyStem { index: 9, korean: "계", hanja: '癸', element: Element::Water, polarity: Polarity::Yin  },
];

/// Look up a Heavenly Stem by its 0-based index
pub fn stem_by_index(index: u8) -> Option<&'static HeavenlyStem> {
    HEAVENLY_STEMS.get(index as usize)
}

/// Look up a Heavenly Stem by its Korean name
pub fn stem_by_korean(name: &str) -> Option<&'static HeavenlyStem> {
    HEAVENLY_STEMS.iter().find(|s| s.korean == name)
}

/// Look up a Heavenly Stem by its Hanja character
pub fn stem_by_hanja(hanja: char) -> Option<&'static HeavenlyStem> {
    HEAVENLY_STEMS.iter().find(|s| s.hanja == hanja)
}

/// Get the Heavenly Stem for a given year.
/// Formula: (year - 4) % 10 maps to the stem index.
/// (Year 4 CE = 甲子年, the start of the sexagenary cycle.)
pub fn stem_for_year(year: i32) -> &'static HeavenlyStem {
    let index = ((year - 4) % 10 + 10) % 10;
    &HEAVENLY_STEMS[index as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stem_count() {
        assert_eq!(HEAVENLY_STEMS.len(), 10);
    }

    #[test]
    fn test_element_mapping() {
        assert_eq!(HEAVENLY_STEMS[0].element, Element::Wood);
        assert_eq!(HEAVENLY_STEMS[2].element, Element::Fire);
        assert_eq!(HEAVENLY_STEMS[4].element, Element::Earth);
        assert_eq!(HEAVENLY_STEMS[6].element, Element::Metal);
        assert_eq!(HEAVENLY_STEMS[8].element, Element::Water);
    }

    #[test]
    fn test_polarity_alternates() {
        for (i, stem) in HEAVENLY_STEMS.iter().enumerate() {
            if i % 2 == 0 {
                assert_eq!(stem.polarity, Polarity::Yang);
            } else {
                assert_eq!(stem.polarity, Polarity::Yin);
            }
        }
    }

    #[test]
    fn test_lookup_by_korean() {
        let stem = stem_by_korean("갑").unwrap();
        assert_eq!(stem.hanja, '甲');
        assert_eq!(stem.element, Element::Wood);
    }

    #[test]
    fn test_stem_for_year_2024() {
        // 2024 = 甲辰年
        let stem = stem_for_year(2024);
        assert_eq!(stem.korean, "갑");
        assert_eq!(stem.hanja, '甲');
    }
}
