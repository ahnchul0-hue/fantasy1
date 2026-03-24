/// Layer 2: 검증 계층 (Validation & Analysis Layer)
///
/// Performs: 오행 balance, 십신, 대운, consistency validation.
/// All deterministic and rule-based. No AI.

use crate::models::birth::{BirthInput, Gender};
use crate::models::saju::*;
use crate::saju::tables::{self, HEAVENLY_STEMS, EARTHLY_BRANCHES, Pillar};
use crate::saju::tables::heavenly_stems::Element;

pub struct SajuAnalyzer;

impl SajuAnalyzer {
    pub fn new() -> Self { Self }

    pub fn analyze(&self, fp: &FourPillars, input: &BirthInput) -> SajuAnalysis {
        let dm_idx = fp.day_stem;
        let dm = &HEAVENLY_STEMS[dm_idx as usize];

        let oheng = self.calculate_oheng_balance(fp);
        let normalized = oheng.normalized();
        let strength = self.evaluate_day_master_strength(dm.element, &oheng);
        let lucky = self.determine_lucky_element(dm.element, &strength, &normalized);
        let ten_gods = self.calculate_ten_gods(fp);
        let daeun = self.calculate_daeun(fp, input);

        let element_statuses: Vec<(String, String)> = [
            Element::Wood, Element::Fire, Element::Earth, Element::Metal, Element::Water
        ].iter().map(|e| {
            (element_korean(*e).to_string(), oheng.element_status(*e).korean().to_string())
        }).collect();

        let keywords = self.generate_keywords(dm.element, &strength, &ten_gods);

        let analysis = SajuAnalysis {
            four_pillars: fp.clone(),
            oheng_balance: oheng,
            normalized_oheng: normalized,
            day_master_index: dm_idx,
            day_master_strength: strength,
            lucky_element: element_korean(lucky).to_string(),
            ten_gods,
            daeun,
            keywords,
            element_statuses,
        };

        self.validate_consistency(&analysis);
        analysis
    }

    fn calculate_oheng_balance(&self, fp: &FourPillars) -> OhengBalance {
        let mut balance = OhengBalance::new();

        let stems: Vec<u8> = {
            let mut v = vec![fp.year_stem, fp.month_stem, fp.day_stem];
            if let Some(s) = fp.hour_stem { v.push(s); }
            v
        };
        let branches: Vec<u8> = {
            let mut v = vec![fp.year_branch, fp.month_branch, fp.day_branch];
            if let Some(b) = fp.hour_branch { v.push(b); }
            v
        };

        for &s in &stems {
            balance.add(HEAVENLY_STEMS[s as usize].element, 1.0);
        }
        for &b in &branches {
            balance.add(EARTHLY_BRANCHES[b as usize].element, 0.7);
        }

        balance
    }

    fn evaluate_day_master_strength(&self, dm_element: Element, balance: &OhengBalance) -> ElementStatus {
        let normalized = balance.normalized();
        let dm_score = normalized.get(dm_element);
        if dm_score >= 3.0 { ElementStatus::Excessive }
        else if dm_score <= 1.0 { ElementStatus::Deficient }
        else { ElementStatus::Balanced }
    }

    fn determine_lucky_element(&self, dm_element: Element, strength: &ElementStatus, normalized: &OhengBalance) -> Element {
        match strength {
            ElementStatus::Excessive => {
                let gen = generates_elem(dm_element);
                let over = overcomes_elem(dm_element);
                if normalized.get(gen) < normalized.get(over) { gen } else { over }
            }
            ElementStatus::Deficient => {
                // Need element that generates day master
                let all = [Element::Wood, Element::Fire, Element::Earth, Element::Metal, Element::Water];
                *all.iter().find(|e| generates_elem(**e) == dm_element).unwrap_or(&dm_element)
            }
            ElementStatus::Balanced => normalized.lucky_element(),
        }
    }

    fn calculate_ten_gods(&self, fp: &FourPillars) -> Vec<(String, String)> {
        let dm = fp.day_stem;
        let mut result = Vec::new();
        result.push(("year_stem".to_string(), TenGod::from_relationship(dm, fp.year_stem).korean().to_string()));
        result.push(("month_stem".to_string(), TenGod::from_relationship(dm, fp.month_stem).korean().to_string()));
        if let Some(h) = fp.hour_stem {
            result.push(("hour_stem".to_string(), TenGod::from_relationship(dm, h).korean().to_string()));
        }
        result
    }

