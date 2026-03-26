"use client";

import { useEffect, useState } from "react";
import Image from "next/image";
import type { SajuCard } from "@/lib/api";
import { ELEMENT_COLORS } from "@/lib/constants";
import AppDownloadBanner from "./AppDownloadBanner";

const ALLOWED_IMAGE_HOSTS = ["cdn.saju.app", "api.saju.app"];

function isSafeImageUrl(url: string): boolean {
  try {
    const parsed = new URL(url);
    return parsed.protocol === "https:" && ALLOWED_IMAGE_HOSTS.includes(parsed.hostname);
  } catch {
    return false;
  }
}

interface SajuCardResultProps {
  card: SajuCard;
}

export default function SajuCardResult({ card }: SajuCardResultProps) {
  const [revealed, setRevealed] = useState(false);
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    const timer = setTimeout(() => setRevealed(true), 100);
    return () => clearTimeout(timer);
  }, []);

  const elementColor =
    ELEMENT_COLORS[card.lucky_element] || ELEMENT_COLORS["earth"];

  const handleShare = async () => {
    const shareUrl = card.share_url || `${window.location.origin}/card/${card.id}`;

    if (navigator.share) {
      try {
        await navigator.share({
          title: `${card.ilju_name} 사주 카드`,
          text: `나의 사주: ${card.ilju_name} (${card.ilju_hanja}) - ${(card.keywords ?? []).join(", ")}`,
          url: shareUrl,
        });
      } catch {
        // User cancelled share
      }
    } else {
      try {
        await navigator.clipboard.writeText(shareUrl);
        setCopied(true);
        setTimeout(() => setCopied(false), 2000);
      } catch {
        window.prompt("링크를 복사해주세요:", shareUrl);
      }
    }
  };

  return (
    <div className="space-y-6">
      <div
        className={`card-surface overflow-hidden transition-all duration-600 ${
          revealed
            ? "opacity-100 translate-y-0 scale-100"
            : "opacity-0 translate-y-5 scale-95"
        }`}
      >
        {/* Card image area */}
        <div className="relative aspect-[3/4] bg-gradient-to-b from-primary/5 to-primary/10 flex items-center justify-center overflow-hidden">
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
            /* Placeholder card design when no image */
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
              <h3 className="font-hanja text-2xl text-primary mb-1">
                {card.ilju_hanja}
              </h3>
              <p className="text-lg font-semibold text-primary">
                {card.ilju_name}
              </p>
            </div>
          )}
        </div>

        {/* Card content */}
        <div className="p-6 space-y-4">
          <div className="text-center">
            <h2 className="font-hanja text-title text-primary">
              {card.ilju_hanja}
            </h2>
            <p className="text-lg font-semibold text-on-surface mt-1">
              {card.ilju_name}
            </p>
          </div>

          {/* Keywords */}
          <div className="flex flex-wrap justify-center gap-2">
            {(card.keywords ?? []).map((keyword) => (
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

      {/* Action buttons */}
      <div
        className={`space-y-3 transition-all duration-500 delay-300 ${
          revealed ? "opacity-100 translate-y-0" : "opacity-0 translate-y-4"
        }`}
      >
        <button onClick={handleShare} className="cta-primary w-full">
          {copied ? "링크가 복사되었습니다!" : "카드 공유하기"}
        </button>

        <AppDownloadBanner
          title="더 자세한 사주 분석을 받아보세요"
          description="AI 사주 상담으로 궁금한 점을 직접 물어볼 수 있어요"
        />
      </div>
    </div>
  );
}
