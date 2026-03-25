use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::saju::tables::{self, Pillar, HEAVENLY_STEMS, EARTHLY_BRANCHES};
use crate::saju::tables::heavenly_stems::Element;

// ========================================
// Four Pillars (사주팔자)
// ========================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FourPillars {
    pub year_stem: u8,
    pub year_branch: u8,
    pub month_stem: u8,
    pub month_branch: u8,
    pub day_stem: u8,
    pub day_branch: u8,
    pub hour_stem: Option<u8>,
    pub hour_branch: Option<u8>,
}

impl FourPillars {
    pub fn from_pillars(year: Pillar, month: Pillar, day: Pillar, hour: Option<Pillar>) -> Self {
        Self {
            year_stem: year.stem_index,
            year_branch: year.branch_index,
            month_stem: month.stem_index,
            month_branch: month.branch_index,
            day_stem: day.stem_index,
            day_branch: day.branch_index,
            hour_stem: hour.map(|h| h.stem_index),
            hour_branch: hour.map(|h| h.branch_index),
        }
    }

    pub fn year_pillar(&self) -> Pillar {
        Pillar::new(self.year_stem, self.year_branch)
    }

    pub fn month_pillar(&self) -> Pillar {
        Pillar::new(self.month_stem, self.month_branch)
    }

    pub fn day_pillar(&self) -> Pillar {
        Pillar::new(self.day_stem, self.day_branch)
    }

    pub fn hour_pillar(&self) -> Option<Pillar> {
        match (self.hour_stem, self.hour_branch) {
            (Some(s), Some(b)) => Some(Pillar::new(s, b)),
            _ => None,
        }
    }

    /// Day master (일간) - the heavenly stem of the day pillar
    pub fn day_master(&self) -> &'static tables::heavenly_stems::HeavenlyStem {
        &HEAVENLY_STEMS[self.day_stem as usize]
    }

    /// Get the 일주 name in Korean (e.g., "갑자")
    /// 일주 = 일간(天干) + 일지(地支), NOT 일간 + 오행
    pub fn ilju_name(&self) -> String {
        let stem = &HEAVENLY_STEMS[self.day_stem as usize];
        let branch = &EARTHLY_BRANCHES[self.day_branch as usize];
        format!("{}{}", stem.korean, branch.korean)
    }

    /// Get the 일주 hanja (e.g., "甲子")
    pub fn ilju_hanja(&self) -> String {
        let stem = &HEAVENLY_STEMS[self.day_stem as usize];
        let branch = &EARTHLY_BRANCHES[self.day_branch as usize];
        format!("{}{}", stem.hanja, branch.hanja)
    }

    /// Get the 일주 key for daily fortune lookup — same as ilju_name (e.g., "갑자")
    /// 60갑자 중 하나로, 오늘의 운세 사전 생성 키로 사용
    pub fn ilju_key(&self) -> String {
        self.ilju_name()
    }
}

// ========================================
// Pillar API Response
// ========================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PillarResponse {
    pub heavenly_stem: String,
    pub earthly_branch: String,
    pub heavenly_stem_hanja: String,
    pub earthly_branch_hanja: String,
}

impl From<Pillar> for PillarResponse {
    fn from(p: Pillar) -> Self {
        Self {
            heavenly_stem: p.stem().korean.to_string(),
            earthly_branch: p.branch().korean.to_string(),
            heavenly_stem_hanja: p.stem().hanja.to_string(),
            earthly_branch_hanja: p.branch().hanja.to_string(),
        }
    }
}

