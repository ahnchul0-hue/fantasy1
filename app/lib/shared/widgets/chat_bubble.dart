import 'package:flutter/material.dart';
import '../../core/theme/app_colors.dart';
import '../../core/theme/app_typography.dart';
import '../../core/theme/app_spacing.dart';
import '../../core/theme/app_motion.dart';
import '../../shared/models/chat_message.dart';

/// Chat bubble — AI: left bg #F5F0EB r16/16/16/4
/// User: right bg #1A1A2E text #F5F0EB r16/16/4/16
/// 좌측에서 슬라이드인 (AI) / 우측에서 슬라이드인 (User)
class ChatBubble extends StatefulWidget {
  final ChatMessage message;
  final bool showTypingIndicator;

  const ChatBubble({
    super.key,
    required this.message,
    this.showTypingIndicator = false,
  });

  @override
  State<ChatBubble> createState() => _ChatBubbleState();
}

class _ChatBubbleState extends State<ChatBubble> {
  bool _visible = false;

  bool get _isUser => widget.message.role == ChatRole.user;

  @override
  void initState() {
    super.initState();
    // 1프레임 후 슬라이드인 트리거
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted) setState(() => _visible = true);
    });
  }

  @override
  Widget build(BuildContext context) {
    return AnimatedSlide(
      offset: _visible
          ? Offset.zero
          : Offset(_isUser ? 0.3 : -0.3, 0),
      duration: AppMotion.chatMessageDuration,
      curve: AppMotion.defaultCurve,
      child: AnimatedOpacity(
        opacity: _visible ? 1.0 : 0.0,
        duration: AppMotion.chatMessageDuration,
        child: Padding(
        padding: const EdgeInsets.symmetric(
          horizontal: AppSpacing.md,
          vertical: AppSpacing.xs,
        ),
        child: Row(
          mainAxisAlignment:
              _isUser ? MainAxisAlignment.end : MainAxisAlignment.start,
          crossAxisAlignment: CrossAxisAlignment.end,
          children: [
            if (!_isUser) ...[
              // AI avatar placeholder
              Container(
                width: 32,
                height: 32,
                decoration: BoxDecoration(
                  color: AppColors.primary,
                  borderRadius: BorderRadius.circular(16),
                ),
                child: const Center(
                  child: Text(
                    '月',
                    style: TextStyle(
                      fontFamily: AppTypography.fontHanja,
                      fontSize: 16,
                      color: AppColors.surface,
                    ),
                  ),
                ),
              ),
              const SizedBox(width: AppSpacing.sm),
            ],
            Flexible(
              child: Container(
                constraints: BoxConstraints(
                  maxWidth: MediaQuery.of(context).size.width * 0.72,
                ),
                padding: const EdgeInsets.symmetric(
                  horizontal: AppSpacing.md,
                  vertical: 12,
                ),
                decoration: BoxDecoration(
                  color: _isUser
                      ? AppColors.chatUserBg
                      : AppColors.chatAiBg,
                  borderRadius: _isUser
                      ? const BorderRadius.only(
                          topLeft: Radius.circular(16),
                          topRight: Radius.circular(16),
                          bottomLeft: Radius.circular(16),
                          bottomRight: Radius.circular(4),
                        )
                      : const BorderRadius.only(
                          topLeft: Radius.circular(16),
                          topRight: Radius.circular(16),
                          bottomLeft: Radius.circular(4),
                          bottomRight: Radius.circular(16),
                        ),
                ),
                child: widget.showTypingIndicator
                    ? const _TypingIndicator()
                    : Text(
                        widget.message.content,
                        style: AppTypography.body.copyWith(
                          color: _isUser
                              ? AppColors.chatUserText
                              : AppColors.onSurface,
                        ),
                      ),
              ),
            ),
          ],
        ),
      ),
      ),
    );
  }
}

/// Typing indicator — 3 animated dots
class _TypingIndicator extends StatefulWidget {
  const _TypingIndicator();

  @override
  State<_TypingIndicator> createState() => _TypingIndicatorState();
}

class _TypingIndicatorState extends State<_TypingIndicator>
    with TickerProviderStateMixin {
  late final List<AnimationController> _controllers;
  late final List<Animation<double>> _animations;

  @override
  void initState() {
    super.initState();
    _controllers = List.generate(3, (i) {
      return AnimationController(
        vsync: this,
        duration: const Duration(milliseconds: 600),
      );
    });
    _animations = _controllers.map((c) {
      return Tween<double>(begin: 0, end: -6).animate(
        CurvedAnimation(parent: c, curve: Curves.easeInOut),
      );
    }).toList();

    for (int i = 0; i < 3; i++) {
      Future.delayed(Duration(milliseconds: i * 200), () {
        if (mounted) _controllers[i].repeat(reverse: true);
      });
    }
  }

  @override
  void dispose() {
    for (final c in _controllers) {
      c.dispose();
    }
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 24,
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: List.generate(3, (i) {
          return AnimatedBuilder(
            animation: _animations[i],
            builder: (_, child) => Transform.translate(
              offset: Offset(0, _animations[i].value),
              child: child,
            ),
            child: Container(
              width: 8,
              height: 8,
              margin: const EdgeInsets.symmetric(horizontal: 2),
              decoration: BoxDecoration(
                color: AppColors.disabled,
                borderRadius: BorderRadius.circular(4),
              ),
            ),
          );
        }),
      ),
    );
  }
}

/// Wrapper for AnimatedBuilder using Animation
class AnimatedBuilder extends AnimatedWidget {
  final Widget Function(BuildContext, Widget?) builder;
  final Widget? child;

  const AnimatedBuilder({
    super.key,
    required Animation<double> animation,
    required this.builder,
    this.child,
  }) : super(listenable: animation);

  @override
  Widget build(BuildContext context) {
    return builder(context, child);
  }
}
