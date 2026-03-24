use serde::{Deserialize, Serialize};

/// 12시진 (Earthly Branch hours)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum BirthHour {
    Ja,      // 자시 23:00~01:00
    Chuk,    // 축시 01:00~03:00
    In,      // 인시 03:00~05:00
    Myo,     // 묘시 05:00~07:00
    Jin,     // 진시 07:00~09:00
    Sa,      // 사시 09:00~11:00
    O,       // 오시 11:00~13:00
    Mi,      // 미시 13:00~15:00
    Sin,     // 신시 15:00~17:00
    Yu,      // 유시 17:00~19:00
    Sul,     // 술시 19:00~21:00
    Hae,     // 해시 21:00~23:00
    Unknown, // 모름
}

impl BirthHour {
    pub fn from_str_val(s: &str) -> Option<Self> {
        match s {
            "ja" => Some(Self::Ja),
            "chuk" => Some(Self::Chuk),
            "in" => Some(Self::In),
            "myo" => Some(Self::Myo),
            "jin" => Some(Self::Jin),
            "sa" => Some(Self::Sa),
            "o" => Some(Self::O),
            "mi" => Some(Self::Mi),
            "sin" => Some(Self::Sin),
            "yu" => Some(Self::Yu),
            "sul" => Some(Self::Sul),
            "hae" => Some(Self::Hae),
            "unknown" => Some(Self::Unknown),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ja => "ja",
            Self::Chuk => "chuk",
            Self::In => "in",
            Self::Myo => "myo",
            Self::Jin => "jin",
            Self::Sa => "sa",
            Self::O => "o",
            Self::Mi => "mi",
            Self::Sin => "sin",
            Self::Yu => "yu",
            Self::Sul => "sul",
            Self::Hae => "hae",
            Self::Unknown => "unknown",
        }
    }

    /// Convert birth hour to index (0-11) for 시주 calculation
    /// Returns None for Unknown
    pub fn to_index(&self) -> Option<usize> {
        match self {
            Self::Ja => Some(0),
            Self::Chuk => Some(1),
            Self::In => Some(2),
            Self::Myo => Some(3),
            Self::Jin => Some(4),
            Self::Sa => Some(5),
            Self::O => Some(6),
            Self::Mi => Some(7),
            Self::Sin => Some(8),
            Self::Yu => Some(9),
            Self::Sul => Some(10),
            Self::Hae => Some(11),
            Self::Unknown => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CalendarType {
    Solar,
    Lunar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
}

/// Birth input from client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BirthInput {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub calendar_type: CalendarType,
    #[serde(default)]
    pub is_leap_month: bool,
    #[serde(default = "default_birth_hour")]
    pub birth_hour: BirthHour,
    pub gender: Gender,
}

fn default_birth_hour() -> BirthHour {
    BirthHour::Unknown
}

impl BirthInput {
    /// Check if this birth date falls in Korean summer time period (1948-1988)
    pub fn needs_summer_time_correction(&self) -> bool {
        if self.calendar_type == CalendarType::Lunar {
            return false; // will check after solar conversion
        }
        self.year >= 1948 && self.year <= 1988
    }

    /// Generate HMAC cache key material (non-PII)
    pub fn cache_key_material(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}:{}:{}",
            self.year,
            self.month,
            self.day,
            match self.calendar_type {
                CalendarType::Solar => "s",
                CalendarType::Lunar => "l",
            },
            self.is_leap_month,
            self.birth_hour.as_str(),
            match self.gender {
                Gender::Male => "m",
                Gender::Female => "f",
            }
        )
    }
}
