//! 만세력 (Manseryeok) Lookup Tables
//!
//! Static data tables for the saju (四柱) calculation engine.
//! These tables encode the fundamental building blocks of Korean fortune-telling:
//!
//! - **Heavenly Stems** (천간/天干): 10 celestial stems with 오행 mapping
//! - **Earthly Branches** (지지/地支): 12 terrestrial branches with 오행/시진 mapping
//! - **Solar Terms** (절기/節氣): 24 solar term transition times (2020-2030, with mathematical fallback)
//! - **Lunar Calendar** (음력): Solar-lunar date conversion tables (2020-2030)

pub mod earthly_branches;
pub mod heavenly_stems;
pub mod lunar_calendar;
pub mod solar_terms;

// Re-export commonly used types and constants
pub use earthly_branches::{
    branch_by_index, branch_by_korean, branch_by_romanized, branch_for_hour, branch_for_year,
    EarthlyBranch, EARTHLY_BRANCHES,
};
pub use heavenly_stems::{
    stem_by_index, stem_by_korean, stem_for_year, Element, HeavenlyStem, Polarity,
    HEAVENLY_STEMS,
};
pub use lunar_calendar::{
    get_lunar_year, lunar_to_solar, solar_to_lunar, LunarYearData, LUNAR_YEARS,
};
pub use solar_terms::{find_active_jie_term, jie_term_to_saju_month, JIE_TERMS};

/// Get the 60 甲子 (Sexagenary cycle) index for a given Heavenly Stem and Earthly Branch pair.
/// Returns None if the pair has mismatched polarity (both must be Yang or both Yin).
pub fn sexagenary_index(stem_idx: u8, branch_idx: u8) -> Option<u8> {
    if stem_idx >= 10 || branch_idx >= 12 {
        return None;
    }
    // Stem and branch must have same polarity (both even or both odd index)
    if stem_idx % 2 != branch_idx % 2 {
        return None;
    }
    // The sexagenary index = (stem_idx * 6 + branch_idx / 2) for matching polarity pairs
    // Simpler: iterate until we find the match
    Some(((stem_idx as u16 * 12 + branch_idx as u16) * 5 % 60) as u8)
}

/// Get the ilju (일주/日柱) string for a given sexagenary day index.
/// Returns a 2-character Korean string like "갑자", "을축", etc.
pub fn ilju_from_day_index(day_index: u8) -> String {
    let stem_idx = day_index % 10;
    let branch_idx = day_index % 12;
    format!(
        "{}{}",
        HEAVENLY_STEMS[stem_idx as usize].korean,
        EARTHLY_BRANCHES[branch_idx as usize].korean,
    )
}

// ========================================
// Pillar Calculation Functions
// ========================================

/// A pillar (기둥) consisting of a Heavenly Stem and Earthly Branch pair.
#[derive(Debug, Clone, Copy)]
pub struct Pillar {
    pub stem_index: u8,
    pub branch_index: u8,
}

impl Pillar {
    pub fn new(stem_index: u8, branch_index: u8) -> Self {
        Self {
            stem_index: stem_index % 10,
            branch_index: branch_index % 12,
        }
    }

    pub fn stem(&self) -> &'static HeavenlyStem {
        &HEAVENLY_STEMS[self.stem_index as usize]
    }

    pub fn branch(&self) -> &'static EarthlyBranch {
        &EARTHLY_BRANCHES[self.branch_index as usize]
    }

    /// Korean display string (e.g., "갑자")
    pub fn korean(&self) -> String {
        format!("{}{}", self.stem().korean, self.branch().korean)
    }

    /// Hanja display string (e.g., "甲子")
    pub fn hanja(&self) -> String {
        format!("{}{}", self.stem().hanja, self.branch().hanja)
    }
}

/// Calculate year pillar.
/// Base: 1984 = 갑자년 (stem 0, branch 0).
/// The Chinese calendar year starts at 입춘 (Start of Spring).
/// Before 입춘, the previous year's pillar is used.
pub fn year_pillar(year: i32, before_ipchun: bool) -> Pillar {
    let effective_year = if before_ipchun { year - 1 } else { year };
    let stem_index = ((effective_year - 4) % 10 + 10) % 10;
    let branch_index = ((effective_year - 4) % 12 + 12) % 12;
    Pillar::new(stem_index as u8, branch_index as u8)
}

