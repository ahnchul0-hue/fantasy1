import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./app/**/*.{ts,tsx}",
    "./components/**/*.{ts,tsx}",
    "./lib/**/*.{ts,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: "#1A1A2E",
        surface: "#F5F0EB",
        "on-surface": "#2D2D2D",
        accent: "#D4A574",
        error: "#B33951",
        success: "#4A7C59",
        disabled: "#B8B8B8",
        divider: "#E0D8CF",
        "secondary-text": "#6B6B6B",
        placeholder: "#B8B8B8",
        oheng: {
          wood: "#4A7C59",
          fire: "#C75C3B",
          earth: "#B8956A",
          metal: "#8B8B8B",
          water: "#3D5A80",
        },
        banner: {
          "info-bg": "#FFF8F0",
          "info-border": "#D4A574",
        },
      },
      fontFamily: {
        pretendard: [
          "Pretendard Variable",
          "Pretendard",
          "-apple-system",
          "BlinkMacSystemFont",
          "system-ui",
          "Helvetica Neue",
          "sans-serif",
        ],
        hanja: ["Noto Serif KR", "serif"],
      },
      fontSize: {
        display: ["28px", { lineHeight: "1.3", fontWeight: "700" }],
        title: ["22px", { lineHeight: "1.4", fontWeight: "600" }],
        body: ["16px", { lineHeight: "1.6", fontWeight: "400" }],
        caption: ["13px", { lineHeight: "1.5", fontWeight: "400" }],
      },
      spacing: {
        xs: "4px",
        sm: "8px",
        md: "16px",
        lg: "24px",
        xl: "40px",
      },
      borderRadius: {
        card: "12px",
        button: "8px",
        input: "8px",
        segment: "20px",
      },
      animation: {
        "fade-in": "fadeIn 0.3s cubic-bezier(0.33, 1, 0.68, 1)",
        "fade-in-up": "fadeInUp 0.4s cubic-bezier(0.33, 1, 0.68, 1)",
        "slide-in-left": "slideInLeft 0.15s cubic-bezier(0.33, 1, 0.68, 1)",
        "card-reveal": "cardReveal 0.2s cubic-bezier(0.33, 1, 0.68, 1)",
      },
      keyframes: {
        fadeIn: {
          "0%": { opacity: "0" },
          "100%": { opacity: "1" },
        },
        fadeInUp: {
          "0%": { opacity: "0", transform: "translateY(16px)" },
          "100%": { opacity: "1", transform: "translateY(0)" },
        },
        slideInLeft: {
          "0%": { opacity: "0", transform: "translateX(-12px)" },
          "100%": { opacity: "1", transform: "translateX(0)" },
        },
        cardReveal: {
          "0%": { opacity: "0", transform: "scale(0.9) translateY(20px)" },
          "100%": { opacity: "1", transform: "scale(1) translateY(0)" },
        },
      },
    },
  },
  plugins: [],
};

export default config;
