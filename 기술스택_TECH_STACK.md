# 사주 앱 기술 스택

> **최종 수정**: 2026-03-24
> **서비스 형태**: 모바일 앱 전용 (웹 없음)

---

## 핵심 스택 요약

| 영역 | 기술 | 비고 |
|------|------|------|
| **Backend** | Rust (Axum) | API 서버 + 사주 연산 엔진 |
| **App** | Flutter | iOS + Android 동시 지원 |
| **DB (운영)** | PostgreSQL | Supabase 호스팅 권장 |
| **DB (개발)** | Docker PostgreSQL | 로컬 개발/테스트용 (프로덕션과 동일 DB) |
| **상태관리** | Riverpod | Flutter 표준, 비동기 처리 우수 |
| **인증** | Kakao + Apple + Google | Apple 필수(앱스토어 정책) |
| **결제** | RevenueCat (IAP 래퍼) | iOS/Android 인앱결제 통합 |
| **호스팅** | Fly.io → AWS 전환 | 초기 단순화, 스케일 시 마이그레이션 |
| **푸시** | FCM (Firebase Cloud Messaging) | iOS/Android 통합 |
| **AI (서비스)** | Anthropic Claude Opus 4.6 (per-token) | 사주 해석 + AI 채팅 + 오늘의 운세 |
| **이미지/GIF** | NanoBanana API | Lottie풍 AI 이미지 생성 (~27원/장) |
| **분석** | Firebase Analytics + Mixpanel | 기본 지표 + 퍼널 분석 |

---

## 1. Backend — Rust (Axum)

### 왜 Axum인가
- Rust 웹 프레임워크 중 **tokio 공식 생태계** (장기 유지보수 안정성)
- Actix-web 대비 러닝커브 낮고 타입 안전성 우수
- Tower 미들웨어 생태계 활용 가능 (인증, 로깅, rate limiting)

### 서버 역할
```
[Flutter App] ←→ REST API (Axum) ←→ PostgreSQL
                      ↓
               사주 연산 엔진 (Rust 네이티브)
                      ↓
               해석 생성 (Anthropic Claude API)
                      ↓
               이미지/GIF 생성 (NanoBanana API)
```

### 주요 크레이트
| 크레이트 | 용도 |
|---------|------|
| `axum` | HTTP 서버 프레임워크 (SSE 스트리밍 내장 — 사주 해석 실시간 전송) |
| `sqlx` | PostgreSQL 비동기 드라이버 (컴파일 타임 쿼리 검증) |
| `serde` | JSON 직렬화/역직렬화 |
| `jsonwebtoken` | JWT 인증 토큰 |
| `tower-http` | CORS, 로깅, 압축 미들웨어 |
| `chrono` | 날짜/시간 기본 연산 (양력). 음력 변환은 자체 변환 테이블 구현 필요 (chrono에 음력 기능 없음) |
| `reqwest` | 외부 API 호출용 HTTP 클라이언트 (Anthropic Claude API + NanoBanana) |
| `tracing` | 구조화 로깅 |

---

## 2. 사주 연산 엔진 (Rust 네이티브)

### 아키텍처: 연산과 해석 분리

청월당의 가장 큰 약점이 **"목 과다" vs "목 부족" 같은 분석 모순**이었음.
이를 방지하기 위해 **연산 계층과 해석 계층을 철저히 분리**.

```
┌─────────────────────────────────────────┐
│  Layer 1: 만세력 연산 (Rust, 결정론적)      │
│  - 양력→음력 변환 (자체 변환 테이블 구현)    │
│  - 24절기 절입시각 (한국천문연구원 데이터)    │
│  - 천간/지지 계산 (연주, 월주, 일주, 시주)   │
│  - 오행 배분 및 점수화                      │
│  - 십신 관계 매핑                          │
│  - 대운/세운/월운 계산                      │
│  - 신살 판별                              │
│  출력: 구조화된 JSON (숫자, enum 값)        │
│                                           │
│  ⚠️ 주의 구현 사항:                        │
│  - 야자시(23:00~01:00) 처리: 야자시론 기본   │
│    + 조자시론 옵션 (사용자 선택 가능)        │
│  - 절기 기반 월주 계산: 음력 월이 아님       │
│  - 대운 시작 나이: 남녀 순행/역행 구분       │
│  - 서머타임 보정 (1948-1988 한국 적용)       │
└──────────────┬──────────────────────────┘
               ↓
┌─────────────────────────────────────────┐
│  Layer 2: 검증 계층 (Rust, 규칙 기반)       │
│  - 오행 과다/부족 판정을 수치로 확정          │
│  - 모순 탐지: 같은 오행에 상반된 판정 불가     │
│  - 결과 일관성 보장                         │
│  출력: 검증된 분석 데이터                    │
└──────────────┬──────────────────────────┘
               ↓
┌─────────────────────────────────────────┐
│  Layer 3: 해석 생성 (Anthropic Claude API)    │
│  - 검증된 데이터를 프롬프트에 주입            │
│  - "목 점수 8/10 (과다)" 같은 확정값 전달    │
│  - LLM은 수치를 자연어로 변환만 담당          │
│  - 생성 후 재검증: 원본 데이터와 불일치 탐지   │
│  출력: 사용자용 해석 텍스트                  │
└─────────────────────────────────────────┘
```

