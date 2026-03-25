import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../core/theme/app_colors.dart';
import '../../core/theme/app_typography.dart';
import '../../core/theme/app_spacing.dart';
import '../../shared/widgets/widgets.dart';
import '../../shared/models/chat_message.dart';
import '../../shared/models/consultation.dart';
import 'chat_providers.dart';

/// AI chat with 월하선생 persona
/// Turn counter, session timer from server expiry
class ChatScreen extends ConsumerStatefulWidget {
  final String consultationId;

  const ChatScreen({super.key, required this.consultationId});

  @override
  ConsumerState<ChatScreen> createState() => _ChatScreenState();
}

class _ChatScreenState extends ConsumerState<ChatScreen> {
  final _controller = TextEditingController();
  final _scrollController = ScrollController();
  final _focusNode = FocusNode();

  // Session timer — driven by server-provided expiresAt
  Timer? _timer;
  Duration? _remainingTime;

  @override
  void initState() {
    super.initState();
  }

  void _startTimerFromExpiry(DateTime expiresAt) {
    _timer?.cancel();
    final diff = expiresAt.difference(DateTime.now());
    _remainingTime = diff.isNegative ? Duration.zero : diff;

    _timer = Timer.periodic(const Duration(seconds: 60), (timer) {
      final remaining = expiresAt.difference(DateTime.now());
      if (remaining.inSeconds > 0) {
        setState(() {
          _remainingTime = remaining;
        });
      } else {
        setState(() {
          _remainingTime = Duration.zero;
        });
        timer.cancel();
      }
    });
  }

