//! 양력↔음력 변환 테이블 (Solar-Lunar Calendar Conversion) 2020-2030
//!
//! This module provides lookup tables for converting between the Gregorian (solar)
//! calendar and the traditional Korean lunar calendar (음력).
//!
//! Each year's lunar data is encoded as:
//! - Month lengths (29 or 30 days for each month)
//! - Leap month position and length (if any)
//! - The solar date of Lunar New Year's Day (1월 1일)
//!
//! Data source: Korea Astronomy and Space Science Institute (KASI).

/// Lunar year metadata
#[derive(Debug, Clone, Copy)]
pub struct LunarYearData {
    /// Gregorian year
    pub year: u16,
    /// Solar date of lunar 1월 1일: (month, day)
    pub new_year_solar: (u8, u8),
    /// Leap month number (0 = no leap month, 1-12 = after which month the leap is inserted)
    pub leap_month: u8,
    /// Bit-encoded month lengths: bit i (0-indexed from LSB) = 1 means month (i+1) has 30 days,
    /// 0 means 29 days. For 12 normal months + optional leap month.
    /// Bits 0-11 = months 1-12, bit 12 = leap month (if any).
    pub month_lengths: u16,
    /// Total days in this lunar year
    pub total_days: u16,
}

/// Lunar calendar data for years 2020-2030.
///
/// month_lengths encoding:
///   Bit 0 (LSB) = month 1, Bit 1 = month 2, ..., Bit 11 = month 12
///   Bit 12 = leap month (inserted after the month indicated by leap_month)
///   1 = 30 days (대월), 0 = 29 days (소월)
pub static LUNAR_YEARS: [LunarYearData; 11] = [
    // 2020 (경자년) - leap 4월
    LunarYearData {
        year: 2020,
        new_year_solar: (1, 25),
        leap_month: 4,
        month_lengths: 0b1_0100_1010_1101, // 13 months: 1-12 + leap4
        total_days: 384,
    },
    // 2021 (신축년) - no leap
    LunarYearData {
        year: 2021,
        new_year_solar: (2, 12),
        leap_month: 0,
        month_lengths: 0b0010_0101_0110,
        total_days: 354,
    },
    // 2022 (임인년) - no leap
    LunarYearData {
        year: 2022,
        new_year_solar: (2, 1),
        leap_month: 0,
        month_lengths: 0b1001_0101_0101,
        total_days: 355,
    },
    // 2023 (계묘년) - leap 2월
    LunarYearData {
        year: 2023,
        new_year_solar: (1, 22),
        leap_month: 2,
        month_lengths: 0b0_1011_0010_1011,
        total_days: 384,
    },
    // 2024 (갑진년) - no leap
    LunarYearData {
        year: 2024,
        new_year_solar: (2, 10),
        leap_month: 0,
        month_lengths: 0b0101_1001_0101,
        total_days: 354,
    },
    // 2025 (을사년) - leap 6월
    LunarYearData {
        year: 2025,
        new_year_solar: (1, 29),
        leap_month: 6,
        month_lengths: 0b1_0101_0110_1010,
        total_days: 384,
    },
    // 2026 (병오년) - no leap
    LunarYearData {
        year: 2026,
        new_year_solar: (2, 17),
        leap_month: 0,
        month_lengths: 0b1010_1011_0100,
        total_days: 354,
    },
    // 2027 (정미년) - no leap
    LunarYearData {
        year: 2027,
        new_year_solar: (2, 7),
        leap_month: 0,
        month_lengths: 0b0101_0101_1010,
        total_days: 355,
    },
    // 2028 (무신년) - leap 5월
    LunarYearData {
        year: 2028,
        new_year_solar: (1, 27),
        leap_month: 5,
        month_lengths: 0b0_1010_1101_0101,
        total_days: 384,
    },
    // 2029 (기유년) - no leap
    LunarYearData {
        year: 2029,
        new_year_solar: (2, 13),
        leap_month: 0,
        month_lengths: 0b0101_0110_1010,
        total_days: 354,
    },
    // 2030 (경술년) - no leap
    LunarYearData {
        year: 2030,
        new_year_solar: (2, 3),
        leap_month: 0,
        month_lengths: 0b1010_1011_0101,
        total_days: 355,
    },
];

/// Get lunar year data for a given Gregorian year (1940-2030).
pub fn get_lunar_year(year: u16) -> Option<&'static LunarYearData> {
    if year >= 2020 && year <= 2030 {
        Some(&LUNAR_YEARS[(year - 2020) as usize])
    } else if year >= 1940 && year <= 2019 {
        Some(&super::lunar_data_1940_2019::LUNAR_YEARS_1940_2019[(year - 1940) as usize])
    } else {
        None
    }
}

/// Get the number of days in a specific lunar month.
///
/// `month`: 1-12 for normal months
/// `is_leap`: true if querying the leap month
pub fn days_in_lunar_month(year: u16, month: u8, is_leap: bool) -> Option<u32> {
    let data = get_lunar_year(year)?;

    if is_leap {
        if data.leap_month != month {
            return None; // No leap month at this position
        }
        // Bit 12 encodes the leap month length
        let bit = (data.month_lengths >> 12) & 1;
        return Some(if bit == 1 { 30 } else { 29 });
    }

    if month < 1 || month > 12 {
        return None;
    }

    let bit = (data.month_lengths >> (month - 1)) & 1;
    Some(if bit == 1 { 30 } else { 29 })
}

/// Get the number of months in a lunar year (12 or 13 if there's a leap month).
pub fn months_in_lunar_year(year: u16) -> Option<u8> {
    let data = get_lunar_year(year)?;
    Some(if data.leap_month > 0 { 13 } else { 12 })
}

