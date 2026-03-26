'use client';

export default function CardError({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  return (
    <div className="min-h-screen flex items-center justify-center">
      <div className="text-center">
        <h2 className="text-xl font-semibold mb-4">
          사주 카드를 불러올 수 없습니다
        </h2>
        <p className="text-gray-600 mb-6">
          {error.message || '서버에 일시적인 문제가 발생했습니다. 잠시 후 다시 시도해주세요.'}
        </p>
        <button
          onClick={reset}
          className="px-4 py-2 bg-primary text-white rounded-lg"
        >
          다시 시도
        </button>
      </div>
    </div>
  );
}
