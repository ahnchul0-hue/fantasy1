import type { Metadata } from "next";

const SITE_URL = process.env.NEXT_PUBLIC_SITE_URL || "https://saju.app";

export const metadata: Metadata = {
  title: "무료 사주 카드 만들기",
  description:
    "생년월일을 입력하고 AI가 분석하는 나만의 사주 카드를 무료로 만들어보세요. 60가지 일주 유형 중 나는 어떤 유형일까요?",
  keywords: [
    "무료 사주",
    "사주 카드",
    "무료 사주 카드",
    "AI 사주",
    "사주팔자",
    "일주",
    "무료 만세력",
    "사주풀이 무료",
    "일주별 성격",
  ],
  alternates: { canonical: `${SITE_URL}/card` },
  openGraph: {
    title: "무료 사주 카드 만들기 | AI 사주",
    description: "생년월일만 입력하면 AI가 나만의 사주 카드를 무료로 만들어드립니다.",
    url: `${SITE_URL}/card`,
    type: "website",
    locale: "ko_KR",
    images: [{ url: `${SITE_URL}/og-default.png`, width: 1200, height: 630, alt: "무료 사주 카드" }],
  },
  twitter: {
    card: "summary_large_image",
    title: "무료 사주 카드 만들기 | AI 사주",
    description: "생년월일만 입력하면 AI가 나만의 사주 카드를 무료로 만들어드립니다.",
    images: [`${SITE_URL}/og-default.png`],
  },
};

export default function CardLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return <>{children}</>;
}
