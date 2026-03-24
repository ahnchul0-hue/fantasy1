import 'package:flutter/material.dart';

/// Design tokens from tokens.json — 오행 기반 color system
class AppColors {
  AppColors._();

  // Core palette
  static const Color primary = Color(0xFF1A1A2E);
  static const Color surface = Color(0xFFF5F0EB);
  static const Color onSurface = Color(0xFF2D2D2D);
  static const Color accent = Color(0xFFD4A574);
  static const Color error = Color(0xFFB33951);
  static const Color success = Color(0xFF4A7C59);
  static const Color disabled = Color(0xFFB8B8B8);
  static const Color divider = Color(0xFFE0D8CF);
  static const Color overlay = Color(0x801A1A2E); // rgba(26,26,46,0.5)
  static const Color secondaryText = Color(0xFF6B6B6B);
  static const Color placeholder = Color(0xFFB8B8B8);

  // 오행 colors — charts & data marks ONLY
  static const Color ohengWood = Color(0xFF4A7C59);
  static const Color ohengFire = Color(0xFFC75C3B);
  static const Color ohengEarth = Color(0xFFB8956A);
  static const Color ohengMetal = Color(0xFF8B8B8B);
  static const Color ohengWater = Color(0xFF3D5A80);

  // Chat
  static const Color chatAiBg = Color(0xFFF5F0EB);
  static const Color chatUserBg = Color(0xFF1A1A2E);
  static const Color chatUserText = Color(0xFFF5F0EB);

  // Banner
  static const Color bannerInfoBg = Color(0xFFFFF8F0);
  static const Color bannerInfoBorder = Color(0xFFD4A574);
  static const Color bannerErrorBg = Color(0x1AB33951); // rgba(179,57,81,0.1)
  static const Color bannerConnectionBg = Color(0x1A8B8B8B);

  // Dark mode tokens (v1.1 준비)
  static const Color darkPrimary = Color(0xFFF5F0EB);
  static const Color darkSurface = Color(0xFF1A1A2E);
  static const Color darkOnSurface = Color(0xFFE8E0D8);
  static const Color darkAccent = Color(0xFFD4A574);
}
