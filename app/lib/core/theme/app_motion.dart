import 'package:flutter/material.dart';
import 'package:flutter/scheduler.dart';

/// Motion tokens from tokens.json
/// reduced-motion: prefers-reduced-motion -> duration 0
class AppMotion {
  AppMotion._();

  static const Curve defaultCurve = Curves.easeOutCubic;
  static const Curve chartCurve = Curves.easeOutBack;

  // Durations (raw values)
  static const int _defaultDurationMs = 300;
  static const int _cardRevealDurationMs = 200;
  static const int _cardRevealStaggerMs = 100;
  static const int _chatMessageDurationMs = 150;
  static const int _chartAnimationDurationMs = 600;

  /// Check if reduced motion is requested
  static bool get reducedMotion {
    return SchedulerBinding.instance.disableAnimations;
  }

  static Duration get defaultDuration => Duration(
        milliseconds: reducedMotion ? 0 : _defaultDurationMs,
      );

  static Duration get cardRevealDuration => Duration(
        milliseconds: reducedMotion ? 0 : _cardRevealDurationMs,
      );

  static Duration get cardRevealStagger => Duration(
        milliseconds: reducedMotion ? 0 : _cardRevealStaggerMs,
      );

  static Duration get chatMessageDuration => Duration(
        milliseconds: reducedMotion ? 0 : _chatMessageDurationMs,
      );

  static Duration get chartAnimationDuration => Duration(
        milliseconds: reducedMotion ? 0 : _chartAnimationDurationMs,
      );
}
