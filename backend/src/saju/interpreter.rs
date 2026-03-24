/// Layer 3: 해석 생성 (Interpretation Layer - Claude API)
///
/// Takes validated L1+L2 data and generates natural language interpretation.
/// The LLM acts as a "translator" only - it does NOT make analytical decisions.

use crate::errors::AppError;
use crate::models::saju::SajuAnalysis;
use crate::saju::tables::{HEAVENLY_STEMS, EARTHLY_BRANCHES};
use crate::services::claude::{ClaudeClient, ClaudeMessage};

pub struct SajuInterpreter {
    claude_client: ClaudeClient,
    max_tokens_per_message: u32,
}

impl SajuInterpreter {
    pub fn new(claude_client: ClaudeClient, max_tokens_per_message: u32) -> Self {
        Self { claude_client, max_tokens_per_message }
    }

    /// Generate full saju interpretation from L2 analysis.
    pub async fn generate_interpretation(&self, analysis: &SajuAnalysis) -> Result<String, AppError> {
        let system_prompt = self.build_system_prompt();
        let user_prompt = self.build_interpretation_prompt(analysis);

        let response = self.claude_client.send_message(
            &system_prompt,
            &[ClaudeMessage { role: "user".to_string(), content: user_prompt }],
            self.max_tokens_per_message,
        ).await?;

        self.validate_interpretation(&response, analysis)?;
        Ok(response)
    }

    /// Generate a chat response in consultation context.
    pub async fn generate_chat_response(
        &self,
        analysis_summary: &str,
        chat_history: &[ClaudeMessage],
        user_message: &str,
        turns_remaining: i32,
    ) -> Result<String, AppError> {
        let system_prompt = self.build_chat_system_prompt(analysis_summary, turns_remaining);
        let mut messages = chat_history.to_vec();
        messages.push(ClaudeMessage { role: "user".to_string(), content: user_message.to_string() });
        self.claude_client.send_message(&system_prompt, &messages, self.max_tokens_per_message).await
    }

    fn build_system_prompt(&self) -> String {
        r#"당신은 "월하선생"입니다. 전통 명리학에 깊은 조예가 있는 사주 상담 전문가입니다.

## 성격과 말투
- 존댓말을 사용합니다. "~입니다", "~하시겠습니다"
- 따뜻하지만 직설적입니다.
- 전통 명리학자가 현대어로 설명하는 느낌입니다.

## 중요 규칙
- 아래 제공되는 사주 분석 데이터는 확정된 수치입니다. 수치를 변경하지 마세요.
- "과다"로 표시된 오행을 "부족"이라고 해석하면 안 됩니다. 반대도 마찬가지입니다.
- 건강, 수명, 사망에 대한 구체적 예언은 하지 마세요.
- 사주와 관련 없는 주제는 정중히 거절하세요.

## 응답 형식
- 각 섹션(성격, 연애, 재물, 커리어, 조언)을 명확히 구분하세요.
- 각 섹션은 3-5문장으로 작성하세요."#.to_string()
    }

