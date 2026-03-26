/**
 * API client for the Saju backend (Rust/Axum at api.saju.app/v1)
 */

import { API_URL } from "@/lib/constants";

const API_BASE = API_URL;

/** Birth input for saju card generation */
export interface BirthInput {
  year: number;
  month: number;
  day: number;
  calendar_type: "solar" | "lunar";
  is_leap_month?: boolean;
  birth_hour?:
    | "ja"
    | "chuk"
    | "in"
    | "myo"
    | "jin"
    | "sa"
    | "o"
    | "mi"
    | "sin"
    | "yu"
    | "sul"
    | "hae"
    | "unknown";
  gender: "male" | "female";
}

/** Saju card response (nullable fields per API contract) */
export interface SajuCard {
  id: string;
  ilju_name: string;
  ilju_hanja: string;
  keywords: string[];
  lucky_element: string;
  image_url: string | null;
  share_url: string | null;
  cached: boolean;
}

/** Daily fortune response */
export interface DailyFortune {
  date: string;
  ilju: string;
  fortune_text: string;
  lucky_color: string;
  lucky_number: number;
  overall_score: number;
}

/** Four Pillars pillar */
export interface Pillar {
  heavenly_stem: string;
  earthly_branch: string;
  heavenly_stem_hanja: string;
  earthly_branch_hanja: string;
}

/** Runtime validation for SajuCard data from the backend */
export function validateSajuCard(data: unknown): SajuCard | null {
  if (!data || typeof data !== "object") return null;
  const d = data as Record<string, unknown>;

  if (typeof d.id !== "string") return null;
  if (typeof d.ilju_name !== "string") return null;
  if (typeof d.ilju_hanja !== "string") return null;
  if (!Array.isArray(d.keywords)) return null;
  if (typeof d.lucky_element !== "string") return null;

  return data as SajuCard;
}

class ApiClient {
  private baseUrl: string;

  constructor(baseUrl: string = API_BASE) {
    this.baseUrl = baseUrl;
  }

  private async request<T>(
    path: string,
    options: RequestInit = {}
  ): Promise<T> {
    const url = `${this.baseUrl}${path}`;
    const res = await fetch(url, {
      ...options,
      headers: {
        "Content-Type": "application/json",
        ...options.headers,
      },
    });

    if (!res.ok) {
      const body = await res.text().catch(() => "");
      throw new ApiError(res.status, body || res.statusText);
    }

    return res.json();
  }

  /** Generate a free saju card (no auth required) */
  async createSajuCard(input: BirthInput): Promise<SajuCard> {
    return this.request<SajuCard>("/saju/card", {
      method: "POST",
      body: JSON.stringify(input),
    });
  }

  /** Get a saju card by ID (for share pages) */
  async getSajuCard(id: string): Promise<SajuCard> {
    return this.request<SajuCard>(`/saju/card/${id}`);
  }

  /** Get daily fortune (requires auth — per API contract, bearerAuth is mandatory) */
  async getDailyFortune(authToken: string): Promise<DailyFortune> {
    return this.request<DailyFortune>("/fortune/daily", {
      headers: { Authorization: `Bearer ${authToken}` },
    });
  }
}

export class ApiError extends Error {
  status: number;

  constructor(status: number, message: string) {
    super(message);
    this.status = status;
    this.name = "ApiError";
  }
}

export const api = new ApiClient();
