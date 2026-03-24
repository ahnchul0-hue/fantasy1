//! 24절기 (Solar Terms) - 절입시각 Lookup Table (2020-2030)
//!
//! The 24 solar terms (절기/節氣) divide the solar year into 24 segments.
//! For saju calculation, the critical terms are the 12 "절" (Jie/節) terms
//! that determine month boundaries in the Four Pillars system.
//!
//! Each entry records the exact minute of the solar term transition (절입시각).
//! Data source: Korea Astronomy and Space Science Institute (KASI).

/// A solar term entry with its exact transition time
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SolarTermEntry {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub term_index: u8,
    pub term_korean: &'static str,
    pub term_hanja: &'static str,
    /// true if this is a 절(Jie) term that marks a month boundary for saju
    pub is_month_boundary: bool,
}

/// The 24 solar term names in order
pub static SOLAR_TERM_NAMES: [(&str, &str, bool); 24] = [
    ("소한", "小寒", true),    // 0  - Minor Cold (month 12 / 축월 start)
    ("대한", "大寒", false),   // 1  - Major Cold
    ("입춘", "立春", true),    // 2  - Start of Spring (month 1 / 인월 start)
    ("우수", "雨水", false),   // 3  - Rain Water
    ("경칩", "驚蟄", true),    // 4  - Awakening of Insects (month 2 / 묘월 start)
    ("춘분", "春分", false),   // 5  - Spring Equinox
    ("청명", "清明", true),    // 6  - Clear and Bright (month 3 / 진월 start)
    ("곡우", "穀雨", false),   // 7  - Grain Rain
    ("입하", "立夏", true),    // 8  - Start of Summer (month 4 / 사월 start)
    ("소만", "小滿", false),   // 9  - Grain Full
    ("망종", "芒種", true),    // 10 - Grain in Ear (month 5 / 오월 start)
    ("하지", "夏至", false),   // 11 - Summer Solstice
    ("소서", "小暑", true),    // 12 - Minor Heat (month 6 / 미월 start)
    ("대서", "大暑", false),   // 13 - Major Heat
    ("입추", "立秋", true),    // 14 - Start of Autumn (month 7 / 신월 start)
    ("처서", "處暑", false),   // 15 - End of Heat
    ("백로", "白露", true),    // 16 - White Dew (month 8 / 유월 start)
    ("추분", "秋分", false),   // 17 - Autumn Equinox
    ("한로", "寒露", true),    // 18 - Cold Dew (month 9 / 술월 start)
    ("상강", "霜降", false),   // 19 - Frost Descent
    ("입동", "立冬", true),    // 20 - Start of Winter (month 10 / 해월 start)
    ("소설", "小雪", false),   // 21 - Minor Snow
    ("대설", "大雪", true),    // 22 - Major Snow (month 11 / 자월 start)
    ("동지", "冬至", false),   // 23 - Winter Solstice
];

