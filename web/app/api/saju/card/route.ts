import { NextRequest, NextResponse } from "next/server";

const API_BASE = process.env.API_URL || "https://api.saju.app/v1";

/** Validate BirthInput shape before forwarding to backend. */
function validateBirthInput(
  body: unknown
): body is {
  year: number;
  month: number;
  day: number;
  birth_hour: string;
  gender: string;
  calendar_type: string;
} {
  if (typeof body !== "object" || body === null) return false;
  const b = body as Record<string, unknown>;
  return (
    typeof b.year === "number" &&
    b.year >= 1900 &&
    b.year <= 2100 &&
    typeof b.month === "number" &&
    b.month >= 1 &&
    b.month <= 12 &&
    typeof b.day === "number" &&
    b.day >= 1 &&
    b.day <= 31 &&
    typeof b.birth_hour === "string" &&
    typeof b.gender === "string" &&
    ["male", "female"].includes(b.gender) &&
    typeof b.calendar_type === "string" &&
    ["solar", "lunar"].includes(b.calendar_type)
  );
}

/**
 * POST /api/saju/card
 * Proxy to Rust backend for saju card generation.
 * Validates input before forwarding to prevent arbitrary payloads.
 */
export async function POST(request: NextRequest) {
  try {
    let body: unknown;
    try {
      body = await request.json();
    } catch {
      return NextResponse.json(
        { error: "Invalid JSON body" },
        { status: 400 }
      );
    }

    if (!validateBirthInput(body)) {
      return NextResponse.json(
        { error: "Invalid birth input" },
        { status: 400 }
      );
    }

    // Forward auth header if present (for optional profile save)
    const headers: Record<string, string> = {
      "Content-Type": "application/json",
    };
    const authHeader = request.headers.get("Authorization");
    if (authHeader) {
      headers["Authorization"] = authHeader;
    }

    const response = await fetch(`${API_BASE}/saju/card`, {
      method: "POST",
      headers,
      body: JSON.stringify(body),
    });

    if (!response.ok) {
      return NextResponse.json(
        { error: "Failed to generate saju card" },
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
