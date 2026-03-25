import { NextRequest, NextResponse } from "next/server";

const API_BASE = process.env.API_URL || "https://api.saju.app/v1";

/**
 * GET /api/fortune/daily
 * Proxy to Rust backend for daily fortune.
 * Backend requires bearerAuth (user's ilju from profile).
 * If no auth header is present, returns a generic placeholder fortune.
 */
export async function GET(request: NextRequest) {
  const authHeader = request.headers.get("Authorization");

  // Without auth, return a static placeholder for the landing page.
  // This is generic (non-personalized) data, safe to cache publicly.
  if (!authHeader) {
    const today = new Date().toLocaleDateString("sv-SE", { timeZone: "Asia/Seoul" });
    return NextResponse.json(
      {
        date: today,
        ilju: "",
        fortune_text:
          "오늘의 운세를 확인하려면 앱에서 로그인해주세요. 사주 프로필을 등록하면 매일 맞춤 운세를 받아볼 수 있습니다.",
        lucky_color: "자색",
        lucky_number: 7,
        overall_score: 3,
      },
      {
        headers: {
          "Cache-Control": "public, s-maxage=3600, stale-while-revalidate=7200",
        },
      }
    );
  }

  try {
    const response = await fetch(`${API_BASE}/fortune/daily`, {
      headers: {
        Authorization: authHeader,
      },
      next: { revalidate: 0 }, // Per-user, no shared cache
    });

    if (!response.ok) {
      return NextResponse.json(
        { error: "Failed to fetch daily fortune" },
        { status: response.status }
      );
    }

    let data;
    try {
      data = await response.json();
    } catch {
      return NextResponse.json(
        { error: "Invalid response from fortune service" },
        { status: 502 }
      );
    }

    return NextResponse.json(data);
  } catch (error) {
    console.error("Daily fortune API error:", error);
    return NextResponse.json(
      { error: "Internal server error" },
      { status: 500 }
    );
  }
}
