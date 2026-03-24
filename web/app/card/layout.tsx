import type { Metadata } from "next";

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
  ],
};

export default function CardLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return <>{children}</>;
}