/// Calculate month pillar.
/// `year_stem_index`: index of the year's heavenly stem (0-9)
/// `saju_month`: saju month number (1=인월 through 12=축월)
pub fn month_pillar(year_stem_index: u8, saju_month: u8) -> Pillar {
    // Earthly branch: 인(2) for month 1, 묘(3) for month 2, ..., 축(1) for month 12
    let branch_index = ((saju_month as u16 + 1) % 12) as u8;

    // Heavenly stem: determined by the year stem
    // 갑/기년→병인월(stem 2), 을/경→무인(4), 병/신→경인(6), 정/임→임인(8), 무/계→갑인(0)
    let base_stem = match year_stem_index % 5 {
        0 => 2u8, // 갑/기
        1 => 4,   // 을/경
        2 => 6,   // 병/신
        3 => 8,   // 정/임
        4 => 0,   // 무/계
        _ => unreachable!(),
    };
    let stem_index = (base_stem + saju_month - 1) % 10;

    Pillar::new(stem_index, branch_index)
}

/// Calculate day pillar from Julian Day Number.
/// Reference: JDN 2451545 (2000-01-01) = 갑진일 (stem 0, branch 4) → cycle index 40
pub fn day_pillar_from_jdn(jdn: i64) -> Pillar {
    let reference_jdn: i64 = 2451545;
    // 2000-01-01 = 甲辰 → stem 0, branch 4
    let stem_offset = ((jdn - reference_jdn) % 10 + 10) % 10;
    let branch_offset = ((jdn - reference_jdn) % 12 + 12) % 12;
    let stem_index = (stem_offset as u8) % 10; // 갑=0 at reference
    let branch_index = ((branch_offset as u8) + 4) % 12; // 辰=4 at reference
    Pillar::new(stem_index, branch_index)
}

/// Calculate hour pillar.
/// `day_stem_index`: index of the day's heavenly stem (0-9)
/// `hour_branch_index`: index of the 시진 (0=자 through 11=해)
pub fn hour_pillar(day_stem_index: u8, hour_branch_index: u8) -> Pillar {
    // 갑/기일→갑자시(0), 을/경→병자(2), 병/신→무자(4), 정/임→경자(6), 무/계→임자(8)
    let base_stem = (day_stem_index % 5) * 2;
    let stem_index = (base_stem + hour_branch_index) % 10;
    Pillar::new(stem_index, hour_branch_index)
}

/// Convert a solar date to Julian Day Number (Meeus algorithm).
pub fn solar_to_jdn(year: i32, month: u32, day: u32) -> i64 {
    let y = if month <= 2 { year - 1 } else { year } as i64;
    let m = if month <= 2 { month + 12 } else { month } as i64;
    let d = day as i64;
    let a = y / 100;
    let b = 2 - a + a / 4;
    (365.25 * (y + 4716) as f64).floor() as i64
        + (30.6001 * (m + 1) as f64).floor() as i64
        + d + b - 1524
}

/// Convert Julian Day Number to solar (Gregorian) date.
pub fn jdn_to_solar(jdn: i64) -> (i32, u32, u32) {
    let a = jdn + 32044;
    let b = (4 * a + 3) / 146097;
    let c = a - (146097 * b) / 4;
    let d = (4 * c + 3) / 1461;
    let e = c - (1461 * d) / 4;
    let m = (5 * e + 2) / 153;
    let day = (e - (153 * m + 2) / 5 + 1) as u32;
    let month = (m + 3 - 12 * (m / 10)) as u32;
    let year = (100 * b + d - 4800 + m / 10) as i32;
    (year, month, day)
}

