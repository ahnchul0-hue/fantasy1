import { NextRequest, NextResponse } from "next/server";
import { BACKEND_URL } from "@/lib/constants";

const VALID_HOURS = [
  "ja", "chuk", "in", "myo", "jin", "sa",
  "o", "mi", "sin", "yu", "sul", "hae", "unknown",
] as const;

/** Validate and sanitize BirthInput, returning a strict whitelist payload or an error message. */
function validateAndSanitize(
  body: unknown
): { ok: true; data: SanitizedBirthInput } | { ok: false; error: string } {
  if (typeof body !== "object" || body === null) {
    return { ok: false, error: "Invalid JSON body" };
  }
  const b = body as Record<string, unknown>;

  const year = Number(b.year);
  const month = Number(b.month);
  const day = Number(b.day);
  const gender = b.gender;
  const calendarType = b.calendar_type;
  const birthHour = b.birth_hour;
  const isLeapMonth = Boolean(b.is_leap_month);

  const maxBirthYear = new Date().getFullYear() - 14;
  if (!Number.isInteger(year) || year < 1940 || year > maxBirthYear) {
    return { ok: false, error: `유효하지 않은 연도입니다 (1940~${maxBirthYear}년, 만 14세 이상)` };
  }
  if (!Number.isInteger(month) || month < 1 || month > 12) {
    return { ok: false, error: "유효하지 않은 월입니다" };
  }
  if (!Number.isInteger(day) || day < 1 || day > 31) {
    return { ok: false, error: "유효하지 않은 일입니다" };
  }
  if (!["male", "female"].includes(gender as string)) {
    return { ok: false, error: "유효하지 않은 성별입니다" };
  }
  if (!["solar", "lunar"].includes(calendarType as string)) {
    return { ok: false, error: "유효하지 않은 달력 유형입니다" };
  }
  if (!VALID_HOURS.includes(birthHour as typeof VALID_HOURS[number])) {
    return { ok: false, error: "유효하지 않은 시진입니다" };
  }

  return {
    ok: true,
    data: {
      year,
      month,
      day,
      gender: gender as "male" | "female",
      calendar_type: calendarType as "solar" | "lunar",
      birth_hour: birthHour as string,
      is_leap_month: isLeapMonth,
    },
  };
}

interface SanitizedBirthInput {
  year: number;
  month: number;
  day: number;
  gender: "male" | "female";
  calendar_type: "solar" | "lunar";
  birth_hour: string;
  is_leap_month: boolean;
}

/**
 * POST /api/saju/card
 * Proxy to Rust backend for saju card generation.
 * Validates input before forwarding to prevent arbitrary payloads.
 */
export async function POST(request: NextRequest) {
  try {
    let rawBody: unknown;
    try {
      rawBody = await request.json();
    } catch {
      return NextResponse.json(
        { error: "Invalid JSON body" },
        { status: 400 }
      );
    }

    const validation = validateAndSanitize(rawBody);
    if (!validation.ok) {
      return NextResponse.json(
        { error: validation.error },
        { status: 400 }
      );
    }
    const sanitizedBody = validation.data;

    // Forward auth header if present (for optional profile save)
    const headers: Record<string, string> = {
      "Content-Type": "application/json",
    };
    const authHeader = request.headers.get("Authorization");
    if (authHeader) {
      headers["Authorization"] = authHeader;
    }

    const response = await fetch(`${BACKEND_URL}/saju/card`, {
      method: "POST",
      headers,
      body: JSON.stringify(sanitizedBody),
    });

    if (!response.ok) {
      let errorMsg = "사주 카드 생성에 실패했습니다";
      try {
        const errBody = await response.json();
        if (errBody?.error?.message) errorMsg = errBody.error.message;
        else if (typeof errBody?.error === "string") errorMsg = errBody.error;
      } catch { /* use default */ }
      return NextResponse.json(
        { error: errorMsg },
        { status: response.status }
      );
    }

    let data;
    try {
      data = await response.json();
    } catch {
      return NextResponse.json(
        { error: "Invalid response from card service" },
        { status: 502 }
      );
    }

    return NextResponse.json(data);
  } catch (error) {
    console.error("Saju card API error:", error);
    return NextResponse.json(
      { error: "Internal server error" },
      { status: 500 }
    );
  }
}
