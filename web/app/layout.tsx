import type { Metadata, Viewport } from "next";
import "./globals.css";
import Header from "@/components/Header";
import Footer from "@/components/Footer";
import { generateJsonLd } from "@/lib/seo";

export const metadata: Metadata = {
  title: {
    default: "사주 - AI 사주 상담 | 무료 사주 카드",
    template: "%s | 사주",
  },
  description:
    "AI가 분석하는 나만의 사주 카드를 무료로 만들어보세요. 3-Layer 만세력 엔진 기반의 정확한 사주 분석과 AI 상담 서비스.",
  keywords: [
    "사주",
    "무료 사주",
    "오늘의 운세",
    "AI 사주",
    "사주 카드",
    "사주팔자",
    "운세",
    "만세력",
    "궁합",
    "사주 상담",
    "무료 운세",
    "무료 만세력",
    "사주풀이 무료",
    "연애운",
    "재물운",
    "직업운",
    "궁합 테스트",
    "2026 운세",
  ],
  authors: [{ name: "사주" }],
  creator: "사주",
  publisher: "사주",
  robots: { index: true, follow: true },
  metadataBase: new URL(
    process.env.NEXT_PUBLIC_SITE_URL || "https://saju.app"
  ),
  alternates: {
    canonical: "/",
  },
  openGraph: {
    type: "website",
    locale: "ko_KR",
    url: "/",
    siteName: "사주 - AI 사주 상담",
    title: "사주 - AI 사주 상담 | 무료 사주 카드",
    description:
      "AI가 분석하는 나만의 사주 카드를 무료로 만들어보세요. 정확한 사주 분석과 AI 상담.",
    images: [
      {
        url: "/og-default.png",
        width: 1200,
        height: 630,
        alt: "사주 - AI 사주 상담",
      },
    ],
  },
  twitter: {
    card: "summary_large_image",
    title: "사주 - AI 사주 상담 | 무료 사주 카드",
    description:
      "AI가 분석하는 나만의 사주 카드를 무료로 만들어보세요.",
    images: ["/og-default.png"],
  },
  verification: {
    other: {
      /** TODO: 네이버 웹마스터 도구에서 인증 코드 발급 후 교체 */
      "naver-site-verification": "NAVER_VERIFICATION_CODE",
    },
  },
};

export const viewport: Viewport = {
  width: "device-width",
  initialScale: 1,
  maximumScale: 5,
  themeColor: "#1A1A2E",
};

const SITE_URL = process.env.NEXT_PUBLIC_SITE_URL || "https://saju.app";

const websiteJsonLd = generateJsonLd({
  type: "WebSite",
  name: "사주 - AI 사주 상담",
  description:
    "AI가 분석하는 나만의 사주 카드를 무료로 만들어보세요. 정확한 사주 분석과 AI 상담.",
  url: SITE_URL,
});

const organizationJsonLd = {
  "@context": "https://schema.org",
  "@type": "Organization",
  name: "사주",
  url: SITE_URL,
  logo: `${SITE_URL}/icon-512.png`,
  sameAs: [],
};

const mobileAppJsonLd = {
  "@context": "https://schema.org",
  "@type": "MobileApplication",
  name: "사주 - AI 사주 상담",
  operatingSystem: "iOS, Android",
  applicationCategory: "LifestyleApplication",
  offers: { "@type": "Offer", price: "0", priceCurrency: "KRW" },
  description:
    "AI 기반 사주 분석, 맞춤 운세, 궁합 분석을 제공하는 사주 상담 앱",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="ko">
      <head>
        {/* Pretendard Variable */}
        <link
          rel="stylesheet"
          as="style"
          crossOrigin="anonymous"
          href="https://cdn.jsdelivr.net/gh/orioncactus/pretendard@v1.3.9/dist/web/variable/pretendardvariable-dynamic-subset.min.css"
        />
        {/* Noto Serif KR for hanja */}
        <link
          rel="preconnect"
          href="https://fonts.googleapis.com"
        />
        <link
          rel="preconnect"
          href="https://fonts.gstatic.com"
          crossOrigin="anonymous"
        />
        <link
          href="https://fonts.googleapis.com/css2?family=Noto+Serif+KR:wght@400;700&display=swap"
          rel="stylesheet"
        />
        {/* JSON-LD structured data */}
        <script
          type="application/ld+json"
          dangerouslySetInnerHTML={{
            __html: JSON.stringify([websiteJsonLd, organizationJsonLd, mobileAppJsonLd]),
          }}
        />
        {/* Firebase Analytics placeholder */}
        {/* <script src="https://www.googletagmanager.com/gtag/js?id=G-XXXXXXXXXX" async></script> */}
      </head>
      <body className="font-pretendard bg-surface text-on-surface min-h-dvh flex flex-col">
        <Header />
        <main className="flex-1">{children}</main>
        <Footer />
      </body>
    </html>
  );
}
