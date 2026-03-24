import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../core/theme/app_colors.dart';
import '../../core/theme/app_typography.dart';
import '../../core/theme/app_spacing.dart';
import '../../shared/widgets/widgets.dart';
import '../../shared/models/chat_message.dart';
import 'chat_providers.dart';

/// AI chat with 월하선생 persona
/// Turn counter, session timer, warning ladder
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

  // Session timer (72h) — demo countdown
  Timer? _timer;
  Duration _remainingTime = const Duration(hours: 72);

  @override
  void initState() {
    super.initState();
    _startTimer();
  }

  void _startTimer() {
    _timer = Timer.periodic(const Duration(seconds: 1), (timer) {
      if (_remainingTime.inSeconds > 0) {
        setState(() {
          _remainingTime = _remainingTime - const Duration(seconds: 1);
        });
      } else {
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
    final chatState =
        ref.watch(chatMessagesProvider(widget.consultationId));
    final isExpired =
        chatState.warningLevel == TurnWarningLevel.expired ||
            _remainingTime.inSeconds <= 0;

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
              child: Text(
                _formatDuration(_remainingTime),
                style: AppTypography.caption.copyWith(
                  color: _remainingTime.inMinutes <= 10
                      ? AppColors.error
                      : AppColors.secondaryText,
                ),
              ),
            ),
          ),
        ],
      ),
      body: Column(
        children: [
          // Session / turn warning banner
          if (chatState.turnWarning != null)
            BannerWidget(
              text: chatState.turnWarning!,
              type: chatState.warningLevel == TurnWarningLevel.expired
                  ? BannerType.error
                  : chatState.warningLevel == TurnWarningLevel.last ||
                          chatState.warningLevel ==
                              TurnWarningLevel.critical
                      ? BannerType.sessionWarning
                      : BannerType.info,
            ),

          // Time warning banners
          if (_remainingTime.inHours <= 24 &&
              _remainingTime.inHours > 1 &&
              chatState.warningLevel == TurnWarningLevel.normal)
            BannerWidget(
              text: '상담 ${_remainingTime.inHours}시간 남음',
              type: BannerType.info,
            ),
          if (_remainingTime.inMinutes <= 60 &&
              _remainingTime.inMinutes > 10)
            BannerWidget(
              text: '${_remainingTime.inMinutes}분 남음. 마지막 질문을 해보세요',
              type: BannerType.sessionWarning,
            ),

          // Chat messages
          Expanded(
            child: ListView.builder(
              controller: _scrollController,
              padding: const EdgeInsets.symmetric(vertical: AppSpacing.sm),
              itemCount: chatState.messages.length +
                  (chatState.isLoading ? 1 : 0),
              itemBuilder: (context, index) {
                if (index == chatState.messages.length &&
                    chatState.isLoading) {
                  // Typing indicator
                  return ChatBubble(
                    message: ChatMessage(
                      id: 'typing',
                      role: ChatRole.assistant,
                      content: '',
                      createdAt: DateTime.now(),
                    ),
                    showTypingIndicator: true,
                  );
                }
                return ChatBubble(message: chatState.messages[index]);
              },
            ),
          ),

          // Error message
          if (chatState.error != null)
            Padding(
              padding: const EdgeInsets.symmetric(
                horizontal: AppSpacing.md,
              ),
              child: Text(
                chatState.error!,
                style: AppTypography.caption
                    .copyWith(color: AppColors.error),
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
          if (!isExpired) _buildInputArea(chatState),
        ],
      ),
    );
  }

  Widget _buildInputArea(ChatState chatState) {
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
              '남은 턴: ${chatState.turnsRemaining}/50',
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
                  onSubmitted: (_) => _sendMessage(chatState),
                ),
              ),
              const SizedBox(width: AppSpacing.sm),
              SizedBox(
                width: AppDimensions.touchTargetMin,
                height: AppDimensions.touchTargetMin,
                child: IconButton(
                  onPressed: chatState.isLoading
                      ? null
                      : () => _sendMessage(chatState),
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

  void _sendMessage(ChatState chatState) {
    final text = _controller.text.trim();
    if (text.isEmpty || chatState.isLoading) return;

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