/// Convert a solar (Gregorian) date to a lunar date.
///
/// Returns: (lunar_year, lunar_month, lunar_day, is_leap_month)
pub fn solar_to_lunar(year: u16, month: u8, day: u8) -> Option<(u16, u8, u8, bool)> {
    // Find which lunar year this solar date falls in
    // We need to check if the date is before or after the lunar new year
    let lunar_year = if let Some(data) = get_lunar_year(year) {
        let (ny_m, ny_d) = data.new_year_solar;
        if (month, day) >= (ny_m, ny_d) {
            year
        } else if year > 1940 {
            year - 1
        } else {
            return None; // Before our data range
        }
    } else if year == 2031 {
        // Date in 2031 before lunar new year still belongs to lunar 2030
        2030
    } else {
        return None;
    };

    let data = get_lunar_year(lunar_year)?;
    let (ny_m, ny_d) = data.new_year_solar;

    // Calculate days from lunar new year's solar date to the target solar date
    let days_offset = days_between_solar(lunar_year, ny_m, ny_d, year, month, day)?;

    // Walk through lunar months to find the right one
    let mut remaining = days_offset as u32;
    let mut current_month: u8 = 1;
    let mut is_leap = false;

    loop {
        let month_days = days_in_lunar_month(lunar_year, current_month, is_leap)?;

        if remaining < month_days {
            return Some((lunar_year, current_month, remaining as u8 + 1, is_leap));
        }

        remaining -= month_days;

        if !is_leap && data.leap_month == current_month {
            // Next iteration processes the leap month
            is_leap = true;
        } else {
            is_leap = false;
            current_month += 1;
            if current_month > 12 {
                return None; // Shouldn't happen within valid data
            }
        }
    }
}

/// Convert a lunar date to a solar (Gregorian) date.
///
/// Returns: (solar_year, solar_month, solar_day)
pub fn lunar_to_solar(
    lunar_year: u16,
    lunar_month: u8,
    lunar_day: u8,
    is_leap_month: bool,
) -> Option<(u16, u8, u8)> {
    let data = get_lunar_year(lunar_year)?;

    // Calculate total days from lunar 1/1 to the target lunar date
    let mut total_days: u32 = 0;

    for m in 1..lunar_month {
        total_days += days_in_lunar_month(lunar_year, m, false)?;
        if data.leap_month == m {
            total_days += days_in_lunar_month(lunar_year, m, true)?;
        }
    }

    if is_leap_month {
        // Add the normal month days first
        total_days += days_in_lunar_month(lunar_year, lunar_month, false)?;
    }

    total_days += (lunar_day - 1) as u32;

    // Add days to the solar new year date
    let (ny_m, ny_d) = data.new_year_solar;
    add_days_to_solar(lunar_year, ny_m, ny_d, total_days)
}

// --- Helper functions ---

/// Days in a Gregorian month
fn days_in_solar_month(year: u16, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

fn is_leap_year(year: u16) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// Calculate the number of days between two solar dates (same year or consecutive).
fn days_between_solar(
    y1: u16,
    m1: u8,
    d1: u8,
    y2: u16,
    m2: u8,
    d2: u8,
) -> Option<u32> {
    if (y2, m2, d2) < (y1, m1, d1) {
        return None;
    }

    let mut days: u32 = 0;
    let mut cy = y1;
    let mut cm = m1;
    let mut cd = d1;

    while (cy, cm, cd) < (y2, m2, d2) {
        days += 1;
        cd += 1;
        if cd > days_in_solar_month(cy, cm) {
            cd = 1;
            cm += 1;
            if cm > 12 {
                cm = 1;
                cy += 1;
            }
        }
    }

    Some(days)
}

/// Add a number of days to a solar date.
fn add_days_to_solar(year: u16, month: u8, day: u8, days: u32) -> Option<(u16, u8, u8)> {
    let mut y = year;
    let mut m = month;
    let mut d = day;

    for _ in 0..days {
        d += 1;
        if d > days_in_solar_month(y, m) {
            d = 1;
            m += 1;
            if m > 12 {
                m = 1;
                y += 1;
            }
        }
    }

    Some((y, m, d))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_range() {
        assert!(get_lunar_year(2019).is_none());
        assert!(get_lunar_year(2020).is_some());
        assert!(get_lunar_year(2030).is_some());
        assert!(get_lunar_year(2031).is_none());
    }

    #[test]
    fn test_lunar_new_year_2024() {
        let data = get_lunar_year(2024).unwrap();
        assert_eq!(data.new_year_solar, (2, 10));
    }

    #[test]
    fn test_solar_to_lunar_new_year() {
        // 2024-02-10 = Lunar 2024-01-01
        let result = solar_to_lunar(2024, 2, 10).unwrap();
        assert_eq!(result, (2024, 1, 1, false));
    }

    #[test]
    fn test_lunar_to_solar_roundtrip() {
        // Lunar 2024-01-01 → Solar 2024-02-10
        let solar = lunar_to_solar(2024, 1, 1, false).unwrap();
        assert_eq!(solar, (2024, 2, 10));

        // And back
        let lunar = solar_to_lunar(solar.0, solar.1, solar.2).unwrap();
        assert_eq!(lunar, (2024, 1, 1, false));
    }

    #[test]
    fn test_leap_year_2020() {
        let data = get_lunar_year(2020).unwrap();
        assert_eq!(data.leap_month, 4);
        assert_eq!(data.total_days, 384);
    }

    #[test]
    fn test_months_in_year() {
        assert_eq!(months_in_lunar_year(2020).unwrap(), 13); // leap year
        assert_eq!(months_in_lunar_year(2021).unwrap(), 12); // normal year
    }
}
