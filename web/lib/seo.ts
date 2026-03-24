import type { Metadata } from "next";

const SITE_URL = process.env.NEXT_PUBLIC_SITE_URL || "https://saju.app";
const SITE_NAME = "사주 - AI 사주 상담";

export function createMetadata(options: {
  title: string;
  description: string;
  path?: string;
  ogImage?: string;
  noIndex?: boolean;
}): Metadata {
  const url = `${SITE_URL}${options.path || ""}`;
  const ogImage = options.ogImage || `${SITE_URL}/og-default.png`;

  return {
    title: options.title,
    description: options.description,
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
    ],
    authors: [{ name: "사주" }],
    creator: "사주",
    publisher: "사주",
    robots: options.noIndex
      ? { index: false, follow: false }
      : { index: true, follow: true },
    alternates: { canonical: url },
    openGraph: {
      type: "website",
      url,
      title: options.title,
      description: options.description,
      siteName: SITE_NAME,
      locale: "ko_KR",
      images: [
        {
          url: ogImage,
          width: 1200,
          height: 630,
          alt: options.title,
        },
      ],
    },
    twitter: {
      card: "summary_large_image",
      title: options.title,
      description: options.description,
      images: [ogImage],
    },
  };
}

/** Generate JSON-LD structured data for a page */
export function generateJsonLd(options: {
  type: "WebSite" | "WebPage" | "Article";
  name: string;
  description: string;
  url: string;
  image?: string;
  datePublished?: string;
  dateModified?: string;
}) {
  const base: Record<string, unknown> = {
    "@context": "https://schema.org",
    "@type": options.type,
    name: options.name,
    description: options.description,
    url: options.url,
  };

  if (options.image) {
    base.image = options.image;
  }

  if (options.type === "WebSite") {
    base.potentialAction = {
      "@type": "SearchAction",
      target: {
        "@type": "EntryPoint",
        urlTemplate: `${SITE_URL}/card?q={search_term_string}`,
      },
      "query-input": "required name=search_term_string",
    };
  }

  if (options.datePublished) {
    base.datePublished = options.datePublished;
  }
  if (options.dateModified) {
    base.dateModified = options.dateModified;
  }

  return base;
}