    fn build_interpretation_prompt(&self, analysis: &SajuAnalysis) -> String {
        let fp = &analysis.four_pillars;
        let yp = fp.year_pillar();
        let mp = fp.month_pillar();
        let dp = fp.day_pillar();

        let hour_info = match fp.hour_pillar() {
            Some(hp) => format!("시주: {}({}) {}({})", hp.stem().korean, hp.stem().hanja, hp.branch().korean, hp.branch().hanja),
            None => "시주: 미상 (출생시간 모름)".to_string(),
        };

        let dm = &HEAVENLY_STEMS[analysis.day_master_index as usize];

        let element_lines: Vec<String> = analysis.element_statuses.iter()
            .map(|(elem, status)| format!("- {}: {}", elem, status))
            .collect();

        let ten_gods_lines: Vec<String> = analysis.ten_gods.iter()
            .map(|(pos, god)| format!("- {}: {}", pos, god))
            .collect();

        let daeun_lines: Vec<String> = analysis.daeun.iter().take(4)
            .map(|d| {
                let s = &HEAVENLY_STEMS[d.stem_index as usize];
                let b = &EARTHLY_BRANCHES[d.branch_index as usize];
                format!("- {}세~{}세: {}{}({})", d.start_age, d.start_age + 10, s.korean, b.korean, s.element)
            })
            .collect();

        format!(
            r#"다음 사주 분석 데이터를 기반으로 종합 해석을 생성해주세요.

## 사주팔자 (확정 데이터)

연주: {}({}) {}({})
월주: {}({}) {}({})
일주: {}({}) {}({})
{}

## 일간 (Day Master)
- 일간: {}({})
- 일간 강도: {}
- 용신: {}

## 오행 분석
{}

## 십신 관계
{}

## 대운
{}

## 키워드
{}

---

위 데이터를 기반으로 다음 5개 섹션을 작성해주세요:
1. 성격 2. 연애운 3. 재물운 4. 커리어 5. 올해의 조언"#,
            yp.stem().korean, yp.stem().hanja, yp.branch().korean, yp.branch().hanja,
            mp.stem().korean, mp.stem().hanja, mp.branch().korean, mp.branch().hanja,
            dp.stem().korean, dp.stem().hanja, dp.branch().korean, dp.branch().hanja,
            hour_info,
            dm.korean, dm.element,
            analysis.day_master_strength.korean(),
            analysis.lucky_element,
            element_lines.join("\n"),
            ten_gods_lines.join("\n"),
            daeun_lines.join("\n"),
            analysis.keywords.join(", "),
        )
    }

    fn build_chat_system_prompt(&self, analysis_summary: &str, turns_remaining: i32) -> String {
        let base = self.build_system_prompt();
        format!(
            "{}\n\n## 사주 분석 요약\n{}\n\n## 채팅 규칙\n- 남은 대화 횟수: {}회\n- 5회 이하일 때 안내\n- 마지막 턴에서 마무리 인사",
            base, analysis_summary, turns_remaining
        )
    }

    fn validate_interpretation(&self, text: &str, analysis: &SajuAnalysis) -> Result<(), AppError> {
        for (elem, status) in &analysis.element_statuses {
            if status == "과다" && (text.contains(&format!("{} 부족", elem)) || text.contains(&format!("{}이 부족", elem))) {
                tracing::warn!("L3 validation: {} is 과다 but text says 부족", elem);
            }
            if status == "부족" && (text.contains(&format!("{} 과다", elem)) || text.contains(&format!("{}이 과다", elem))) {
                tracing::warn!("L3 validation: {} is 부족 but text says 과다", elem);
            }
        }
        Ok(())
    }

    /// Generate daily fortune for a given ilju and date.
    pub async fn generate_daily_fortune(
        &self, ilju_key: &str, date: &str, day_stem: &str, day_branch: &str,
    ) -> Result<(String, String, i32, i32), AppError> {
        let system = r#"당신은 "월하선생"입니다. 오늘의 운세를 간결하게 전달합니다.
- 3-4문장으로 짧게 작성. JSON으로 응답."#;
        let user = format!(
            "일주: {}\n날짜: {}\n오늘 일간: {}({})\n\nJSON: {{\"fortune_text\": \"...\", \"lucky_color\": \"...\", \"lucky_number\": N, \"overall_score\": N}}",
            ilju_key, date, day_stem, day_branch
        );
        let response = self.claude_client.send_message(
            system, &[ClaudeMessage { role: "user".to_string(), content: user }], 500
        ).await?;

        let parsed: serde_json::Value = serde_json::from_str(&response).unwrap_or_else(|_| serde_json::json!({}));
        Ok((
            parsed["fortune_text"].as_str().unwrap_or("오늘 하루도 좋은 일이 있을 것입니다.").to_string(),
            parsed["lucky_color"].as_str().unwrap_or("파란색").to_string(),
            parsed["lucky_number"].as_i64().unwrap_or(7) as i32,
            parsed["overall_score"].as_i64().unwrap_or(3).max(1).min(5) as i32,
        ))
    }
}
