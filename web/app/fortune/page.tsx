import type { Metadata } from "next";
import Link from "next/link";
import AppDownloadBanner from "@/components/AppDownloadBanner";
import { generateJsonLd, generateBreadcrumbJsonLd } from "@/lib/seo";

const SITE_URL = process.env.NEXT_PUBLIC_SITE_URL || "https://saju.app";

export const metadata: Metadata = {
  title: "오늘의 운세",
  description:
    "오행(五行)으로 보는 오늘의 운세. 목·화·토·금·수 오행별 운세 개요와 조언을 확인하세요. 앱에서 나의 일주별 맞춤 운세도 받아볼 수 있습니다.",
  keywords: [
    "오늘의 운세",
    "무료 운세",
    "오행 운세",
    "매일 운세",
    "AI 운세",
    "사주 운세",
    "2026 운세",
    "오늘 운세 보기",
  ],
  alternates: { canonical: `${SITE_URL}/fortune` },
  openGraph: {
    title: "오늘의 운세 | AI 사주",
    description: "오행(五行)으로 보는 오늘의 운세. 매일 업데이트.",
    url: `${SITE_URL}/fortune`,
    type: "article",
    locale: "ko_KR",
    images: [{ url: `${SITE_URL}/og-default.png`, width: 1200, height: 630, alt: "오늘의 운세" }],
  },
  twitter: {
    card: "summary_large_image",
    title: "오늘의 운세 | AI 사주",
    description: "오행(五行)으로 보는 오늘의 운세. 매일 업데이트.",
    images: [`${SITE_URL}/og-default.png`],
  },
};

/** Revalidate every hour for ISR */
export const revalidate = 3600;

/** Five element daily overview data (pre-rendered for SEO) */
const DAILY_ELEMENTS = [
  {
    element: "목 (木)",
    color: "#4A7C59",
    overview: "새로운 시작과 성장의 기운이 강한 날입니다.",
    advice: "창의적인 아이디어를 실행에 옮기기 좋은 시기입니다.",
  },
  {
    element: "화 (火)",
    color: "#C75C3B",
    overview: "열정과 활력이 넘치는 날입니다.",
    advice: "적극적인 자세로 도전하면 좋은 결과를 얻을 수 있습니다.",
  },
  {
    element: "토 (土)",
    color: "#B8956A",
    overview: "안정과 균형의 에너지가 흐르는 날입니다.",
    advice: "신뢰를 쌓고 관계를 돈독히 하는 데 집중하세요.",
  },
  {
    element: "금 (金)",
    color: "#8B8B8B",
    overview: "결단력과 정의로움이 빛나는 날입니다.",
    advice: "미루어둔 결정을 내리기에 적합한 날입니다.",
  },
  {
    element: "수 (水)",
    color: "#3D5A80",
    overview: "지혜와 통찰의 기운이 가득한 날입니다.",
    advice: "깊이 있는 사고와 학습에 몰두하면 큰 깨달음을 얻을 수 있습니다.",
  },
];

function getKSTDate() {
  const now = new Date();
  const formatted = new Intl.DateTimeFormat("ko-KR", {
    timeZone: "Asia/Seoul",
    year: "numeric",
    month: "long",
    day: "numeric",
  }).format(now);
  const iso = now.toLocaleDateString("sv-SE", { timeZone: "Asia/Seoul" });
  return { formatted, iso };
}

