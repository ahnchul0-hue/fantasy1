import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/api/api_client.dart';
import '../../shared/models/models.dart';

/// Chat messages for a consultation — loaded from server
final chatMessagesProvider = StateNotifierProvider.autoDispose
    .family<ChatMessagesNotifier, AsyncValue<List<ChatMessage>>, String>(
  (ref, consultationId) {
    final apiClient = ref.watch(apiClientProvider);
    return ChatMessagesNotifier(
      apiClient: apiClient,
      consultationId: consultationId,
    );
  },
);

class ChatMessagesNotifier
    extends StateNotifier<AsyncValue<List<ChatMessage>>> {
  final ApiClient _apiClient;
  final String consultationId;

  /// 최근 전송 에러 (null이면 에러 없음). UI에서 스낵바로 표시 용도.
  Object? _lastSendError;
  Object? get lastSendError => _lastSendError;

  /// 에러 확인 후 클리어
  void clearSendError() => _lastSendError = null;

  ChatMessagesNotifier({
    required ApiClient apiClient,
    required this.consultationId,
  })  : _apiClient = apiClient,
        super(const AsyncValue.loading()) {
    _loadMessages();
  }

  Future<void> _loadMessages() async {
    try {
      final messages =
          await _apiClient.getConsultationMessages(consultationId);
      state = AsyncValue.data(messages);
    } catch (e, st) {
      state = AsyncValue.error(e, st);
    }
  }

  Future<void> sendMessage(String content) async {
    if (content.trim().isEmpty) return;

    final currentMessages = state.valueOrNull ?? [];
    // Optimistic update — add user message immediately
    final userMessage = ChatMessage(
      id: 'temp_${DateTime.now().millisecondsSinceEpoch}',
      role: ChatRole.user,
      content: content.trim(),
      createdAt: DateTime.now(),
    );
    state = AsyncValue.data([...currentMessages, userMessage]);

    try {
      final response = await _apiClient.sendChatMessage(
        consultationId,
        {'message': content.trim()},
      );
      // Replace temp message with server response (contains AI reply)
      final updated =
          currentMessages.where((m) => m.id != userMessage.id).toList();
      // Add the confirmed user message and AI reply
      updated.add(response);
      // Reload full list to stay in sync
      await _loadMessages();
    } catch (e, st) {
      // Revert optimistic update — 메시지 목록은 복원하되 에러도 함께 표시
      // state를 error로 바꾸면 전체 채팅 UI가 사라지므로, 메시지는 유지하고
      // 마지막 메시지에 에러 표시를 위해 sendError를 별도 상태로 노출
      state = AsyncValue.data(currentMessages);
      _lastSendError = e;
    }
  }
}

/// Consultation session info (for expiry timer, turn count, etc.)
final consultationSessionProvider = FutureProvider.autoDispose
    .family<Consultation?, String>((ref, consultationId) async {
  final apiClient = ref.watch(apiClientProvider);
  return await apiClient.getConsultation(consultationId);
});
