import Link from "next/link";

export default function NotFound() {
  return (
    <div className="section-container py-20 text-center">
      <div className="max-w-md mx-auto">
        <div className="w-20 h-20 bg-primary/10 rounded-full mx-auto flex items-center justify-center mb-6">
          <span className="font-hanja text-3xl text-primary">迷</span>
        </div>
        <h1 className="text-display font-bold text-primary mb-3">
          페이지를 찾을 수 없습니다
        </h1>
        <p className="text-body text-secondary-text mb-8">
          요청하신 페이지가 존재하지 않거나 이동되었습니다.
        </p>
        <div className="flex flex-col sm:flex-row gap-3 justify-center">
          <Link href="/" className="cta-primary">
            홈으로 돌아가기
          </Link>
          <Link href="/card" className="cta-secondary">
            무료 사주 카드 만들기
          </Link>
        </div>
      </div>
    </div>
  );
}