export default function FortunePage() {
  const { formatted: formattedDate, iso: isoDate } = getKSTDate();

  const breadcrumbJsonLd = generateBreadcrumbJsonLd([
    { name: "홈", url: SITE_URL },
    { name: "오늘의 운세", url: `${SITE_URL}/fortune` },
  ]);

  const fortuneJsonLd = generateJsonLd({
    type: "Article",
    name: `${formattedDate} 오늘의 운세`,
    description: "AI가 분석하는 오늘의 운세. 오행별 운세 개요와 조언.",
    url: `${SITE_URL}/fortune`,
    datePublished: `${isoDate}T00:00:00+09:00`,
    dateModified: `${isoDate}T00:00:00+09:00`,
  });

  return (
    <>
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{
          __html: JSON.stringify([fortuneJsonLd, breadcrumbJsonLd]),
        }}
      />

      <div className="section-container py-8 sm:py-12">
        {/* Header */}
        <div className="text-center mb-10">
          <div className="inline-flex items-center gap-1.5 px-3 py-1.5 mb-4 bg-accent/10 rounded-full">
            <div className="w-1.5 h-1.5 rounded-full bg-accent animate-pulse" />
            <span className="text-xs font-medium text-accent">
              매일 업데이트
            </span>
          </div>
          <h1 className="text-display font-bold text-primary">
            오늘의 운세
          </h1>
          <p className="text-body text-secondary-text mt-2">
            {formattedDate}
          </p>
        </div>

        {/* CTA to get personalized fortune */}
        <div className="max-w-md mx-auto mb-10">
          <div className="card-surface p-6 text-center">
            <h2 className="text-lg font-semibold text-primary mb-2">
              나의 일주별 맞춤 운세
            </h2>
            <p className="text-sm text-secondary-text mb-4">
              사주 카드를 먼저 만들면 나의 일주에 맞는
              <br />
              정확한 오늘의 운세를 확인할 수 있어요
            </p>
            <Link href="/card" className="cta-primary text-sm">
              무료 사주 카드 만들기
            </Link>
          </div>
        </div>

        {/* Five elements overview */}
        <div className="max-w-2xl mx-auto">
          <h2 className="text-title font-bold text-primary text-center mb-8">
            오행별 오늘의 운세
          </h2>

          <div className="space-y-4">
            {DAILY_ELEMENTS.map((item) => (
              <article
                key={item.element}
                className="card-surface p-5 sm:p-6"
              >
                <div className="flex items-start gap-4">
                  <div
                    className="w-12 h-12 rounded-full flex items-center justify-center shrink-0"
                    style={{ backgroundColor: `${item.color}15` }}
                  >
                    <span
                      className="font-hanja text-lg font-bold"
                      style={{ color: item.color }}
                    >
                      {item.element.charAt(0)}
                    </span>
                  </div>
                  <div className="flex-1">
                    <h3
                      className="font-semibold text-base mb-1"
                      style={{ color: item.color }}
                    >
                      {item.element}
                    </h3>
                    <p className="text-sm text-on-surface leading-relaxed">
                      {item.overview}
                    </p>
                    <p className="text-sm text-secondary-text mt-2 leading-relaxed">
                      {item.advice}
                    </p>
                  </div>
                </div>
              </article>
            ))}
          </div>
        </div>

        {/* SEO content */}
        <div className="max-w-2xl mx-auto mt-12">
          <div className="prose prose-sm max-w-none">
            <h2 className="text-title font-bold text-primary text-center mb-6">
              사주와 오늘의 운세
            </h2>
            <div className="text-sm text-secondary-text leading-relaxed space-y-4">
              <p>
                사주(四柱)는 태어난 년, 월, 일, 시의 네 기둥을 바탕으로
                인생의 흐름을 분석하는 전통 명리학입니다. 각 기둥은
                천간(天干)과 지지(地支)로 구성되어 있으며, 이를 통해
                개인의 성향, 적성, 운세를 파악할 수 있습니다.
              </p>
              <p>
                오늘의 운세는 개인의 일주(日柱)와 오늘의 천간지지가
                어떻게 상호작용하는지를 분석하여 제공됩니다. 오행(五行) -
                목(木), 화(火), 토(土), 금(金), 수(水)의 균형과 흐름이
                오늘 하루의 에너지를 결정합니다.
              </p>
              <p>
                AI 사주 분석은 3-Layer 만세력 엔진을 기반으로 정확한
                사주팔자를 계산하고, 이를 바탕으로 개인 맞춤형 운세를
                제공합니다. 무료 사주 카드를 만들고 매일 나의 운세를
                확인해보세요.
              </p>
            </div>
          </div>
        </div>

        {/* App download */}
        <div className="max-w-md mx-auto mt-12">
          <AppDownloadBanner
            title="앱에서 매일 맞춤 운세 받기"
            description="내 일주에 맞는 정확한 운세를 매일 아침 알림으로 받아보세요"
          />
        </div>
      </div>
    </>
  );
}
