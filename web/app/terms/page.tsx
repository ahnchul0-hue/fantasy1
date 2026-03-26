import type { Metadata } from "next";

const SITE_URL = process.env.NEXT_PUBLIC_SITE_URL || "https://saju.app";

export const metadata: Metadata = {
  title: "이용약관",
  description: "AI 사주 서비스의 이용약관입니다.",
  alternates: { canonical: `${SITE_URL}/terms` },
  openGraph: {
    title: "이용약관 | AI 사주",
    description: "AI 사주 서비스의 이용약관입니다.",
    url: `${SITE_URL}/terms`,
    type: "website",
    locale: "ko_KR",
    images: [{ url: `${SITE_URL}/og-default.png`, width: 1200, height: 630, alt: "AI 사주" }],
  },
  twitter: {
    card: "summary",
    title: "이용약관 | AI 사주",
    description: "AI 사주 서비스의 이용약관입니다.",
    images: [`${SITE_URL}/og-default.png`],
  },
};

export default function TermsPage() {
  return (
    <div className="section-container py-12">
      <div className="max-w-2xl mx-auto prose prose-sm">
        <h1 className="text-display font-bold text-primary mb-8">이용약관</h1>

        <div className="space-y-6 text-sm text-on-surface leading-relaxed">
          <section>
            <h2 className="text-title font-semibold text-primary mb-3">
              1. 서비스 개요
            </h2>
            <p>
              본 서비스는 전통 명리학과 AI 기술을 결합하여 사주 분석, 운세,
              궁합 등을 제공하는 엔터테인먼트 목적의 서비스입니다.
            </p>
          </section>

          <section>
            <h2 className="text-title font-semibold text-primary mb-3">
              2. 면책 조항
            </h2>
            <p>
              본 서비스의 사주 분석 결과는 참고용이며, 중요한 의사결정의
              근거로 사용해서는 안 됩니다. 서비스 이용으로 인한 결과에 대해
              법적 책임을 지지 않습니다.
            </p>
          </section>

          <section>
            <h2 className="text-title font-semibold text-primary mb-3">
              3. 이용 제한
            </h2>
            <p>
              서비스의 부정 이용, 자동화된 대량 요청, 서비스 방해 행위 등은
              이용이 제한될 수 있습니다.
            </p>
          </section>

          <section>
            <h2 className="text-title font-semibold text-primary mb-3">
              4. 유료 서비스
            </h2>
            <p>
              유료 상담 서비스는 인앱 결제를 통해 이용 가능하며,
              결제 및 환불 정책은 각 앱 스토어의 정책을 따릅니다.
            </p>
          </section>

          <section>
            <h2 className="text-title font-semibold text-primary mb-3">
              5. 지적 재산권
            </h2>
            <p>
              서비스 내 모든 콘텐츠(사주 카드 디자인, 분석 텍스트, UI 등)의
              지적 재산권은 서비스 제공자에게 귀속됩니다.
            </p>
          </section>
        </div>

        <p className="text-xs text-secondary-text mt-12">
          시행일: 2025년 1월 1일
        </p>
      </div>
    </div>
  );
}