impl From<Option<Pillar>> for PillarResponse {
    fn from(p: Option<Pillar>) -> Self {
        match p {
            Some(p) => p.into(),
            None => Self {
                heavenly_stem: "미상".to_string(),
                earthly_branch: "미상".to_string(),
                heavenly_stem_hanja: "未詳".to_string(),
                earthly_branch_hanja: "未詳".to_string(),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FourPillarsResponse {
    pub year: PillarResponse,
    pub month: PillarResponse,
    pub day: PillarResponse,
    pub hour: PillarResponse,
}

impl From<&FourPillars> for FourPillarsResponse {
    fn from(fp: &FourPillars) -> Self {
        Self {
            year: fp.year_pillar().into(),
            month: fp.month_pillar().into(),
            day: fp.day_pillar().into(),
            hour: fp.hour_pillar().into(),
        }
    }
}

// ========================================
// 오행 (Five Elements) Balance
// ========================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OhengBalance {
    pub wood: f64,
    pub fire: f64,
    pub earth: f64,
    pub metal: f64,
    pub water: f64,
}

impl OhengBalance {
    pub fn new() -> Self {
        Self { wood: 0.0, fire: 0.0, earth: 0.0, metal: 0.0, water: 0.0 }
    }

    pub fn add(&mut self, element: Element, amount: f64) {
        match element {
            Element::Wood => self.wood += amount,
            Element::Fire => self.fire += amount,
            Element::Earth => self.earth += amount,
            Element::Metal => self.metal += amount,
            Element::Water => self.water += amount,
        }
    }

    pub fn total(&self) -> f64 {
        self.wood + self.fire + self.earth + self.metal + self.water
    }

    pub fn normalized(&self) -> Self {
        let total = self.total();
        if total == 0.0 { return Self::new(); }
        let scale = 10.0 / total;
        Self {
            wood: (self.wood * scale * 10.0).round() / 10.0,
            fire: (self.fire * scale * 10.0).round() / 10.0,
            earth: (self.earth * scale * 10.0).round() / 10.0,
            metal: (self.metal * scale * 10.0).round() / 10.0,
            water: (self.water * scale * 10.0).round() / 10.0,
        }
    }

    pub fn get(&self, element: Element) -> f64 {
        match element {
            Element::Wood => self.wood,
            Element::Fire => self.fire,
            Element::Earth => self.earth,
            Element::Metal => self.metal,
            Element::Water => self.water,
        }
    }

    pub fn element_status(&self, element: Element) -> ElementStatus {
        let normalized = self.normalized();
        let value = normalized.get(element);
        if value >= 3.0 { ElementStatus::Excessive }
        else if value <= 1.0 { ElementStatus::Deficient }
        else { ElementStatus::Balanced }
    }

    pub fn lucky_element(&self) -> Element {
        let normalized = self.normalized();
        let elements = [Element::Wood, Element::Fire, Element::Earth, Element::Metal, Element::Water];
        *elements.iter()
            .min_by(|a, b| normalized.get(**a).partial_cmp(&normalized.get(**b)).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(&Element::Wood)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ElementStatus {
    Excessive,
    Balanced,
    Deficient,
}

impl ElementStatus {
    pub fn korean(&self) -> &'static str {
        match self {
            Self::Excessive => "과다",
            Self::Balanced => "적절",
            Self::Deficient => "부족",
        }
    }
}

// ========================================
// 십신 (Ten Gods)
// ========================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TenGod {
    Bigyeon,    // 비견
    Geupjae,    // 겁재
    Sikshin,    // 식신
    Sanggwan,   // 상관
    Pyeonjae,   // 편재
    Jeongjae,   // 정재
    Pyeongwan,  // 편관
    Jeonggwan,  // 정관
    Pyeonin,    // 편인
    Jeongin,    // 정인
}

impl TenGod {
    pub fn korean(&self) -> &'static str {
        match self {
            Self::Bigyeon => "비견", Self::Geupjae => "겁재",
            Self::Sikshin => "식신", Self::Sanggwan => "상관",
            Self::Pyeonjae => "편재", Self::Jeongjae => "정재",
            Self::Pyeongwan => "편관", Self::Jeonggwan => "정관",
            Self::Pyeonin => "편인", Self::Jeongin => "정인",
        }
    }

    /// Determine the ten god relationship between day master and another stem.
    pub fn from_relationship(day_master_idx: u8, other_idx: u8) -> Self {
        let dm = &HEAVENLY_STEMS[day_master_idx as usize];
        let ot = &HEAVENLY_STEMS[other_idx as usize];
        let same_polarity = dm.polarity == ot.polarity;

        // Element relationships
        let dm_elem = dm.element;
        let ot_elem = ot.element;

        if dm_elem == ot_elem {
            if same_polarity { Self::Bigyeon } else { Self::Geupjae }
        } else if generates(dm_elem) == ot_elem {
            if same_polarity { Self::Sikshin } else { Self::Sanggwan }
        } else if overcomes(dm_elem) == ot_elem {
            if same_polarity { Self::Pyeonjae } else { Self::Jeongjae }
        } else if overcomes(ot_elem) == dm_elem {
            if same_polarity { Self::Pyeongwan } else { Self::Jeonggwan }
        } else {
            if same_polarity { Self::Pyeonin } else { Self::Jeongin }
        }
    }
}

/// 상생 (generating cycle)
fn generates(e: Element) -> Element {
    match e {
        Element::Wood => Element::Fire,
        Element::Fire => Element::Earth,
        Element::Earth => Element::Metal,
        Element::Metal => Element::Water,
        Element::Water => Element::Wood,
    }
}

/// 상극 (overcoming cycle)
fn overcomes(e: Element) -> Element {
    match e {
        Element::Wood => Element::Earth,
        Element::Fire => Element::Metal,
        Element::Earth => Element::Water,
        Element::Metal => Element::Wood,
        Element::Water => Element::Fire,
    }
}

// ========================================
// 대운 (Major Fortune Period)
// ========================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaeunPeriod {
    pub start_age: i32,
    pub stem_index: u8,
    pub branch_index: u8,
}

// ========================================
// L2 Analysis Result
// ========================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SajuAnalysis {
    pub four_pillars: FourPillars,
    pub oheng_balance: OhengBalance,
    pub normalized_oheng: OhengBalance,
    pub day_master_index: u8,
    pub day_master_strength: ElementStatus,
    pub lucky_element: String,
    pub ten_gods: Vec<(String, String)>,
    pub daeun: Vec<DaeunPeriod>,
    pub keywords: Vec<String>,
    pub element_statuses: Vec<(String, String)>,
}

// ========================================
// API Response Types
// ========================================
#[derive(Debug, Serialize, Deserialize)]
pub struct SajuCardResponse {
    pub id: Uuid,
    pub ilju_name: String,
    pub ilju_hanja: String,
    pub keywords: Vec<String>,
    pub lucky_element: String,
    pub image_url: Option<String>,
    pub share_url: Option<String>,
    pub cached: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SajuProfileResponse {
    pub id: Uuid,
    pub birth_input: super::birth::BirthInput,
    pub four_pillars: FourPillarsResponse,
    pub oheng_balance: OhengBalance,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompatibilityPreviewResponse {
    pub score: i32,
    pub summary: String,
    pub person1_element: String,
    pub person2_element: String,
}

// ========================================
// DB Row for saju_cards
// ========================================
#[derive(Debug, sqlx::FromRow)]
pub struct SajuCardRow {
    pub id: Uuid,
    pub birth_hmac: String,
    pub ilju_name: String,
    pub ilju_hanja: String,
    pub keywords: serde_json::Value,
    pub lucky_element: String,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 60갑자 회귀 테스트: ilju_name()이 천간+지지 조합을 정확히 반환하는지 검증
    #[test]
    fn test_ilju_name_all_60_ganji() {
        let expected_60 = [
            "갑자", "을축", "병인", "정묘", "무진", "기사", "경오", "신미", "임신", "계유",
            "갑술", "을해", "병자", "정축", "무인", "기묘", "경진", "신사", "임오", "계미",
            "갑신", "을유", "병술", "정해", "무자", "기축", "경인", "신묘", "임진", "계사",
            "갑오", "을미", "병신", "정유", "무술", "기해", "경자", "신축", "임인", "계묘",
            "갑진", "을사", "병오", "정미", "무신", "기유", "경술", "신해", "임자", "계축",
            "갑인", "을묘", "병진", "정사", "무오", "기미", "경신", "신유", "임술", "계해",
        ];

        for (i, expected) in expected_60.iter().enumerate() {
            let stem_idx = (i % 10) as u8;
            let branch_idx = (i % 12) as u8;
            let fp = FourPillars {
                year_stem: 0, year_branch: 0,
                month_stem: 0, month_branch: 0,
                day_stem: stem_idx, day_branch: branch_idx,
                hour_stem: None, hour_branch: None,
            };
            assert_eq!(
                fp.ilju_name(), *expected,
                "60갑자 #{}: expected {} but got {}", i, expected, fp.ilju_name()
            );
            // ilju_key()는 ilju_name()과 동일해야 함
            assert_eq!(fp.ilju_key(), fp.ilju_name(), "ilju_key != ilju_name at #{}", i);
        }
    }

    /// ilju_hanja()가 한자 조합을 정확히 반환하는지 검증 (첫 번째와 마지막)
    #[test]
    fn test_ilju_hanja_boundaries() {
        let fp_first = FourPillars {
            year_stem: 0, year_branch: 0,
            month_stem: 0, month_branch: 0,
            day_stem: 0, day_branch: 0,
            hour_stem: None, hour_branch: None,
        };
        assert_eq!(fp_first.ilju_hanja(), "甲子");

        let fp_last = FourPillars {
            year_stem: 0, year_branch: 0,
            month_stem: 0, month_branch: 0,
            day_stem: 9, day_branch: 11,
            hour_stem: None, hour_branch: None,
        };
        assert_eq!(fp_last.ilju_hanja(), "癸亥");
    }
}

/// Helper: element Korean name from Element enum
pub fn element_korean(e: Element) -> &'static str {
    match e {
        Element::Wood => "목",
        Element::Fire => "화",
        Element::Earth => "토",
        Element::Metal => "금",
        Element::Water => "수",
    }
}

/// Helper: element Hanja from Element enum
pub fn element_hanja(e: Element) -> &'static str {
    match e {
        Element::Wood => "木",
        Element::Fire => "火",
        Element::Earth => "土",
        Element::Metal => "金",
        Element::Water => "水",
    }
}