### 왜 이 구조인가
- Layer 1은 **결정론적** — 같은 입력에 항상 같은 결과 (테스트 가능)
- Layer 2가 **모순 방지 게이트** 역할 — 청월당이 비판받은 핵심 문제 해결
- Layer 3의 LLM은 **"번역기"** 역할만 — 분석 판단을 LLM에 위임하지 않음

---

## 2.5. AI & 이미지 생성

### AI LLM — Anthropic Claude Opus 4.6

| 항목 | 내용 |
|------|------|
| **서비스** | Anthropic Claude API (https://console.anthropic.com) |
| **요금제** | per-token 과금 — 입력 $15/MTok, 출력 $75/MTok (Opus 4.6) |
| **장점** | 한국어 품질 최상, 세션 제한 없음, 안정적 API SLA |
| **용도** | Layer 3 해석 생성 + AI 채팅 상담 + 오늘의 운세 |
| **모델** | Claude Opus 4.6 (서비스용) |
| **비용 관리** | per-message 토큰캡 + per-session 비용캡 설계 필요 (TODOS.md [P1] 참조) |
| **폴백 전략** | Claude API 장애 시 → OpenAI GPT-4o 전환. 서버 추상화 레이어로 LLM 교체 용이하게 설계 |

### 이미지/GIF 생성 — NanoBanana API

| 항목 | 내용 |
|------|------|
| **서비스** | NanoBanana API (Google Gemini 기반) |
| **가격** | ~$0.02/장 (~27원), 무료 티어 포함 |
| **장점** | GIF 생성 지원, 빠른 추론 (<10초), 4K 품질 |
| **용도** | Phase 1부터 — 사주 카드 이미지, 유료 결과 이미지/GIF, 바이럴 카드 |
| **스타일** | Lottie풍 — 귀엽고 플랫한 벡터 일러스트, 파스텔 톤, 동양적 모티프 |
| **캐싱** | 동일 생년월일 카드는 캐시 반환 (재생성 비용 0원) |

---

## 3. App — Flutter + Riverpod

### 왜 Riverpod인가
- Provider의 진화판, Flutter 커뮤니티 표준
- **코드 생성 기반** (`@riverpod`) — 보일러플레이트 최소화
- 비동기 상태(API 호출, 로딩, 에러) 처리가 깔끔
- 테스트 시 의존성 오버라이드 용이

### 주요 패키지
| 패키지 | 용도 |
|--------|------|
| `flutter_riverpod` | 상태관리 |
| `dio` | HTTP 클라이언트 (인터셉터, 리트라이) |
| `go_router` | 선언적 라우팅 |
| `flutter_secure_storage` | JWT 토큰 안전 저장 |
| `kakao_flutter_sdk` | 카카오 로그인 + 카카오 Link (카드 공유) |
| `sign_in_with_apple` | Apple 로그인 |
| `google_sign_in` | Google 로그인 |
| `purchases_flutter` | RevenueCat 인앱결제 |
| `firebase_messaging` | 푸시 알림 |
| `firebase_analytics` | 사용자 분석 |
| `lottie` | 로딩/UI 전환 애니메이션 (사주 결과 이미지는 NanoBanana가 생성) |
| `cached_network_image` | 이미지 캐싱 |
| `firebase_dynamic_links` | 딥링크 (공유 카드 → 앱/스토어 유도) |
| `share_plus` | 시스템 공유 시트 (이미지 + URL) |

### 폴더 구조 (Feature-first)
```
lib/
├── core/              # 공통 유틸, 테마, 상수
│   ├── theme/
│   ├── network/       # Dio 설정, 인터셉터
│   └── constants/
├── features/
│   ├── auth/          # 로그인/회원가입
│   ├── saju/          # 사주 입력 → 결과 조회
│   ├── chat/          # AI 채팅 상담 (월하선생)
│   ├── compatibility/ # 궁합 미리보기 + 유료 궁합
│   ├── daily_fortune/ # 오늘의 운세 + iOS 위젯
│   ├── profile/       # 평생 사주 프로필 + 오행 차트
│   ├── payment/       # 결제 플로우
│   ├── history/       # 내 분석 기록
│   └── settings/      # 설정, 캐릭터 선택, 알림
├── shared/            # 공용 위젯, 모델
└── main.dart
```

---

## 4. 인증

### 필수 로그인 수단
| 방식 | 이유 |
|------|------|
| **카카오 로그인** | 한국 사용자 90%+ 카카오톡 사용 |
| **Apple 로그인** | App Store 정책 필수 (소셜 로그인 제공 시) |
| **Google 로그인** | Android 사용자 편의 |

### 인증 플로우
```
App → 소셜 로그인 SDK → 소셜 토큰 획득
   → Rust 서버 전송 → 소셜 토큰 검증
   → 자체 JWT 발급 (access + refresh)
   → App에서 secure storage 저장
```

---

## 5. 결제 — RevenueCat

### 왜 RevenueCat인가
- iOS/Android **인앱결제(IAP)를 단일 API로 통합**
- 영수증 검증을 서버 없이 처리
- 구독/일회성 구매 모두 지원
- 대시보드에서 매출, 이탈, LTV 추적
- Flutter SDK 공식 지원

### 결제 모델 (확정 — DESIGN.md와 통일)
| 상품 | IAP 가격 | 개발자 수령 (70%) | 설명 |
|------|----------|------------------|------|
| AI 사주 상담 | ₩15,000 | ₩10,500 | 종합 분석 + AI 채팅 (72시간, 50턴) |
| 궁합 상담 (Phase 1) | ₩12,000 | ₩8,400 | 두 사람 궁합 + AI 채팅 (무료 미리보기 포함) |
| 구독 모델 (v2) | 미정 | — | 월운 + 오늘의 운세 + 알림 |

---

## 6. 인프라 & 배포

### 초기 (MVP ~ 사용자 1만명)
```
Fly.io
├── Rust API 서버 (2 인스턴스 + 로드밸런싱)
└── Supabase (호스팅 PostgreSQL + Storage)
```
- Fly.io: Rust 바이너리 Docker 배포, 자동 TLS, 한국 리전 없지만 도쿄 리전 사용
- Supabase: PostgreSQL + 파일 스토리지 + 실시간 기능 (무료 티어로 시작)

### 스케일 단계 (사용자 1만명+)
```
AWS (서울 리전)
├── ECS Fargate (Rust API 컨테이너)
├── RDS PostgreSQL (Multi-AZ)
├── S3 + CloudFront (이미지/에셋)
├── ElastiCache Redis (세션, 캐싱)
└── CloudWatch (모니터링)
```

---

## 7. 분석 & 모니터링

| 도구 | 용도 | 단계 |
|------|------|------|
| **Firebase Analytics** | DAU, 리텐션, 기본 이벤트 | 초기부터 |
| **Mixpanel** | 퍼널 분석 (결제 전환율 등) | 초기부터 |
| **Sentry** | 앱 + 서버 크래시, 에러 트래킹 | 초기부터 |
| **Grafana + Prometheus** | 서버 메트릭 모니터링 | 스케일 단계 |

---

## 8. CI/CD

| 영역 | 도구 |
|------|------|
| **Backend** | GitHub Actions → Docker build → Fly.io deploy |
| **App** | GitHub Actions → Fastlane → App Store / Play Store |
| **테스트** | `cargo test` (Rust) + `flutter test` (Dart) |
| **코드 품질** | `clippy` (Rust linter) + `dart analyze` |

---

## 8.5. 개발 도구

| 도구 | 용도 |
|------|------|
| **Anthropic Opus 4.6 (thinking high)** | 개발/빌드 시 AI 보조 — 프롬프트 엔지니어링, 코드 생성, 리뷰 |

---

## 9. 개발 단계별 DB 전략

### Phase 1: 로컬 개발
```toml
# Rust - sqlx 설정
[dev]
database_url = "postgres://saju:saju@localhost:5432/saju_dev"
```
- Docker PostgreSQL로 프로덕션과 동일 환경 사용 (`docker run -e POSTGRES_DB=saju_dev -e POSTGRES_USER=saju -e POSTGRES_PASSWORD=saju -p 5432:5432 postgres:16`)
- JSONB, NOW() 등 PostgreSQL 전용 기능을 로컬에서도 검증 가능 — 스테이징/프로덕션과의 문법 차이 문제 없음

### Phase 2: 스테이징
- Supabase 무료 PostgreSQL 인스턴스
- 실제 PostgreSQL 동작 검증

### Phase 3: 프로덕션
- Supabase Pro 또는 AWS RDS PostgreSQL
- 자동 백업, Point-in-time Recovery

### 마이그레이션 도구
- `sqlx-cli` — Rust 네이티브 마이그레이션 관리
```bash
sqlx migrate add create_users
sqlx migrate run
```

---

## 10. 보안 체크리스트

- [ ] JWT 시크릿 환경변수 관리 (하드코딩 금지)
- [ ] API rate limiting (Tower 미들웨어)
- [ ] HTTPS 강제 (Fly.io 자동 TLS)
- [ ] 사용자 생년월일 등 개인정보 암호화 저장
- [ ] LLM API 키 (Anthropic Claude) 서버사이드만 보관 (앱에 노출 금지)
- [ ] NanoBanana API 키 서버사이드만 보관 (앱에 노출 금지)
- [ ] SQL injection 방지 (sqlx 파라미터 바인딩)
- [ ] App Transport Security (iOS)
- [ ] ProGuard/R8 난독화 (Android)
