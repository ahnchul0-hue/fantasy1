"use client";

import { useState } from "react";
import BirthInputForm from "@/components/BirthInputForm";
import SajuCardResult from "@/components/SajuCardResult";
import type { BirthInput, SajuCard } from "@/lib/api";

export default function CardPage() {
  const [card, setCard] = useState<SajuCard | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (input: BirthInput) => {
    setIsLoading(true);
    setError(null);

    try {
      const res = await fetch("/api/saju/card", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(input),
      });
      if (!res.ok) throw new Error("Failed to generate saju card");
      const result: SajuCard = await res.json();
      setCard(result);
    } catch (err) {
      console.error("Failed to create saju card:", err);
      setError(
        "사주 카드 생성에 실패했습니다. 잠시 후 다시 시도해주세요."
      );
    } finally {
      setIsLoading(false);
    }
  };

  const handleReset = () => {
    setCard(null);
    setError(null);
  };

  return (
    <div className="section-container py-8 sm:py-12">
      <div className="max-w-md mx-auto">
        {!card ? (
          <>
            {/* Header */}
            <div className="text-center mb-8">
              <h1 className="text-display font-bold text-primary">
                무료 사주 카드
              </h1>
              <p className="text-body text-secondary-text mt-2">
                생년월일을 입력하고 나만의 사주 카드를 만들어보세요
              </p>
            </div>

            {/* Form */}
            <div className="card-surface p-6">
              <BirthInputForm onSubmit={handleSubmit} isLoading={isLoading} />
            </div>

            {/* Error */}
            {error && (
              <div className="mt-4 p-4 bg-error/10 border border-error/20 rounded-card text-sm text-error text-center animate-fade-in">
                {error}
              </div>
            )}

            {/* Info */}
            <p className="text-center text-caption text-secondary-text mt-6">
              입력된 정보는 사주 카드 생성에만 사용되며,
              <br />
              별도 가입 없이 무료로 이용 가능합니다.
            </p>
          </>
        ) : (
          <>
            {/* Result */}
            <div className="text-center mb-6">
              <h1 className="text-title font-bold text-primary">
                나의 사주 카드
              </h1>
            </div>

            <SajuCardResult card={card} />

            <button
              onClick={handleReset}
              className="cta-secondary w-full mt-4"
            >
              다시 만들기
            </button>
          </>
        )}
      </div>
    </div>
  );
}
