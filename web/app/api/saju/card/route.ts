import { NextRequest, NextResponse } from "next/server";

const API_BASE = process.env.API_URL || "https://api.saju.app/v1";

/**
 * POST /api/saju/card
 * Proxy to Rust backend for saju card generation.
 * This avoids CORS issues and keeps the backend URL private.
 */
export async function POST(request: NextRequest) {
  try {
    const body = await request.json();

    const response = await fetch(`${API_BASE}/saju/card`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(body),
    });

    if (!response.ok) {
      return NextResponse.json(
        { error: "Failed to generate saju card" },
        { status: response.status }
      );
    }

    const data = await response.json();
    return NextResponse.json(data);
  } catch (error) {
    console.error("Saju card API error:", error);
    return NextResponse.json(
      { error: "Internal server error" },
      { status: 500 }
    );
  }
}
