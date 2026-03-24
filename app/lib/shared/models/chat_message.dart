import 'package:freezed_annotation/freezed_annotation.dart';

part 'chat_message.freezed.dart';
part 'chat_message.g.dart';

@freezed
class ChatMessage with _$ChatMessage {
  const factory ChatMessage({
    required String id,
    required ChatRole role,
    required String content,
    int? turnsRemaining,
    required DateTime createdAt,
    @Default(false) @JsonKey(includeFromJson: false, includeToJson: false) bool isStreaming,
  }) = _ChatMessage;

  factory ChatMessage.fromJson(Map<String, dynamic> json) =>
      _$ChatMessageFromJson(json);
}

@JsonEnum(valueField: 'value')
enum ChatRole {
  user('user'),
  assistant('assistant');

  final String value;
  const ChatRole(this.value);

  static ChatRole fromValue(String v) =>
      values.firstWhere((e) => e.value == v, orElse: () => assistant);
}
