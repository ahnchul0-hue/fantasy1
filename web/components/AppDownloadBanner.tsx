import { APP_STORE_URL, PLAY_STORE_URL } from "@/lib/constants";

interface AppDownloadBannerProps {
  title?: string;
  description?: string;
  variant?: "default" | "compact";
}

export default function AppDownloadBanner({
  title = "앱에서 더 알아보세요",
  description = "AI 사주 상담, 궁합 분석, 매일 운세를 앱에서 만나보세요",
  variant = "default",
}: AppDownloadBannerProps) {
  if (variant === "compact") {
    return (
      <div className="flex items-center justify-between p-4 bg-banner-info-bg border border-banner-info-border/30 rounded-card">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 bg-primary rounded-lg flex items-center justify-center shrink-0">
            <span className="text-surface font-hanja text-sm font-bold">
              命
            </span>
          </div>
          <div>
            <p className="text-sm font-semibold text-on-surface">{title}</p>
          </div>
        </div>
        <a
          href={APP_STORE_URL}
          target="_blank"
          rel="noopener noreferrer"
          className="text-sm font-semibold text-accent hover:underline shrink-0"
        >
          다운로드
        </a>
      </div>
    );
  }

  return (
    <div className="card-surface p-6 text-center space-y-4">
      <div className="w-14 h-14 bg-primary rounded-xl mx-auto flex items-center justify-center">
        <span className="text-surface font-hanja text-xl font-bold">命</span>
      </div>
      <div>
        <h3 className="text-lg font-semibold text-primary">{title}</h3>
        <p className="text-sm text-secondary-text mt-1">{description}</p>
      </div>
      <div className="flex gap-3 justify-center">
        <a
          href={APP_STORE_URL}
          target="_blank"
          rel="noopener noreferrer"
          className="cta-primary text-sm px-6"
        >
          <svg
            className="w-4 h-4 mr-1.5"
            viewBox="0 0 24 24"
            fill="currentColor"
          >
            <path d="M18.71 19.5c-.83 1.24-1.71 2.45-3.05 2.47-1.34.03-1.77-.79-3.29-.79-1.53 0-2 .77-3.27.82-1.31.05-2.3-1.32-3.14-2.53C4.25 17 2.94 12.45 4.7 9.39c.87-1.52 2.43-2.48 4.12-2.51 1.28-.02 2.5.87 3.29.87.78 0 2.26-1.07 3.8-.91.65.03 2.47.26 3.64 1.98-.09.06-2.17 1.28-2.15 3.81.03 3.02 2.65 4.03 2.68 4.04-.03.07-.42 1.44-1.38 2.83M13 3.5c.73-.83 1.94-1.46 2.94-1.5.13 1.17-.34 2.35-1.04 3.19-.69.85-1.83 1.51-2.95 1.42-.15-1.15.41-2.35 1.05-3.11z" />
          </svg>
          App Store
        </a>
        <a
          href={PLAY_STORE_URL}
          target="_blank"
          rel="noopener noreferrer"
          className="cta-secondary text-sm px-6"
        >
          <svg
            className="w-4 h-4 mr-1.5"
            viewBox="0 0 24 24"
            fill="currentColor"
          >
            <path d="M3 20.5v-17c0-.59.34-1.11.84-1.35L13.69 12l-9.85 9.85c-.5-.24-.84-.76-.84-1.35zm13.81-5.38L6.05 21.34l8.49-8.49 2.27 2.27zm.91-.91L19.61 12l-1.89-2.21-2.27 2.27 2.27 2.15zM6.05 2.66l10.76 6.22-2.27 2.27-8.49-8.49z" />
          </svg>
          Google Play
        </a>
      </div>
    </div>
  );
}
