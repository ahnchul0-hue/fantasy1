import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* ISR / SSG / SSR strategy:
   * - Landing page (/) : SSG (static)
   * - Card input (/card) : SSG (static form)
   * - Card share (/card/[id]) : SSR (dynamic OG tags per card)
   * - Daily fortune (/fortune) : ISR revalidate every 3600s (hourly)
   * - API routes : server-side proxy
   */
  images: {
    remotePatterns: [
      {
        protocol: "https",
        hostname: "cdn.saju.app",
        pathname: "/images/**",
      },
      {
        protocol: "https",
        hostname: "api.saju.app",
        pathname: "/v1/**",
      },
    ],
  },
  async headers() {
    return [
      {
        source: "/(.*)",
        headers: [
          { key: "X-Frame-Options", value: "DENY" },
          { key: "X-Content-Type-Options", value: "nosniff" },
          { key: "Referrer-Policy", value: "strict-origin-when-cross-origin" },
          {
            key: "Content-Security-Policy",
            value: [
              "default-src 'self'",
              // JSON-LD (<script type="application/ld+json">) is a data block per the HTML spec
              // and is NOT subject to CSP script-src. No 'unsafe-inline' needed for JSON-LD.
              "script-src 'self' 'strict-dynamic' https://www.googletagmanager.com",
              "style-src 'self' 'unsafe-inline' https://fonts.googleapis.com",
              "font-src 'self' https://fonts.gstatic.com",
              "img-src 'self' data: https://cdn.saju.app https://api.saju.app",
              "connect-src 'self' https://api.saju.app",
            ].join("; "),
          },
        ],
      },
    ];
  },
};

export default nextConfig;