/// Determine the saju month for a given solar date using the precise 절기 data.
/// Returns (saju_month 1-12, is_before_ipchun).
///
/// For dates within the precise table range (2020-2030), exact 절입시각 is used.
/// For dates outside that range, a mathematical approximation is applied.
/// The approximation is accurate to ±1 day around month boundaries;
/// for dates well within a month, the result is always correct.
pub fn solar_date_to_saju_month(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> (u8, bool) {
    let term = find_active_jie_term(year as u16, month as u8, day as u8, hour as u8, minute as u8);
    match term {
        Some(term_index) => {
            let saju_month = jie_term_to_saju_month(term_index);
            // Before 입춘 if the active jie is 소한(0), we're in 축월 which is before 입춘
            (saju_month, term_index == 0)
        }
        None => {
            // Fallback: mathematical approximation for dates outside 2020-2030.
            // 절(Jie) terms that mark month boundaries occur around these solar dates:
            //   입춘 ~Feb 4, 경칩 ~Mar 6, 청명 ~Apr 5, 입하 ~May 6,
            //   망종 ~Jun 6, 소서 ~Jul 7, 입추 ~Aug 7, 백로 ~Sep 8,
            //   한로 ~Oct 8, 입동 ~Nov 7, 대설 ~Dec 7, 소한 ~Jan 6
            let approximate_saju_month = match (month, day) {
                (1, _) if day < 6 => 12,                          // 축월 (소한 이전 → 전년 12월)
                (1, _) | (2, 1..=3) => 12,                        // 축월
                (2, _) | (3, 1..=5) => 1,                         // 인월 (입춘 후)
                (3, _) | (4, 1..=4) => 2,                         // 묘월
                (4, _) | (5, 1..=5) => 3,                         // 진월
                (5, _) | (6, 1..=5) => 4,                         // 사월
                (6, _) | (7, 1..=6) => 5,                         // 오월
                (7, _) | (8, 1..=6) => 6,                         // 미월
                (8, _) | (9, 1..=7) => 7,                         // 신월
                (9, _) | (10, 1..=7) => 8,                        // 유월
                (10, _) | (11, 1..=6) => 9,                       // 술월
                (11, _) | (12, 1..=6) => 10,                      // 해월
                (12, _) => 11,                                     // 자월
                _ => 1,
            };
            let before_ipchun = approximate_saju_month == 12;
            tracing::debug!(
                "Solar terms table miss for {}-{}-{}; using mathematical approximation (saju month {})",
                year, month, day, approximate_saju_month
            );
            (approximate_saju_month, before_ipchun)
        }
    }
}

/// Check if a date falls in Korean DST period (1948-1988).
pub fn is_korean_dst(year: i32, month: u32, day: u32) -> bool {
    match year {
        1948 => (month == 6 && day >= 1) || (month >= 7 && month <= 8) || (month == 9 && day <= 12),
        1949..=1951 => (month >= 4 && month <= 8) || (month == 9 && day <= 20),
        1955..=1960 => (month >= 5 && month <= 8) || (month == 9 && day <= 20),
        1987..=1988 => (month >= 5 && month <= 9) || (month == 10 && day <= 14),
        _ => false,
    }
}

/// Generate all 60 일주 keys for daily fortune batch generation.
pub fn all_ilju_keys() -> Vec<String> {
    (0..60u8).map(|i| ilju_from_day_index(i)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ilju_from_day_index() {
        assert_eq!(ilju_from_day_index(0), "갑자");
        assert_eq!(ilju_from_day_index(1), "을축");
    }

    #[test]
    fn test_60_unique_ilju() {
        let mut seen = std::collections::HashSet::new();
        for i in 0..60 {
            let ilju = ilju_from_day_index(i);
            assert!(seen.insert(ilju.clone()), "Duplicate ilju: {}", ilju);
        }
        assert_eq!(seen.len(), 60);
    }

    #[test]
    fn test_year_pillar_2024() {
        let p = year_pillar(2024, false);
        assert_eq!(p.stem().korean, "갑");
        assert_eq!(p.branch().korean, "진");
    }

    #[test]
    fn test_year_pillar_2026() {
        let p = year_pillar(2026, false);
        assert_eq!(p.stem().korean, "병");
        assert_eq!(p.branch().korean, "오");
    }

    #[test]
    fn test_jdn_roundtrip() {
        let jdn = solar_to_jdn(2024, 3, 15);
        let (y, m, d) = jdn_to_solar(jdn);
        assert_eq!((y, m, d), (2024, 3, 15));
    }

    #[test]
    fn test_jdn_reference() {
        assert_eq!(solar_to_jdn(2000, 1, 1), 2451545);
    }

    #[test]
    fn test_korean_dst() {
        assert!(is_korean_dst(1987, 7, 15));
        assert!(!is_korean_dst(2024, 7, 15));
    }

    #[test]
    fn test_all_ilju_keys_count() {
        assert_eq!(all_ilju_keys().len(), 60);
    }

    #[test]
    fn test_saju_month_outside_table_range() {
        // 1990년 7월 15일 → 오월(5) 또는 미월(6) 근처
        let (month, before) = solar_date_to_saju_month(1990, 7, 15, 12, 0);
        assert!(month >= 5 && month <= 6, "1990-07-15 should be around month 5-6, got {}", month);
        assert!(!before);
    }

    #[test]
    fn test_saju_month_before_ipchun_fallback() {
        // 1985년 1월 3일 → 축월(12), 입춘 이전
        let (month, before) = solar_date_to_saju_month(1985, 1, 3, 12, 0);
        assert_eq!(month, 12);
        assert!(before);
    }

    #[test]
    fn test_saju_month_within_table_range() {
        // 2024년 3월 15일 → 묘월(2)
        let (month, before) = solar_date_to_saju_month(2024, 3, 15, 12, 0);
        assert_eq!(month, 2);
        assert!(!before);
    }
}