    fn calculate_daeun(&self, fp: &FourPillars, input: &BirthInput) -> Vec<DaeunPeriod> {
        let year_stem = &HEAVENLY_STEMS[fp.year_stem as usize];
        let is_yang = year_stem.polarity == crate::saju::tables::heavenly_stems::Polarity::Yang;
        let is_male = input.gender == Gender::Male;
        let forward = (is_male && is_yang) || (!is_male && !is_yang);

        let start_age = 4i32;
        let mut periods = Vec::new();

        for i in 0..8i32 {
            let age = start_age + i * 10;
            let offset = if forward { i + 1 } else { -(i + 1) };
            let stem_idx = ((fp.month_stem as i32 + offset) % 10 + 10) % 10;
            let branch_idx = ((fp.month_branch as i32 + offset) % 12 + 12) % 12;
            periods.push(DaeunPeriod {
                start_age: age,
                stem_index: stem_idx as u8,
                branch_index: branch_idx as u8,
            });
        }
        periods
    }

    fn generate_keywords(&self, dm_element: Element, strength: &ElementStatus, ten_gods: &[(String, String)]) -> Vec<String> {
        let mut kw = Vec::new();
        kw.push(match dm_element {
            Element::Wood => "성장", Element::Fire => "열정", Element::Earth => "안정",
            Element::Metal => "결단", Element::Water => "지혜",
        }.to_string());
        kw.push(match strength {
            ElementStatus::Excessive => "자신감", ElementStatus::Balanced => "조화",
            ElementStatus::Deficient => "섬세함",
        }.to_string());
        if let Some((_, god)) = ten_gods.first() {
            kw.push(match god.as_str() {
                "비견" => "독립", "겁재" => "경쟁", "식신" => "창의", "상관" => "표현",
                "편재" => "재능", "정재" => "안정", "편관" => "리더십", "정관" => "책임",
                "편인" => "학습", "정인" => "배려", _ => "조화",
            }.to_string());
        }
        kw.truncate(3);
        kw
    }

    fn validate_consistency(&self, analysis: &SajuAnalysis) {
        let sum = analysis.normalized_oheng.total();
        if (sum - 10.0).abs() > 1.0 {
            tracing::warn!("Oheng normalized total ({}) deviates from 10", sum);
        }
    }

    /// Calculate compatibility between two sets of four pillars.
    pub fn calculate_compatibility(&self, p1: &FourPillars, p2: &FourPillars) -> (i32, String) {
        let dm1 = &HEAVENLY_STEMS[p1.day_stem as usize];
        let dm2 = &HEAVENLY_STEMS[p2.day_stem as usize];
        let mut score: f64 = 50.0;

        if generates_elem(dm1.element) == dm2.element || generates_elem(dm2.element) == dm1.element {
            score += 20.0;
        } else if overcomes_elem(dm1.element) == dm2.element || overcomes_elem(dm2.element) == dm1.element {
            score -= 10.0;
        } else if dm1.element == dm2.element {
            score += 10.0;
        }

        if dm1.polarity != dm2.polarity { score += 10.0; }

        let final_score = score.max(0.0).min(100.0) as i32;
        let summary = if final_score >= 80 {
            format!("{}({})과 {}({})의 궁합이 매우 좋습니다.", dm1.korean, element_hanja(dm1.element), dm2.korean, element_hanja(dm2.element))
        } else if final_score >= 60 {
            format!("{}({})과 {}({})는 좋은 궁합입니다.", dm1.korean, element_hanja(dm1.element), dm2.korean, element_hanja(dm2.element))
        } else if final_score >= 40 {
            format!("{}({})과 {}({})는 보통의 궁합입니다.", dm1.korean, element_hanja(dm1.element), dm2.korean, element_hanja(dm2.element))
        } else {
            format!("{}({})과 {}({})는 노력이 필요한 궁합입니다.", dm1.korean, element_hanja(dm1.element), dm2.korean, element_hanja(dm2.element))
        };
        (final_score, summary)
    }
}

fn generates_elem(e: Element) -> Element {
    match e {
        Element::Wood => Element::Fire, Element::Fire => Element::Earth,
        Element::Earth => Element::Metal, Element::Metal => Element::Water,
        Element::Water => Element::Wood,
    }
}

fn overcomes_elem(e: Element) -> Element {
    match e {
        Element::Wood => Element::Earth, Element::Fire => Element::Metal,
        Element::Earth => Element::Water, Element::Metal => Element::Wood,
        Element::Water => Element::Fire,
    }
}
