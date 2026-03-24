/** App Store and Play Store links */
export const APP_STORE_URL = "https://apps.apple.com/app/saju/id0000000000";
export const PLAY_STORE_URL =
  "https://play.google.com/store/apps/details?id=app.saju";

/** Site URL */
export const SITE_URL =
  process.env.NEXT_PUBLIC_SITE_URL || "https://saju.app";

/** API URL */
export const API_URL =
  process.env.NEXT_PUBLIC_API_URL || "https://api.saju.app/v1";

/** 12 시진 (birth hours) with Korean labels */
export const BIRTH_HOURS = [
  { value: "unknown", label: "모름" },
  { value: "ja", label: "자시 (23:00~01:00)" },
  { value: "chuk", label: "축시 (01:00~03:00)" },
  { value: "in", label: "인시 (03:00~05:00)" },
  { value: "myo", label: "묘시 (05:00~07:00)" },
  { value: "jin", label: "진시 (07:00~09:00)" },
  { value: "sa", label: "사시 (09:00~11:00)" },
  { value: "o", label: "오시 (11:00~13:00)" },
  { value: "mi", label: "미시 (13:00~15:00)" },
  { value: "sin", label: "신시 (15:00~17:00)" },
  { value: "yu", label: "유시 (17:00~19:00)" },
  { value: "sul", label: "술시 (19:00~21:00)" },
  { value: "hae", label: "해시 (21:00~23:00)" },
] as const;

/** Five elements color mapping */
export const ELEMENT_COLORS: Record<string, string> = {
  wood: "#4A7C59",
  fire: "#C75C3B",
  earth: "#B8956A",
  metal: "#8B8B8B",
  water: "#3D5A80",
  "\uBAA9": "#4A7C59",
  "\uD654": "#C75C3B",
  "\uD1A0": "#B8956A",
  "\uAE08": "#8B8B8B",
  "\uC218": "#3D5A80",
};

/** Five elements Korean names */
export const ELEMENT_NAMES: Record<string, string> = {
  wood: "\uBAA9 (\u6728)",
  fire: "\uD654 (\u706B)",
  earth: "\uD1A0 (\u571F)",
  metal: "\uAE08 (\u91D1)",
  water: "\uC218 (\u6C34)",
};
