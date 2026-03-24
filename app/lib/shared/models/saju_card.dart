import 'package:freezed_annotation/freezed_annotation.dart';

part 'saju_card.freezed.dart';
part 'saju_card.g.dart';

@freezed
class SajuCard with _$SajuCard {
  const factory SajuCard({
    required String id,
    required String iljuName,
    required String iljuHanja,
    required List<String> keywords,
    required String luckyElement,
    String? imageUrl,
    String? shareUrl,
    @Default(false) bool cached,
  }) = _SajuCard;

  factory SajuCard.fromJson(Map<String, dynamic> json) =>
      _$SajuCardFromJson(json);
}
