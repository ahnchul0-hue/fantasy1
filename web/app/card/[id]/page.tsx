import type { Metadata } from "next";
import { notFound } from "next/navigation";
import Image from "next/image";
import Link from "next/link";
import AppDownloadBanner from "@/components/AppDownloadBanner";
import { ELEMENT_COLORS } from "@/lib/constants";
import type { SajuCard } from "@/lib/api";

const API_BASE = process.env.API_URL || "https://api.saju.app/v1";
const SITE_URL = process.env.NEXT_PUBLIC_SITE_URL || "https://saju.app";
const ALLOWED_IMAGE_HOSTS = ["cdn.saju.app", "api.saju.app"];

function isSafeImageUrl(url: string): boolean {
  try {
    const { hostname } = new URL(url);
    return ALLOWED_IMAGE_HOSTS.includes(hostname);
  } catch {
    return false;
  }
}

async function getCard(id: string): Promise<SajuCard | null> {
  try {
    const res = await fetch(`${API_BASE}/saju/card/${id}`, {
      next: { revalidate: 3600 },
    });
    if (!res.ok) return null;
    return res.json();
  } catch {
    return null;
  }
}

interface PageProps {
  params: Promise<{ id: string }>;
}

export async function generateMetadata({
  params,
}: PageProps): Promise<Metadata> {
  const { id } = await params;
  const card = await getCard(id);

  if (!card) {
    return {
      title: "사주 카드를 찾을 수 없습니다",
    };
  }

  const title = `${card.ilju_name} (${card.ilju_hanja}) 사주 카드`;
  const description = `나의 사주: ${card.ilju_name} - ${card.keywords.join(", ")}. AI 사주 분석으로 만든 나만의 사주 카드를 확인하세요.`;
  const ogImage = card.image_url || `${SITE_URL}/og-default.png`;

  return {
    title,
    description,
    robots: { index: false, follow: true },
    alternates: { canonical: `${SITE_URL}/card/${id}` },
    openGraph: {
      type: "article",
      url: `${SITE_URL}/card/${id}`,
      title,
      description,
      siteName: "사주 - AI 사주 상담",
      locale: "ko_KR",
      images: [
        {
          url: ogImage,
          width: 1200,
          height: 630,
          alt: title,
        },
      ],
    },
    twitter: {
      card: "summary_large_image",
      title,
      description,
      images: [ogImage],
    },
  };
}

export default async function SharedCardPage({ params }: PageProps) {
  const { id } = await params;
  const card = await getCard(id);

  if (!card) {
    notFound();
  }

  const elementColor =
    ELEMENT_COLORS[card.lucky_element] || ELEMENT_COLORS["earth"];

  const jsonLd = {
    "@context": "https://schema.org",
    "@type": "CreativeWork",
    name: `${card.ilju_name} 사주 카드`,
    description: `${card.ilju_name} (${card.ilju_hanja}) - ${card.keywords.join(", ")}`,
    url: `${SITE_URL}/card/${id}`,
    image: card.image_url,
    creator: {
      "@type": "Organization",
      name: "사주",
    },
  };

  const breadcrumbJsonLd = {
    "@context": "https://schema.org",
    "@type": "BreadcrumbList",
    itemListElement: [
      { "@type": "ListItem", position: 1, name: "홈", item: SITE_URL },
      { "@type": "ListItem", position: 2, name: "사주 카드", item: `${SITE_URL}/card` },
      { "@type": "ListItem", position: 3, name: card.ilju_name, item: `${SITE_URL}/card/${id}` },
    ],
  };

  return (
    <>
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{
          __html: JSON.stringify([jsonLd, breadcrumbJsonLd]).replace(/</g, "\\u003c"),
        }}
      />

      <div className="section-container py-8 sm:py-12">
        <div className="max-w-md mx-auto space-y-6">
          {/* Card */}
          <div className="card-surface overflow-hidden animate-card-reveal">
            {/* Card image */}
            <div className="relative aspect-[3/4] bg-gradient-to-b from-primary/5 to-primary/10 flex items-center justify-center">
              {card.image_url && isSafeImageUrl(card.image_url) ? (
                <Image
                  src={card.image_url}
                  alt={`${card.ilju_name} 사주 카드`}
                  fill
                  sizes="(max-width: 448px) 100vw, 448px"
                  className="object-cover"
                  priority
                />
              ) : (
                <div className="text-center p-8">
                  <div
                    className="w-24 h-24 rounded-full mx-auto mb-4 flex items-center justify-center"
                    style={{ backgroundColor: `${elementColor}20` }}
                  >
                    <span
                      className="font-hanja text-4xl font-bold"
                      style={{ color: elementColor }}
                    >
                      {card.ilju_hanja?.charAt(0) || "命"}
                    </span>
                  </div>
                  <h2 className="font-hanja text-2xl text-primary mb-1">
                    {card.ilju_hanja}
                  </h2>
                  <p className="text-lg font-semibold text-primary">
                    {card.ilju_name}
                  </p>
                </div>
              )}
            </div>

            {/* Card content */}
            <div className="p-6 space-y-4">
              <div className="text-center">
                <h1 className="font-hanja text-title text-primary">
                  {card.ilju_hanja}
                </h1>
                <p className="text-lg font-semibold text-on-surface mt-1">
                  {card.ilju_name}
                </p>
              </div>

              {/* Keywords */}
              <div className="flex flex-wrap justify-center gap-2">
                {card.keywords.map((keyword) => (
                  <span
                    key={keyword}
                    className="px-3 py-1.5 rounded-full text-sm font-medium"
                    style={{
                      backgroundColor: `${elementColor}15`,
                      color: elementColor,
                    }}
                  >
                    {keyword}
                  </span>
                ))}
              </div>

              {/* Lucky element */}
              <div className="text-center pt-2">
                <span className="text-caption text-secondary-text">
                  행운의 오행
                </span>
                <div className="flex items-center justify-center gap-2 mt-1">
                  <div
                    className="w-3 h-3 rounded-full"
                    style={{ backgroundColor: elementColor }}
                  />
                  <span className="font-medium text-on-surface">
                    {card.lucky_element}
                  </span>
                </div>
              </div>
            </div>
          </div>

          {/* CTAs */}
          <div className="space-y-3">
            <Link href="/card" className="cta-primary w-full block text-center">
              나도 만들기
            </Link>

            <AppDownloadBanner
              title="더 자세한 상담받기"
              description="AI 사주 상담으로 궁금한 점을 직접 물어볼 수 있어요"
            />
          </div>
        </div>
      </div>
    </>
  );
}
