/// Layer 1: 만세력 연산 (Deterministic Saju Calculation Engine)
///
/// This layer performs pure table-lookup-based calculation:
/// - Solar/Lunar date handling via pre-computed KASI tables
/// - Four Pillars (사주팔자) determination
/// - All calculations are deterministic and testable

use crate::errors::AppError;
use crate::models::birth::{BirthHour, BirthInput, CalendarType};
use crate::models::saju::FourPillars;

use super::tables;

/// The Saju calculation engine (Layer 1)
pub struct SajuEngine;

impl SajuEngine {
    pub fn new() -> Self {
        Self
    }

    /// Calculate the four pillars from birth input.
    pub fn calculate_four_pillars(&self, input: &BirthInput) -> Result<FourPillars, AppError> {
        // Step 1: Resolve to solar date
        let (solar_year, solar_month, solar_day) = self.resolve_solar_date(input)?;

        // Step 2: Determine saju month using precise 절기 data
        let (saju_month, before_ipchun) =
            tables::solar_date_to_saju_month(solar_year, solar_month, solar_day, 12, 0);

        // Step 3: Handle 야자시 (23:00-01:00) — in 야자시론, 자시 belongs to the NEXT day
        let (eff_year, eff_saju_month, day_jdn, eff_before_ipchun) =
            if input.birth_hour == BirthHour::Ja {
                let next_jdn = tables::solar_to_jdn(solar_year, solar_month, solar_day) + 1;
                let (ny, nm, nd) = tables::jdn_to_solar(next_jdn);
                let (new_month, new_before) =
                    tables::solar_date_to_saju_month(ny as i32, nm, nd, 0, 0);
                (ny as i32, new_month, next_jdn, new_before)
            } else {
                let jdn = tables::solar_to_jdn(solar_year, solar_month, solar_day);
                (solar_year, saju_month, jdn, before_ipchun)
            };

        // Step 4: Year pillar
        let year_pillar = tables::year_pillar(eff_year, eff_before_ipchun);

        // Step 5: Month pillar
        let month_pillar = tables::month_pillar(year_pillar.stem_index, eff_saju_month);

        // Step 6: Day pillar
        let day_pillar = tables::day_pillar_from_jdn(day_jdn);

        // Step 7: Hour pillar (if birth hour is known)
        let hour_pillar = self.calculate_hour_pillar(
            day_pillar.stem_index,
            input.birth_hour,
            solar_year,
            solar_month,
            solar_day,
        );

        Ok(FourPillars::from_pillars(year_pillar, month_pillar, day_pillar, hour_pillar))
    }

    /// Resolve birth input to a solar date.
    fn resolve_solar_date(&self, input: &BirthInput) -> Result<(i32, u32, u32), AppError> {
        match input.calendar_type {
            CalendarType::Solar => {
                if input.month < 1 || input.month > 12 || input.day < 1 || input.day > 31 {
                    return Err(AppError::BadRequest("Invalid date".to_string()));
                }
                Ok((input.year, input.month, input.day))
            }
            CalendarType::Lunar => {
                let result = tables::lunar_calendar::lunar_to_solar(
                    input.year as u16,
                    input.month as u8,
                    input.day as u8,
                    input.is_leap_month,
                );
                match result {
                    Some((y, m, d)) => Ok((y as i32, m as u32, d as u32)),
                    None => {
                        // 음력 변환 테이블은 2020-2030만 지원.
                        // 이 범위 밖의 음력 입력은 양력으로 입력하도록 안내.
                        if input.year < 2020 || input.year > 2030 {
                            Err(AppError::BadRequest(format!(
                                "음력 {}년은 지원 범위(2020-2030) 밖입니다. 양력(solar)으로 입력해 주세요.",
                                input.year
                            )))
                        } else {
                            Err(AppError::BadRequest(format!(
                                "유효하지 않은 음력 날짜: {}-{}-{} (윤달: {})",
                                input.year, input.month, input.day, input.is_leap_month
                            )))
                        }
                    }
                }
            }
        }
    }

    /// Calculate hour pillar if birth hour is known.
    fn calculate_hour_pillar(
        &self,
        day_stem_index: u8,
        birth_hour: BirthHour,
        solar_year: i32,
        solar_month: u32,
        solar_day: u32,
    ) -> Option<tables::Pillar> {
        let hour_index = birth_hour.to_index()? as u8;

        // DST note: for 1948-1988, the recorded time may be 1 hour ahead.
        // Since we only know the 시진 (2-hour period), not exact minute,
        // DST typically doesn't change the 시진. We log a warning.
        if tables::is_korean_dst(solar_year, solar_month, solar_day) {
            tracing::debug!(
                "Birth during Korean DST period ({}-{}-{}). 시진 may need adjustment for exact birth time.",
                solar_year, solar_month, solar_day
            );
        }

        Some(tables::hour_pillar(day_stem_index, hour_index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::birth::Gender;

    fn make_input(year: i32, month: u32, day: u32, hour: BirthHour) -> BirthInput {
        BirthInput {
            year, month, day,
            calendar_type: CalendarType::Solar,
            is_leap_month: false,
            birth_hour: hour,
            gender: Gender::Male,
        }
    }

    #[test]
    fn test_basic_calculation() {
        let engine = SajuEngine::new();
        let input = make_input(2024, 3, 15, BirthHour::In);
        let result = engine.calculate_four_pillars(&input);
        assert!(result.is_ok());
        let fp = result.unwrap();
        // 2024 is 갑진년
        assert_eq!(fp.year_pillar().stem().korean, "갑");
        assert!(fp.hour_pillar().is_some());
    }

    #[test]
    fn test_unknown_birth_hour() {
        let engine = SajuEngine::new();
        let input = make_input(2024, 6, 15, BirthHour::Unknown);
        let fp = engine.calculate_four_pillars(&input).unwrap();
        assert!(fp.hour_pillar().is_none());
    }

    #[test]
    fn test_ilju_name() {
        let engine = SajuEngine::new();
        let input = make_input(2026, 3, 24, BirthHour::In);
        let fp = engine.calculate_four_pillars(&input).unwrap();
        assert!(!fp.ilju_name().is_empty());
    }

    #[test]
    fn test_outside_table_range_solar() {
        // 양력 입력은 범위 밖이어도 수학적 fallback으로 계산 가능
        let engine = SajuEngine::new();
        let input = make_input(1990, 7, 15, BirthHour::O);
        let result = engine.calculate_four_pillars(&input);
        assert!(result.is_ok(), "Solar dates outside 2020-2030 should work with fallback");
        let fp = result.unwrap();
        // 1990 = 경오년
        assert_eq!(fp.year_pillar().stem().korean, "경");
        assert_eq!(fp.year_pillar().branch().korean, "오");
    }

    #[test]
    fn test_outside_table_range_lunar_rejected() {
        // 음력 입력은 범위 밖이면 에러
        let engine = SajuEngine::new();
        let mut input = make_input(1990, 7, 15, BirthHour::O);
        input.calendar_type = CalendarType::Lunar;
        let result = engine.calculate_four_pillars(&input);
        assert!(result.is_err(), "Lunar dates outside 2020-2030 should error");
    }
}