/// Solar term transition times (절입시각) for 2020-2030.
///
/// Each tuple: (year, month, day, hour, minute, term_index)
/// Only 절(Jie) terms (month boundaries) are included here for saju calculation.
/// These are the 12 terms where is_month_boundary = true.
///
/// Source: Korea Astronomy and Space Science Institute (KASI) data.
/// Times are in KST (UTC+9).
pub static JIE_TERMS: &[(u16, u8, u8, u8, u8, u8)] = &[
    // 2020
    (2020, 1, 6, 6, 30, 0),    // 소한
    (2020, 2, 4, 18, 3, 2),    // 입춘
    (2020, 3, 5, 11, 57, 4),   // 경칩
    (2020, 4, 4, 16, 38, 6),   // 청명
    (2020, 5, 5, 9, 51, 8),    // 입하
    (2020, 6, 5, 13, 58, 10),  // 망종
    (2020, 7, 6, 23, 14, 12),  // 소서
    (2020, 8, 7, 10, 6, 14),   // 입추
    (2020, 9, 7, 13, 8, 16),   // 백로
    (2020, 10, 8, 4, 55, 18),  // 한로
    (2020, 11, 7, 8, 14, 20),  // 입동
    (2020, 12, 7, 1, 9, 22),   // 대설

    // 2021
    (2021, 1, 5, 12, 23, 0),
    (2021, 2, 3, 23, 59, 2),
    (2021, 3, 5, 17, 54, 4),
    (2021, 4, 4, 22, 35, 6),
    (2021, 5, 5, 15, 47, 8),
    (2021, 6, 5, 19, 52, 10),
    (2021, 7, 7, 5, 5, 12),
    (2021, 8, 7, 15, 54, 14),
    (2021, 9, 7, 18, 53, 16),
    (2021, 10, 8, 10, 39, 18),
    (2021, 11, 7, 13, 59, 20),
    (2021, 12, 7, 6, 57, 22),

    // 2022
    (2022, 1, 5, 18, 14, 0),
    (2022, 2, 4, 5, 51, 2),
    (2022, 3, 5, 23, 44, 4),
    (2022, 4, 5, 4, 20, 6),
    (2022, 5, 5, 21, 26, 8),
    (2022, 6, 6, 1, 26, 10),
    (2022, 7, 7, 10, 38, 12),
    (2022, 8, 7, 21, 29, 14),
    (2022, 9, 8, 0, 32, 16),
    (2022, 10, 8, 16, 22, 18),
    (2022, 11, 7, 19, 45, 20),
    (2022, 12, 7, 12, 46, 22),

    // 2023
    (2023, 1, 6, 0, 5, 0),
    (2023, 2, 4, 11, 43, 2),
    (2023, 3, 6, 5, 36, 4),
    (2023, 4, 5, 10, 13, 6),
    (2023, 5, 6, 3, 19, 8),
    (2023, 6, 6, 7, 18, 10),
    (2023, 7, 7, 16, 31, 12),
    (2023, 8, 8, 3, 23, 14),
    (2023, 9, 8, 6, 27, 16),
    (2023, 10, 8, 22, 16, 18),
    (2023, 11, 8, 1, 36, 20),
    (2023, 12, 7, 18, 33, 22),

    // 2024
    (2024, 1, 6, 5, 49, 0),
    (2024, 2, 4, 17, 27, 2),
    (2024, 3, 5, 11, 23, 4),
    (2024, 4, 4, 16, 2, 6),
    (2024, 5, 5, 9, 10, 8),
    (2024, 6, 5, 13, 10, 10),
    (2024, 7, 6, 22, 20, 12),
    (2024, 8, 7, 9, 9, 14),
    (2024, 9, 7, 12, 11, 16),
    (2024, 10, 8, 3, 59, 18),
    (2024, 11, 7, 7, 20, 20),
    (2024, 12, 7, 0, 17, 22),

    // 2025
    (2025, 1, 5, 11, 33, 0),
    (2025, 2, 3, 23, 10, 2),
    (2025, 3, 5, 17, 7, 4),
    (2025, 4, 4, 21, 48, 6),
    (2025, 5, 5, 14, 57, 8),
    (2025, 6, 5, 19, 0, 10),
    (2025, 7, 7, 4, 12, 12),
    (2025, 8, 7, 14, 52, 14),
    (2025, 9, 7, 17, 52, 16),
    (2025, 10, 8, 9, 41, 18),
    (2025, 11, 7, 13, 4, 20),
    (2025, 12, 7, 6, 5, 22),

    // 2026
    (2026, 1, 5, 17, 23, 0),
    (2026, 2, 4, 4, 52, 2),
    (2026, 3, 5, 22, 49, 4),
    (2026, 4, 5, 3, 30, 6),
    (2026, 5, 5, 20, 41, 8),
    (2026, 6, 6, 0, 42, 10),
    (2026, 7, 7, 9, 57, 12),
    (2026, 8, 7, 20, 43, 14),
    (2026, 9, 7, 23, 41, 16),
    (2026, 10, 8, 15, 29, 18),
    (2026, 11, 7, 18, 52, 20),
    (2026, 12, 7, 11, 52, 22),

    // 2027
    (2027, 1, 5, 23, 10, 0),
    (2027, 2, 4, 10, 46, 2),
    (2027, 3, 6, 4, 39, 4),
    (2027, 4, 5, 9, 17, 6),
    (2027, 5, 6, 2, 25, 8),
    (2027, 6, 6, 6, 26, 10),
    (2027, 7, 7, 15, 37, 12),
    (2027, 8, 8, 2, 27, 14),
    (2027, 9, 8, 5, 28, 16),
    (2027, 10, 8, 21, 16, 18),
    (2027, 11, 8, 0, 38, 20),
    (2027, 12, 7, 17, 37, 22),

    // 2028
    (2028, 1, 6, 4, 55, 0),
    (2028, 2, 4, 16, 34, 2),
    (2028, 3, 5, 10, 26, 4),
    (2028, 4, 4, 15, 2, 6),
    (2028, 5, 5, 8, 10, 8),
    (2028, 6, 5, 12, 9, 10),
    (2028, 7, 6, 21, 21, 12),
    (2028, 8, 7, 8, 11, 14),
    (2028, 9, 7, 11, 15, 16),
    (2028, 10, 8, 3, 1, 18),
    (2028, 11, 7, 6, 21, 20),
    (2028, 12, 6, 23, 19, 22),

    // 2029
    (2029, 1, 5, 10, 41, 0),
    (2029, 2, 3, 22, 20, 2),
    (2029, 3, 5, 16, 7, 4),
    (2029, 4, 4, 20, 43, 6),
    (2029, 5, 5, 13, 47, 8),
    (2029, 6, 5, 17, 49, 10),
    (2029, 7, 7, 3, 1, 12),
    (2029, 8, 7, 13, 52, 14),
    (2029, 9, 7, 17, 1, 16),
    (2029, 10, 8, 8, 49, 18),
    (2029, 11, 7, 12, 8, 20),
    (2029, 12, 7, 5, 4, 22),

    // 2030
    (2030, 1, 5, 16, 23, 0),
    (2030, 2, 4, 4, 8, 2),
    (2030, 3, 5, 22, 0, 4),
    (2030, 4, 5, 2, 33, 6),
    (2030, 5, 5, 19, 33, 8),
    (2030, 6, 5, 23, 28, 10),
    (2030, 7, 7, 8, 41, 12),
    (2030, 8, 7, 19, 36, 14),
    (2030, 9, 7, 22, 47, 16),
    (2030, 10, 8, 14, 35, 18),
    (2030, 11, 7, 17, 54, 20),
    (2030, 12, 7, 10, 48, 22),
];

