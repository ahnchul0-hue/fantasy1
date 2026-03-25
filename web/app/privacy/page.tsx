import type { Metadata } from "next";

const SITE_URL = process.env.NEXT_PUBLIC_SITE_URL || "https://saju.app";

export const metadata: Metadata = {
  title: "개인정보처리방침",
  description: "AI 사주 서비스의 개인정보처리방침입니다.",
  alternates: { canonical: `${SITE_URL}/privacy` },
  openGraph: {
    title: "개인정보처리방침 | AI 사주",
    description: "AI 사주 서비스의 개인정보처리방침입니다.",
    url: `${SITE_URL}/privacy`,
    type: "website",
    locale: "ko_KR",
  },
};

export default function PrivacyPage() {
  return (
    <div className="section-container py-12">
      <div className="max-w-2xl mx-auto prose prose-sm">
        <h1 className="text-display font-bold text-primary mb-8">
          개인정보처리방침
        </h1>

        <div className="space-y-6 text-sm text-on-surface leading-relaxed">
          <section>
            <h2 className="text-title font-semibold text-primary mb-3">
              1. 수집하는 개인정보 항목
            </h2>
            <p>
              본 서비스는 사주 분석을 위해 다음 정보를 수집합니다:
              생년월일, 출생 시간, 성별, 달력 유형(양력/음력).
              해당 정보는 사주 카드 생성 목적으로만 사용되며,
              별도 회원가입 없이 이용 시 서버에 영구 저장되지 않습니다.
            </p>
          </section>

          <section>
            <h2 className="text-title font-semibold text-primary mb-3">
              2. 개인정보의 이용 목적
            </h2>
            <p>
              수집된 정보는 사주팔자 계산, 사주 카드 생성, 일일 운세 제공,
              궁합 분석 등 서비스 제공 목적으로만 이용됩니다.
            </p>
          </section>

          <section>
            <h2 className="text-title font-semibold text-primary mb-3">
              3. 개인정보의 보유 및 파기
            </h2>
            <p>
              회원 탈퇴 시 또는 서비스 이용 목적 달성 후 지체 없이 파기합니다.
              비회원 이용 시 생년월일 정보는 해시 처리되어 캐싱 목적으로만
              일시 보관되며, 원본 데이터는 암호화하여 저장합니다.
            </p>
          </section>

          <section>
            <h2 className="text-title font-semibold text-primary mb-3">
              4. 개인정보의 제3자 제공
            </h2>
            <p>
              본 서비스는 이용자의 동의 없이 개인정보를 제3자에게 제공하지 않습니다.
            </p>
          </section>

          <section>
            <h2 className="text-title font-semibold text-primary mb-3">
              5. 문의
            </h2>
            <p>
              개인정보 관련 문의사항은 앱 내 고객센터 또는 이메일로
              연락해주시기 바랍니다.
            </p>
          </section>
        </div>

        <p className="text-xs text-secondary-text mt-12">
          시행일: 2025년 1월 1일
        </p>
      </div>
    </div>
  );
}