  @override
  void dispose() {
    _timer?.cancel();
    _controller.dispose();
    _scrollController.dispose();
    _focusNode.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final messagesAsync =
        ref.watch(chatMessagesProvider(widget.consultationId));
    final sessionAsync =
        ref.watch(consultationSessionProvider(widget.consultationId));

    // Start timer when session data arrives
    sessionAsync.whenData((consultation) {
      if (consultation?.expiresAt != null && _timer == null) {
        WidgetsBinding.instance.addPostFrameCallback((_) {
          _startTimerFromExpiry(consultation!.expiresAt!);
        });
      }
    });

    final turnsRemaining = sessionAsync.valueOrNull?.chatTurnsRemaining ?? 50;
    final isSessionExpired = sessionAsync.valueOrNull?.isExpired ?? false;
    final isTimeExpired = _remainingTime != null &&
        _remainingTime!.inSeconds <= 0;
    final isExpired = isSessionExpired || isTimeExpired;

    // Turn warning
    String? turnWarning;
    if (turnsRemaining <= 0) {
      turnWarning = '세션이 만료되었습니다';
    } else if (turnsRemaining == 1) {
      turnWarning = '마지막 턴입니다';
    } else if (turnsRemaining <= 5) {
      turnWarning = '남은 턴: $turnsRemaining';
    } else if (turnsRemaining <= 10) {
      turnWarning = '남은 턴: $turnsRemaining';
    }

    BannerType turnBannerType;
    if (turnsRemaining <= 0) {
      turnBannerType = BannerType.error;
    } else if (turnsRemaining <= 5) {
      turnBannerType = BannerType.sessionWarning;
    } else {
      turnBannerType = BannerType.info;
    }

    return Scaffold(
      appBar: AppBar(
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => context.pop(),
          tooltip: '뒤로',
        ),
        title: const Text('월하선생'),
        actions: [
          Padding(
            padding: const EdgeInsets.only(right: AppSpacing.md),
            child: Center(
              child: _remainingTime != null
                  ? Text(
                      _formatDuration(_remainingTime!),
                      style: AppTypography.caption.copyWith(
                        color: _remainingTime!.inMinutes <= 10
                            ? AppColors.error
                            : AppColors.secondaryText,
                      ),
                    )
                  : Text(
                      '상담 진행 중',
                      style: AppTypography.caption.copyWith(
                        color: AppColors.secondaryText,
                      ),
                    ),
            ),
          ),
        ],
      ),
      body: Column(
        children: [
          // Turn warning banner
          if (turnWarning != null)
            BannerWidget(
              text: turnWarning,
              type: turnBannerType,
            ),

          // Time warning banners
          if (_remainingTime != null &&
              _remainingTime!.inHours <= 24 &&
              _remainingTime!.inHours > 1 &&
              turnsRemaining > 10)
            BannerWidget(
              text: '상담 ${_remainingTime!.inHours}시간 남음',
              type: BannerType.info,
            ),
          if (_remainingTime != null &&
              _remainingTime!.inMinutes <= 60 &&
              _remainingTime!.inMinutes > 10)
            BannerWidget(
              text: '${_remainingTime!.inMinutes}분 남음. 마지막 질문을 해보세요',
              type: BannerType.sessionWarning,
            ),

          // Chat messages
          Expanded(
            child: messagesAsync.when(
              loading: () =>
                  const Center(child: CircularProgressIndicator()),
              error: (e, _) => ErrorRetryWidget(
                message: '메시지를 불러올 수 없습니다',
                onRetry: () => ref.invalidate(
                    chatMessagesProvider(widget.consultationId)),
              ),
              data: (messages) => ListView.builder(
                controller: _scrollController,
                padding:
                    const EdgeInsets.symmetric(vertical: AppSpacing.sm),
                itemCount: messages.length,
                itemBuilder: (context, index) {
                  return ChatBubble(message: messages[index]);
                },
              ),
            ),
          ),

          // Session expired state
          if (isExpired)
            Container(
              width: double.infinity,
              padding: const EdgeInsets.all(AppSpacing.md),
              color: AppColors.bannerInfoBg,
              child: Column(
                children: [
                  Text(
                    '상담이 종료되었습니다. 기록은 계속 열람 가능합니다',
                    style: AppTypography.body.copyWith(
                      color: AppColors.secondaryText,
                    ),
                    textAlign: TextAlign.center,
                  ),
                  const SizedBox(height: AppSpacing.sm),
                  PrimaryButton(
                    label: '새 상담 시작',
                    onPressed: () => context.go('/home'),
                  ),
                ],
              ),
            ),

          // Input area
          if (!isExpired) _buildInputArea(turnsRemaining),
        ],
      ),
    );
  }

  Widget _buildInputArea(int turnsRemaining) {
    final messagesAsync =
        ref.watch(chatMessagesProvider(widget.consultationId));
    final isSending = messagesAsync is AsyncLoading;

    return Container(
      padding: EdgeInsets.only(
        left: AppSpacing.md,
        right: AppSpacing.md,
        bottom: MediaQuery.of(context).padding.bottom + AppSpacing.sm,
        top: AppSpacing.sm,
      ),
      decoration: const BoxDecoration(
        color: AppColors.surface,
        border: Border(
          top: BorderSide(color: AppColors.divider),
        ),
      ),
      child: Column(
        children: [
          // Turn counter
          Padding(
            padding: const EdgeInsets.only(bottom: AppSpacing.xs),
            child: Text(
              '남은 턴: $turnsRemaining/50',
              style: AppTypography.caption,
            ),
          ),
          Row(
            children: [
              Expanded(
                child: TextField(
                  controller: _controller,
                  focusNode: _focusNode,
                  maxLength: 500,
                  maxLines: null,
                  decoration: InputDecoration(
                    hintText: '메시지를 입력하세요...',
                    counterText: '',
                    contentPadding: const EdgeInsets.symmetric(
                      horizontal: AppSpacing.md,
                      vertical: 12,
                    ),
                    border: OutlineInputBorder(
                      borderRadius:
                          BorderRadius.circular(AppRadius.input),
                      borderSide:
                          const BorderSide(color: AppColors.divider),
                    ),
                  ),
                  onSubmitted: (_) => _sendMessage(),
                ),
              ),
              const SizedBox(width: AppSpacing.sm),
              SizedBox(
                width: AppDimensions.touchTargetMin,
                height: AppDimensions.touchTargetMin,
                child: IconButton(
                  onPressed: isSending ? null : () => _sendMessage(),
                  style: IconButton.styleFrom(
                    backgroundColor: AppColors.accent,
                    foregroundColor: AppColors.primary,
                    disabledBackgroundColor: AppColors.disabled,
                  ),
                  icon: const Icon(Icons.send, size: 20),
                  tooltip: '전송',
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }

  void _sendMessage() {
    final text = _controller.text.trim();
    if (text.isEmpty) return;

    ref
        .read(chatMessagesProvider(widget.consultationId).notifier)
        .sendMessage(text);
    _controller.clear();

    // Scroll to bottom
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (_scrollController.hasClients) {
        _scrollController.animateTo(
          _scrollController.position.maxScrollExtent,
          duration: const Duration(milliseconds: 300),
          curve: Curves.easeOutCubic,
        );
      }
    });
  }

  String _formatDuration(Duration d) {
    final hours = d.inHours;
    final minutes = d.inMinutes % 60;
    return '$hours:${minutes.toString().padLeft(2, '0')}';
  }
}