/// Find the active 절기 month boundary for a given date+time in KST.
/// Returns the term_index of the most recent Jie term before the given datetime.
pub fn find_active_jie_term(year: u16, month: u8, day: u8, hour: u8, minute: u8) -> Option<u8> {
    let target = (year, month, day, hour, minute);

    // Find the last Jie term that is <= the target datetime
    let mut result: Option<u8> = None;
    for &(y, m, d, h, min, term_idx) in JIE_TERMS.iter() {
        if (y, m, d, h, min) <= target {
            result = Some(term_idx);
        } else {
            break;
        }
    }

    // If we didn't find one in the same year, check the previous year's last entry
    if result.is_none() && year > 2020 {
        // The target is before the first Jie term of the year,
        // so the active term is the last 대설(Major Snow) of the previous year
        for &(y, _m, _d, _h, _min, term_idx) in JIE_TERMS.iter().rev() {
            if y < year {
                result = Some(term_idx);
                break;
            }
        }
    }

    result
}

/// Get the saju month number (1-12) from a Jie term index.
/// 입춘(2) = month 1 (인월), 경칩(4) = month 2 (묘월), etc.
pub fn jie_term_to_saju_month(term_index: u8) -> u8 {
    match term_index {
        2 => 1,   // 입춘 → 인월 (1월)
        4 => 2,   // 경칩 → 묘월 (2월)
        6 => 3,   // 청명 → 진월 (3월)
        8 => 4,   // 입하 → 사월 (4월)
        10 => 5,  // 망종 → 오월 (5월)
        12 => 6,  // 소서 → 미월 (6월)
        14 => 7,  // 입추 → 신월 (7월)
        16 => 8,  // 백로 → 유월 (8월)
        18 => 9,  // 한로 → 술월 (9월)
        20 => 10, // 입동 → 해월 (10월)
        22 => 11, // 대설 → 자월 (11월)
        0 => 12,  // 소한 → 축월 (12월)
        _ => 0,   // Invalid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jie_term_count_per_year() {
        for year in 2020..=2030 {
            let count = JIE_TERMS.iter().filter(|t| t.0 == year).count();
            assert_eq!(count, 12, "Year {} should have 12 Jie terms", year);
        }
    }

    #[test]
    fn test_find_active_jie_term() {
        // 2024-02-05 (after 입춘 on 2024-02-04 17:27)
        let term = find_active_jie_term(2024, 2, 5, 0, 0);
        assert_eq!(term, Some(2)); // 입춘
    }

    #[test]
    fn test_jie_to_saju_month() {
        assert_eq!(jie_term_to_saju_month(2), 1);   // 입춘 → 1월
        assert_eq!(jie_term_to_saju_month(22), 11);  // 대설 → 11월
        assert_eq!(jie_term_to_saju_month(0), 12);   // 소한 → 12월
    }

    #[test]
    fn test_terms_are_chronologically_sorted() {
        for window in JIE_TERMS.windows(2) {
            let a = (window[0].0, window[0].1, window[0].2, window[0].3, window[0].4);
            let b = (window[1].0, window[1].1, window[1].2, window[1].3, window[1].4);
            assert!(a < b, "Terms must be chronologically sorted: {:?} >= {:?}", a, b);
        }
    }
}
