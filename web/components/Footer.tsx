import Link from "next/link";

export default function Footer() {
  const currentYear = new Date().getFullYear();

  return (
    <footer className="bg-primary text-surface/80">
      <div className="section-container py-12">
        <div className="grid grid-cols-1 sm:grid-cols-3 gap-8">
          {/* Brand */}
          <div>
            <div className="flex items-center gap-2 mb-4">
              <div className="w-7 h-7 bg-surface/10 rounded-lg flex items-center justify-center">
                <span className="text-surface font-hanja text-xs font-bold">
                  命
                </span>
              </div>
              <span className="font-pretendard font-bold text-surface">
                사주
              </span>
            </div>
            <p className="text-sm leading-relaxed text-surface/60">
              AI 기술과 전통 명리학을 결합한
              <br />
              새로운 사주 상담 서비스
            </p>
          </div>

          {/* Links */}
          <div>
            <h3 className="font-semibold text-surface mb-3 text-sm">
              서비스
            </h3>
            <ul className="space-y-2 text-sm">
              <li>
                <Link
                  href="/card"
                  className="hover:text-accent transition-colors"
                >
                  무료 사주 카드
                </Link>
              </li>
              <li>
                <Link
                  href="/fortune"
                  className="hover:text-accent transition-colors"
                >
                  오늘의 운세
                </Link>
              </li>
            </ul>
          </div>

          {/* Legal */}
          <div>
            <h3 className="font-semibold text-surface mb-3 text-sm">
              안내
            </h3>
            <ul className="space-y-2 text-sm">
              <li>
                <Link
                  href="/privacy"
                  className="hover:text-accent transition-colors"
                >
                  개인정보처리방침
                </Link>
              </li>
              <li>
                <Link
                  href="/terms"
                  className="hover:text-accent transition-colors"
                >
                  이용약관
                </Link>
              </li>
            </ul>
          </div>
        </div>

        {/* Divider */}
        <div className="border-t border-surface/10 mt-8 pt-8">
          {/* Disclaimer */}
          <p className="text-xs text-surface/40 leading-relaxed mb-4">
            면책고지: 본 서비스는 전통 명리학과 AI 기술을 결합한 엔터테인먼트
            목적의 서비스입니다. 사주 분석 결과는 참고용이며, 중요한 의사결정의
            근거로 사용하지 마시기 바랍니다. 본 서비스의 분석 결과에 대해
            법적 책임을 지지 않습니다.
          </p>
          <p className="text-xs text-surface/30">
            &copy; {currentYear} 사주. All rights reserved.
          </p>
        </div>
      </div>
    </footer>
  );
}
