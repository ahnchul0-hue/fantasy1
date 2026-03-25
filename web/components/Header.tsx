"use client";

import Link from "next/link";
import { useState } from "react";
import { APP_STORE_URL, PLAY_STORE_URL } from "@/lib/constants";

export default function Header() {
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);

  return (
    <header className="sticky top-0 z-50 bg-surface/95 backdrop-blur-md border-b border-divider/50">
      <div className="section-container flex items-center justify-between h-16">
        {/* Logo */}
        <Link href="/" className="flex items-center gap-2 group">
          <div className="w-8 h-8 bg-primary rounded-lg flex items-center justify-center">
            <span className="text-surface font-hanja text-sm font-bold">
              命
            </span>
          </div>
          <span className="font-pretendard font-bold text-lg text-primary">
            사주
          </span>
        </Link>

        {/* Desktop Navigation */}
        <nav className="hidden sm:flex items-center gap-6">
          <Link
            href="/card"
            className="text-secondary-text hover:text-primary transition-colors text-sm font-medium"
          >
            무료 사주 카드
          </Link>
          <Link
            href="/fortune"
            className="text-secondary-text hover:text-primary transition-colors text-sm font-medium"
          >
            오늘의 운세
          </Link>
          <AppDownloadButton />
        </nav>

        {/* Mobile menu button */}
        <button
          onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
          className="sm:hidden p-2 text-primary"
          aria-label={mobileMenuOpen ? "메뉴 닫기" : "메뉴 열기"}
          aria-expanded={mobileMenuOpen}
        >
          <svg
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
          >
            {mobileMenuOpen ? (
              <>
                <line x1="18" y1="6" x2="6" y2="18" />
                <line x1="6" y1="6" x2="18" y2="18" />
              </>
            ) : (
              <>
                <line x1="3" y1="8" x2="21" y2="8" />
                <line x1="3" y1="16" x2="21" y2="16" />
              </>
            )}
          </svg>
        </button>
      </div>

      {/* Mobile menu */}
      {mobileMenuOpen && (
        <div className="sm:hidden border-t border-divider/50 bg-surface animate-fade-in">
          <nav className="section-container py-4 flex flex-col gap-3">
            <Link
              href="/card"
              className="text-on-surface font-medium py-2"
              onClick={() => setMobileMenuOpen(false)}
            >
              무료 사주 카드
            </Link>
            <Link
              href="/fortune"
              className="text-on-surface font-medium py-2"
              onClick={() => setMobileMenuOpen(false)}
            >
              오늘의 운세
            </Link>
            <div className="flex gap-2 pt-2">
              <a
                href={APP_STORE_URL}
                target="_blank"
                rel="noopener noreferrer"
                className="cta-primary flex-1 text-sm"
              >
                App Store
              </a>
              <a
                href={PLAY_STORE_URL}
                target="_blank"
                rel="noopener noreferrer"
                className="cta-secondary flex-1 text-sm text-center"
              >
                Google Play
              </a>
            </div>
          </nav>
        </div>
      )}
    </header>
  );
}

function AppDownloadButton() {
  return (
    <a
      href={APP_STORE_URL}
      target="_blank"
      rel="noopener noreferrer"
      className="inline-flex items-center gap-1.5 h-9 px-4 bg-primary text-surface
        text-sm font-semibold rounded-button transition-all duration-300
        hover:bg-primary/90 hover:shadow-md"
    >
      <svg
        width="16"
        height="16"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
      >
        <path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4" />
        <polyline points="7 10 12 15 17 10" />
        <line x1="12" y1="15" x2="12" y2="3" />
      </svg>
      앱 다운로드
    </a>
  );
}
