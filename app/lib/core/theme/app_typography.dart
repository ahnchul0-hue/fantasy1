import 'package:flutter/material.dart';
import 'app_colors.dart';

/// Typography tokens from tokens.json
/// 한글: Pretendard Variable, 한자: Noto Serif KR
class AppTypography {
  AppTypography._();

  static const String fontPrimary = 'Pretendard';
  static const String fontHanja = 'NotoSerifKR';

  // Display: 28px/Bold
  static const TextStyle display = TextStyle(
    fontFamily: fontPrimary,
    fontSize: 28,
    fontWeight: FontWeight.w700,
    color: AppColors.onSurface,
    height: 1.3,
  );

  // Title: 22px/SemiBold
  static const TextStyle title = TextStyle(
    fontFamily: fontPrimary,
    fontSize: 22,
    fontWeight: FontWeight.w600,
    color: AppColors.onSurface,
    height: 1.3,
  );

  // Body: 16px/Regular
  static const TextStyle body = TextStyle(
    fontFamily: fontPrimary,
    fontSize: 16,
    fontWeight: FontWeight.w400,
    color: AppColors.onSurface,
    height: 1.5,
  );

  // Body SemiBold
  static const TextStyle bodySemiBold = TextStyle(
    fontFamily: fontPrimary,
    fontSize: 16,
    fontWeight: FontWeight.w600,
    color: AppColors.onSurface,
    height: 1.5,
  );

  // Caption: 13px
  static const TextStyle caption = TextStyle(
    fontFamily: fontPrimary,
    fontSize: 13,
    fontWeight: FontWeight.w400,
    color: AppColors.secondaryText,
    height: 1.4,
  );

  // Hanja style for 한자 병기 e.g. "(甲木)"
  static const TextStyle hanja = TextStyle(
    fontFamily: fontHanja,
    fontSize: 16,
    fontWeight: FontWeight.w400,
    color: AppColors.onSurface,
    height: 1.5,
  );

  // Hanja display
  static const TextStyle hanjaDisplay = TextStyle(
    fontFamily: fontHanja,
    fontSize: 28,
    fontWeight: FontWeight.w700,
    color: AppColors.onSurface,
    height: 1.3,
  );
}
