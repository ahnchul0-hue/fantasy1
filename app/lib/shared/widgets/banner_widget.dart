import 'package:flutter/material.dart';
import '../../core/theme/app_colors.dart';
import '../../core/theme/app_typography.dart';
import '../../core/theme/app_spacing.dart';

enum BannerType { info, error, connection, sessionWarning }

/// Full-width banner widget — h48
class BannerWidget extends StatelessWidget {
  final String text;
  final BannerType type;
  final VoidCallback? onTap;
  final IconData? icon;

  const BannerWidget({
    super.key,
    required this.text,
    this.type = BannerType.info,
    this.onTap,
    this.icon,
  });

  Color get _bgColor {
    switch (type) {
      case BannerType.info:
        return AppColors.bannerInfoBg;
      case BannerType.error:
        return AppColors.bannerErrorBg;
      case BannerType.connection:
        return AppColors.bannerConnectionBg;
      case BannerType.sessionWarning:
        return AppColors.bannerErrorBg;
    }
  }

  Color? get _borderColor {
    switch (type) {
      case BannerType.info:
        return AppColors.bannerInfoBorder;
      default:
        return null;
    }
  }

  Color get _textColor {
    switch (type) {
      case BannerType.info:
        return AppColors.onSurface;
      case BannerType.error:
      case BannerType.sessionWarning:
        return AppColors.error;
      case BannerType.connection:
        return AppColors.onSurface;
    }
  }

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: onTap,
      child: Container(
        width: double.infinity,
        height: AppDimensions.bannerHeight,
        padding: const EdgeInsets.symmetric(horizontal: AppSpacing.md),
        decoration: BoxDecoration(
          color: _bgColor,
          border: _borderColor != null
              ? Border.all(color: _borderColor!)
              : null,
        ),
        child: Row(
          children: [
            if (icon != null) ...[
              Icon(icon, size: 18, color: _textColor),
              const SizedBox(width: AppSpacing.sm),
            ],
            Expanded(
              child: Text(
                text,
                style: AppTypography.caption.copyWith(color: _textColor),
                maxLines: 1,
                overflow: TextOverflow.ellipsis,
              ),
            ),
            if (onTap != null)
              Icon(Icons.chevron_right, size: 18, color: _textColor),
          ],
        ),
      ),
    );
  }
}
