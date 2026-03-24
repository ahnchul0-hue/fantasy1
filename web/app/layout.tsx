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
  ],
  authors: [{ name: "사주" }],
  creator: "사주",
  publisher: "사주",
  robots: { index: true, follow: true },
  metadataBase: new URL(
    process.env.NEXT_PUBLIC_SITE_URL || "https://saju.app"
  ),
  alternates: { canonical: "/" },
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
  verification: {},
  other: {
    "naver-site-verification": "",
  },
};

export const viewport: Viewport = {
  width: "device-width",
  initialScale: 1,
  maximumScale: 5,
  themeColor: "#1A1A2E",
};

const websiteJsonLd = generateJsonLd({
  type: "WebSite",
  name: "사주 - AI 사주 상담",
  description:
    "AI가 분석하는 나만의 사주 카드를 무료로 만들어보세요. 정확한 사주 분석과 AI 상담.",
  url: process.env.NEXT_PUBLIC_SITE_URL || "https://saju.app",
});

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
            __html: JSON.stringify(websiteJsonLd),
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
