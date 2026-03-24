import Link from "next/link";
import AppDownloadBanner from "@/components/AppDownloadBanner";
import { generateJsonLd } from "@/lib/seo";

const SITE_URL = process.env.NEXT_PUBLIC_SITE_URL || "https://saju.app";

const landingJsonLd = generateJsonLd({
  type: "WebPage",
  name: "사주 - AI 사주 상담 | 무료 사주 카드",
  description:
    "AI가 분석하는 나만의 사주 카드를 무료로 만들어보세요. 3-Layer 만세력 엔진 기반 정확한 사주 분석.",
  url: SITE_URL,
});

export default function LandingPage() {
  return (
    <>
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{ __html: JSON.stringify(landingJsonLd) }}
      />

      {/* ===== Hero Section ===== */}
      <section className="relative overflow-hidden">
        {/* Background decoration */}
        <div className="absolute inset-0 bg-gradient-to-b from-primary/[0.03] to-transparent pointer-events-none" />
        <div
          className="absolute top-20 right-0 w-72 h-72 rounded-full opacity-20 blur-3xl pointer-events-none"
          style={{ backgroundColor: "#D4A574" }}
        />
        <div
          className="absolute bottom-0 left-0 w-96 h-96 rounded-full opacity-10 blur-3xl pointer-events-none"
          style={{ backgroundColor: "#1A1A2E" }}
        />

        <div className="section-container relative py-20 sm:py-28 lg:py-36 text-center">
          {/* Badge */}
          <div className="inline-flex items-center gap-1.5 px-3 py-1.5 mb-6 bg-accent/10 rounded-full animate-fade-in">
            <div className="w-1.5 h-1.5 rounded-full bg-accent animate-pulse" />
            <span className="text-xs font-medium text-accent">
              3-Layer 만세력 엔진 기반
            </span>
          </div>

          <h1 className="text-display sm:text-4xl lg:text-5xl font-bold text-primary leading-tight animate-fade-in-up">
            나의 사주를
            <br className="sm:hidden" />
            {" "}알아보세요
          </h1>

          <p className="mt-4 text-body sm:text-lg text-secondary-text max-w-md mx-auto animate-fade-in-up">
            AI가 분석하는 나만의 사주 카드를
            <br />
            지금 바로 무료로 만들어 보세요
          </p>

          <div className="mt-8 animate-fade-in-up">
            <Link href="/card" className="cta-primary text-base">
              생년월일 입력 시작
              <svg
                className="w-5 h-5 ml-1.5"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
              >
                <line x1="5" y1="12" x2="19" y2="12" />
                <polyline points="12 5 19 12 12 19" />
              </svg>
            </Link>
          </div>

          {/* Stats */}
          <div className="mt-12 flex items-center justify-center gap-8 sm:gap-12 text-center animate-fade-in">
            <div>
              <div className="text-xl sm:text-2xl font-bold text-primary">
                60
              </div>
              <div className="text-xs text-secondary-text mt-0.5">
                일주 유형
              </div>
            </div>
            <div className="w-px h-8 bg-divider" />
            <div>
              <div className="text-xl sm:text-2xl font-bold text-primary">
                AI
              </div>
              <div className="text-xs text-secondary-text mt-0.5">
                대화형 상담
              </div>
            </div>
            <div className="w-px h-8 bg-divider" />
            <div>
              <div className="text-xl sm:text-2xl font-bold text-primary">
                무료
              </div>
              <div className="text-xs text-secondary-text mt-0.5">
                사주 카드
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* ===== How It Works ===== */}
      <section className="py-16 sm:py-20 bg-white">
        <div className="section-container">
          <h2 className="text-title sm:text-2xl font-bold text-primary text-center mb-12">
            간단한 3단계로 사주 카드 완성
          </h2>

          <div className="grid grid-cols-1 sm:grid-cols-3 gap-8 sm:gap-6">
            {[
              {
                step: "01",
                icon: (
                  <svg className="w-7 h-7" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round">
                    <rect x="3" y="4" width="18" height="18" rx="2" ry="2" />
                    <line x1="16" y1="2" x2="16" y2="6" />
                    <line x1="8" y1="2" x2="8" y2="6" />
                    <line x1="3" y1="10" x2="21" y2="10" />
                  </svg>
                ),
                title: "생년월일 입력",
                desc: "양력 또는 음력으로 생년월일시를 입력하세요",
              },
              {
                step: "02",
                icon: (
                  <svg className="w-7 h-7" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round">
                    <path d="M12 2a10 10 0 1 0 10 10H12V2z" />
                    <path d="M20.66 7A10 10 0 0 0 7 20.66L12 12l8.66-5z" />
                  </svg>
                ),
                title: "AI가 사주 분석",
                desc: "3-Layer 만세력 엔진이 정확한 사주팔자를 계산합니다",
              },
              {
                step: "03",
                icon: (
                  <svg className="w-7 h-7" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round">
                    <rect x="2" y="3" width="20" height="14" rx="2" ry="2" />
                    <line x1="8" y1="21" x2="16" y2="21" />
                    <line x1="12" y1="17" x2="12" y2="21" />
                  </svg>
                ),
                title: "사주 카드 생성",
                desc: "나만의 귀여운 Lottie풍 사주 카드를 공유하세요",
              },
            ].map((item) => (
              <div
                key={item.step}
                className="text-center p-6 rounded-card hover:bg-surface/50 transition-colors"
              >
                <div className="inline-flex items-center justify-center w-14 h-14 rounded-full bg-accent/10 text-accent mb-4">
                  {item.icon}
                </div>
                <div className="text-xs font-bold text-accent mb-2">
                  STEP {item.step}
                </div>
                <h3 className="text-lg font-semibold text-primary mb-2">
                  {item.title}
                </h3>
                <p className="text-sm text-secondary-text leading-relaxed">
                  {item.desc}
                </p>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* ===== Social Proof / Engine ===== */}
      <section className="py-16 sm:py-20">
        <div className="section-container text-center">
          <div className="inline-flex items-center gap-2 px-4 py-2 bg-success/10 rounded-full mb-6">
            <svg
              className="w-4 h-4 text-success"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            >
              <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
              <polyline points="22 4 12 14.01 9 11.01" />
            </svg>
            <span className="text-sm font-medium text-success">
              정확한 분석
            </span>
          </div>

          <h2 className="text-title sm:text-2xl font-bold text-primary mb-4">
            3-Layer 만세력 엔진으로
            <br />
            정확한 사주 분석
          </h2>

          <p className="text-body text-secondary-text max-w-lg mx-auto mb-10">
            단순한 생년월일 계산이 아닌, 전통 명리학의 만세력 체계를
            3단계로 검증하여 정확한 사주팔자를 도출합니다.
          </p>

          {/* Engine layers */}
          <div className="grid grid-cols-1 sm:grid-cols-3 gap-4 max-w-2xl mx-auto">
            {[
              {
                layer: "Layer 1",
                title: "만세력 계산",
                desc: "천간/지지 정밀 계산",
                color: "#D4A574",
              },
              {
                layer: "Layer 2",
                title: "오행 분석",
                desc: "목화토금수 균형 분석",
                color: "#4A7C59",
              },
              {
                layer: "Layer 3",
                title: "AI 해석",
                desc: "맥락 기반 AI 상담",
                color: "#3D5A80",
              },
            ].map((item) => (
              <div
                key={item.layer}
                className="card-surface p-5 text-center"
              >
                <div
                  className="text-xs font-bold mb-2"
                  style={{ color: item.color }}
                >
                  {item.layer}
                </div>
                <h3 className="font-semibold text-primary text-sm">
                  {item.title}
                </h3>
                <p className="text-xs text-secondary-text mt-1">
                  {item.desc}
                </p>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* ===== Sample Card Preview ===== */}
      <section className="py-16 sm:py-20 bg-white">
        <div className="section-container">
          <h2 className="text-title sm:text-2xl font-bold text-primary text-center mb-4">
            무료 사주 카드 미리보기
          </h2>
          <p className="text-body text-secondary-text text-center mb-10">
            나만의 사주 카드를 만들고 친구에게 공유하세요
          </p>

          {/* Sample cards grid */}
          <div className="grid grid-cols-2 sm:grid-cols-4 gap-4 max-w-2xl mx-auto">
            {[
              { name: "갑목", hanja: "甲木", keywords: ["리더십", "진취"], color: "#4A7C59" },
              { name: "병화", hanja: "丙火", keywords: ["열정", "활력"], color: "#C75C3B" },
              { name: "무토", hanja: "戊土", keywords: ["안정", "신뢰"], color: "#B8956A" },
              { name: "임수", hanja: "壬水", keywords: ["지혜", "유연"], color: "#3D5A80" },
            ].map((card) => (
              <div
                key={card.name}
                className="card-surface overflow-hidden group hover:shadow-md transition-shadow"
              >
                <div
                  className="aspect-[3/4] flex flex-col items-center justify-center p-4"
                  style={{
                    background: `linear-gradient(135deg, ${card.color}08, ${card.color}15)`,
                  }}
                >
                  <div
                    className="w-12 h-12 sm:w-16 sm:h-16 rounded-full flex items-center justify-center mb-3 transition-transform group-hover:scale-110"
                    style={{ backgroundColor: `${card.color}20` }}
                  >
                    <span
                      className="font-hanja text-xl sm:text-2xl font-bold"
                      style={{ color: card.color }}
                    >
                      {card.hanja.charAt(0)}
                    </span>
                  </div>
                  <p
                    className="font-hanja text-sm font-bold"
                    style={{ color: card.color }}
                  >
                    {card.hanja}
                  </p>
                  <p className="text-xs text-on-surface font-medium mt-0.5">
                    {card.name}
                  </p>
                  <div className="flex gap-1 mt-2">
                    {card.keywords.map((kw) => (
                      <span
                        key={kw}
                        className="text-[10px] px-1.5 py-0.5 rounded-full"
                        style={{
                          backgroundColor: `${card.color}15`,
                          color: card.color,
                        }}
                      >
                        {kw}
                      </span>
                    ))}
                  </div>
                </div>
              </div>
            ))}
          </div>

          <div className="text-center mt-8">
            <Link href="/card" className="cta-primary">
              나의 사주 카드 만들기
            </Link>
          </div>
        </div>
      </section>

      {/* ===== App Features ===== */}
      <section className="py-16 sm:py-20">
        <div className="section-container">
          <h2 className="text-title sm:text-2xl font-bold text-primary text-center mb-4">
            앱에서 더 알아보기
          </h2>
          <p className="text-body text-secondary-text text-center mb-10">
            무료 사주 카드를 넘어, 깊이 있는 AI 사주 상담을 경험하세요
          </p>

          <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 max-w-2xl mx-auto mb-10">
            {[
              {
                icon: (
                  <svg className="w-6 h-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round">
                    <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
                  </svg>
                ),
                title: "AI 대화형 상담",
                desc: "사주 결과를 보고 궁금한 점을 바로 질문하세요. 이직, 연애, 건강 등 맞춤 상담.",
                price: "15,000원",
              },
              {
                icon: (
                  <svg className="w-6 h-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round">
                    <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" />
                  </svg>
                ),
                title: "궁합 분석",
                desc: "두 사람의 사주를 비교 분석합니다. 오행 궁합과 상성을 한눈에 확인.",
                price: "12,000원",
              },
              {
                icon: (
                  <svg className="w-6 h-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round">
                    <circle cx="12" cy="12" r="10" />
                    <line x1="2" y1="12" x2="22" y2="12" />
                    <path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z" />
                  </svg>
                ),
                title: "매일 운세",
                desc: "내 일주에 맞는 오늘의 운세를 매일 아침 확인하세요.",
                price: "무료",
              },
              {
                icon: (
                  <svg className="w-6 h-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round">
                    <rect x="2" y="3" width="20" height="14" rx="2" ry="2" />
                    <line x1="8" y1="21" x2="16" y2="21" />
                    <line x1="12" y1="17" x2="12" y2="21" />
                  </svg>
                ),
                title: "Lottie풍 AI 이미지",
                desc: "NanoBanana AI가 생성한 귀여운 Lottie풍 일러스트 결과물.",
                price: "포함",
              },
            ].map((feature) => (
              <div
                key={feature.title}
                className="card-surface p-5 flex gap-4"
              >
                <div className="w-11 h-11 rounded-lg bg-accent/10 text-accent flex items-center justify-center shrink-0">
                  {feature.icon}
                </div>
                <div className="flex-1">
                  <div className="flex items-center gap-2">
                    <h3 className="font-semibold text-primary text-sm">
                      {feature.title}
                    </h3>
                    <span className="text-xs text-accent font-medium">
                      {feature.price}
                    </span>
                  </div>
                  <p className="text-xs text-secondary-text mt-1 leading-relaxed">
                    {feature.desc}
                  </p>
                </div>
              </div>
            ))}
          </div>

          <AppDownloadBanner />
        </div>
      </section>

      {/* ===== Final CTA ===== */}
      <section className="py-16 sm:py-20 bg-primary text-surface">
        <div className="section-container text-center">
          <h2 className="text-title sm:text-2xl font-bold mb-3">
            지금 바로 나의 사주를 확인하세요
          </h2>
          <p className="text-surface/60 mb-8">
            무료로 시작하세요. 설치 없이 바로 확인할 수 있습니다.
          </p>
          <Link
            href="/card"
            className="inline-flex items-center justify-center h-[52px] px-8 bg-accent text-primary
              font-semibold rounded-button transition-all duration-300
              hover:brightness-110 hover:shadow-lg hover:shadow-accent/30"
          >
            무료 사주 카드 만들기
            <svg
              className="w-5 h-5 ml-1.5"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            >
              <line x1="5" y1="12" x2="19" y2="12" />
              <polyline points="12 5 19 12 12 19" />
            </svg>
          </Link>
        </div>
      </section>
    </>
  );
}
