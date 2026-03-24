import { NextResponse } from "next/server";

const API_BASE = process.env.API_URL || "https://api.saju.app/v1";

/**
 * GET /api/fortune/daily
 * Proxy to Rust backend for daily fortune.
 * This is a public endpoint (no auth required for the web landing page variant).
 */
export async function GET() {
  try {
    const response = await fetch(`${API_BASE}/fortune/daily`, {
      next: { revalidate: 3600 }, // Cache for 1 hour
    });

    if (!response.ok) {
      return NextResponse.json(
        { error: "Failed to fetch daily fortune" },
        { status: response.status }
      );
    }

    const data = await response.json();
    return NextResponse.json(data, {
      headers: {
        "Cache-Control": "public, s-maxage=3600, stale-while-revalidate=7200",
      },
    });
  } catch (error) {
    console.error("Daily fortune API error:", error);
    return NextResponse.json(
      { error: "Internal server error" },
      { status: 500 }
    );
  }
}
