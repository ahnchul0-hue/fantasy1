"use client";

import { useState, useCallback } from "react";
import { BIRTH_HOURS } from "@/lib/constants";
import type { BirthInput } from "@/lib/api";

interface BirthInputFormProps {
  onSubmit: (input: BirthInput) => void;
  isLoading?: boolean;
}

export default function BirthInputForm({
  onSubmit,
  isLoading = false,
}: BirthInputFormProps) {
  const [year, setYear] = useState("");
  const [month, setMonth] = useState("");
  const [day, setDay] = useState("");
  const [calendarType, setCalendarType] = useState<"solar" | "lunar">("solar");
  const [isLeapMonth, setIsLeapMonth] = useState(false);
  const [birthHour, setBirthHour] = useState<BirthInput["birth_hour"]>("unknown");
  const [gender, setGender] = useState<"male" | "female" | "">("");
  const [errors, setErrors] = useState<Record<string, string>>({});

  const validate = useCallback((): boolean => {
    const newErrors: Record<string, string> = {};
    const y = parseInt(year);
    const m = parseInt(month);
    const d = parseInt(day);

    if (!year || isNaN(y) || y < 1900 || y > 2100) {
      newErrors.year = "올바른 연도를 입력해주세요 (1900~2100)";
    }
    if (!month || isNaN(m) || m < 1 || m > 12) {
      newErrors.month = "올바른 월을 입력해주세요 (1~12)";
    }

    const maxDay =
      !isNaN(y) && !isNaN(m) && m >= 1 && m <= 12
        ? new Date(y, m, 0).getDate()
        : 31;
    if (!day || isNaN(d) || d < 1 || d > maxDay) {
      newErrors.day =
        maxDay < 31
          ? `${m}월은 최대 ${maxDay}일까지 입력 가능합니다`
          : "올바른 일을 입력해주세요 (1~31)";
    }
    if (!gender) {
      newErrors.gender = "성별을 선택해주세요";
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  }, [year, month, day, gender]);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!validate()) return;

    onSubmit({
      year: parseInt(year),
      month: parseInt(month),
      day: parseInt(day),
      calendar_type: calendarType,
      is_leap_month: calendarType === "lunar" ? isLeapMonth : false,
      birth_hour: birthHour,
      gender: gender as "male" | "female",
    });
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* Calendar type toggle */}
      <div>
        <label className="block text-sm font-medium text-on-surface mb-2">
          달력 유형
        </label>
        <div className="flex rounded-segment bg-divider/50 p-0.5">
          <button
            type="button"
            onClick={() => setCalendarType("solar")}
            className={`flex-1 py-2 text-sm font-medium rounded-[18px] transition-all duration-300 ${
              calendarType === "solar"
                ? "bg-white text-primary shadow-sm"
                : "text-secondary-text"
            }`}
          >
            양력
          </button>
          <button
            type="button"
            onClick={() => setCalendarType("lunar")}
            className={`flex-1 py-2 text-sm font-medium rounded-[18px] transition-all duration-300 ${
              calendarType === "lunar"
                ? "bg-white text-primary shadow-sm"
                : "text-secondary-text"
            }`}
          >
            음력
          </button>
        </div>
      </div>

      {/* Birth date */}
      <fieldset>
        <legend className="block text-sm font-medium text-on-surface mb-2">
          생년월일
        </legend>
        <div className="grid grid-cols-3 gap-3">
          <div>
            <label htmlFor="birth-year" className="sr-only">년</label>
            <input
              id="birth-year"
              type="number"
              placeholder="년 (YYYY)"
              value={year}
              onChange={(e) => setYear(e.target.value)}
              className="input-field text-center"
              min={1900}
              max={2100}
              inputMode="numeric"
              aria-describedby={errors.year ? "error-year" : undefined}
              aria-invalid={!!errors.year}
            />
            {errors.year && (
              <p id="error-year" className="text-error text-xs mt-1" role="alert">{errors.year}</p>
            )}
          </div>
          <div>
            <label htmlFor="birth-month" className="sr-only">월</label>
            <input
              id="birth-month"
              type="number"
              placeholder="월"
              value={month}
              onChange={(e) => setMonth(e.target.value)}
              className="input-field text-center"
              min={1}
              max={12}
              inputMode="numeric"
              aria-describedby={errors.month ? "error-month" : undefined}
              aria-invalid={!!errors.month}
            />
            {errors.month && (
              <p id="error-month" className="text-error text-xs mt-1" role="alert">{errors.month}</p>
            )}
          </div>
          <div>
            <label htmlFor="birth-day" className="sr-only">일</label>
            <input
              id="birth-day"
              type="number"
              placeholder="일"
              value={day}
              onChange={(e) => setDay(e.target.value)}
              className="input-field text-center"
              min={1}
              max={31}
              inputMode="numeric"
              aria-describedby={errors.day ? "error-day" : undefined}
              aria-invalid={!!errors.day}
            />
            {errors.day && (
              <p id="error-day" className="text-error text-xs mt-1" role="alert">{errors.day}</p>
            )}
          </div>
        </div>
      </fieldset>

      {/* Leap month (only for lunar) */}
      {calendarType === "lunar" && (
        <label className="flex items-center gap-2 cursor-pointer animate-fade-in">
          <input
            type="checkbox"
            checked={isLeapMonth}
            onChange={(e) => setIsLeapMonth(e.target.checked)}
            className="w-4 h-4 rounded accent-accent"
          />
          <span className="text-sm text-secondary-text">윤달</span>
        </label>
      )}

      {/* Birth hour */}
      <div>
        <label htmlFor="birth-hour" className="block text-sm font-medium text-on-surface mb-2">
          태어난 시간
        </label>
        <select
          id="birth-hour"
          value={birthHour}
          onChange={(e) =>
            setBirthHour(e.target.value as BirthInput["birth_hour"])
          }
          className="input-field appearance-none bg-[url('data:image/svg+xml;charset=utf-8,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20width%3D%2212%22%20height%3D%2212%22%20viewBox%3D%220%200%2012%2012%22%3E%3Cpath%20fill%3D%22%236B6B6B%22%20d%3D%22M2%204l4%204%204-4%22%2F%3E%3C%2Fsvg%3E')] bg-[length:12px] bg-[right_16px_center] bg-no-repeat pr-10"
        >
          {BIRTH_HOURS.map((hour) => (
            <option key={hour.value} value={hour.value}>
              {hour.label}
            </option>
          ))}
        </select>
      </div>

      {/* Gender */}
      <fieldset>
        <legend className="block text-sm font-medium text-on-surface mb-2">
          성별
        </legend>
        <div className="flex gap-3" role="radiogroup" aria-label="성별 선택">
          <button
            type="button"
            role="radio"
            aria-checked={gender === "male"}
            onClick={() => setGender("male")}
            className={`flex-1 h-12 rounded-button border text-sm font-medium transition-all duration-300 ${
              gender === "male"
                ? "border-accent bg-accent/10 text-accent"
                : "border-divider text-secondary-text hover:border-accent/50"
            }`}
          >
            남성
          </button>
          <button
            type="button"
            role="radio"
            aria-checked={gender === "female"}
            onClick={() => setGender("female")}
            className={`flex-1 h-12 rounded-button border text-sm font-medium transition-all duration-300 ${
              gender === "female"
                ? "border-accent bg-accent/10 text-accent"
                : "border-divider text-secondary-text hover:border-accent/50"
            }`}
          >
            여성
          </button>
        </div>
        {errors.gender && (
          <p id="error-gender" className="text-error text-xs mt-1" role="alert">{errors.gender}</p>
        )}
      </fieldset>

      {/* Submit */}
      <button
        type="submit"
        disabled={isLoading}
        className="cta-primary w-full"
      >
        {isLoading ? (
          <span className="flex items-center gap-2">
            <svg
              className="animate-spin h-5 w-5"
              viewBox="0 0 24 24"
              fill="none"
            >
              <circle
                className="opacity-25"
                cx="12"
                cy="12"
                r="10"
                stroke="currentColor"
                strokeWidth="4"
              />
              <path
                className="opacity-75"
                fill="currentColor"
                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"
              />
            </svg>
            사주 카드 생성 중...
          </span>
        ) : (
          "무료 사주 카드 만들기"
        )}
      </button>
    </form>
  );
}
