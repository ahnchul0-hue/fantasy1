import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/api/api_client.dart';
import '../../shared/models/models.dart';

/// Chat messages provider for a specific consultation
final chatMessagesProvider = StateNotifierProvider.autoDispose
    .family<ChatNotifier, ChatState, String>((ref, consultationId) {
  return ChatNotifier(
    apiClient: ref.watch(apiClientProvider),
    consultationId: consultationId,
  );
});

class ChatState {
  final List<ChatMessage> messages;
  final int turnsRemaining;
  final bool isLoading;
  final String? error;

  const ChatState({
    this.messages = const [],
    this.turnsRemaining = 50,
    this.isLoading = false,
    this.error,
  });

  ChatState copyWith({
    List<ChatMessage>? messages,
    int? turnsRemaining,
    bool? isLoading,
    String? error,
  }) =>
      ChatState(
        messages: messages ?? this.messages,
        turnsRemaining: turnsRemaining ?? this.turnsRemaining,
        isLoading: isLoading ?? this.isLoading,
        error: error,
      );

  /// Turn warning ladder
  String? get turnWarning {
    if (turnsRemaining <= 0) return '세션이 만료되었습니다';
    if (turnsRemaining == 1) return '마지막 턴입니다';
    if (turnsRemaining <= 5) return '남은 턴: $turnsRemaining';
    if (turnsRemaining <= 10) return '남은 턴: $turnsRemaining';
    return null;
  }

  TurnWarningLevel get warningLevel {
    if (turnsRemaining <= 0) return TurnWarningLevel.expired;
    if (turnsRemaining == 1) return TurnWarningLevel.last;
    if (turnsRemaining <= 5) return TurnWarningLevel.critical;
    if (turnsRemaining <= 10) return TurnWarningLevel.warning;
    return TurnWarningLevel.normal;
  }
}

enum TurnWarningLevel { normal, warning, critical, last, expired }

class ChatNotifier extends StateNotifier<ChatState> {
  final ApiClient _apiClient;
  final String consultationId;

  ChatNotifier({
    required ApiClient apiClient,
    required this.consultationId,
  })  : _apiClient = apiClient,
        super(const ChatState()) {
    _addInitialMessage();
  }

  void _addInitialMessage() {
    state = state.copyWith(
      messages: [
        ChatMessage(
          id: 'initial',
          role: ChatRole.assistant,
          content: '무엇이 궁금하신지요?',
          createdAt: DateTime.now(),
        ),
      ],
    );
  }

  Future<void> sendMessage(String text) async {
    if (text.trim().isEmpty || state.turnsRemaining <= 0) return;

    // Add user message immediately
    final userMsg = ChatMessage(
      id: 'user_${DateTime.now().millisecondsSinceEpoch}',
      role: ChatRole.user,
      content: text.trim(),
      createdAt: DateTime.now(),
    );

    state = state.copyWith(
      messages: [...state.messages, userMsg],
      isLoading: true,
      error: null,
    );

    try {
      final response = await _apiClient.sendChatMessage(
        consultationId,
        {'message': text.trim()},
      );

      state = state.copyWith(
        messages: [...state.messages, response],
        turnsRemaining: response.turnsRemaining ?? state.turnsRemaining - 1,
        isLoading: false,
      );
    } catch (e) {
      state = state.copyWith(
        isLoading: false,
        error: '잠시 후 다시 시도해주세요',
      );
    }
  }
}
