import 'package:freezed_annotation/freezed_annotation.dart';

part 'compatibility.freezed.dart';
part 'compatibility.g.dart';

@freezed
class CompatibilityPreview with _$CompatibilityPreview {
  const factory CompatibilityPreview({
    required int score,
    required String summary,
    required String person1Element,
    required String person2Element,
  }) = _CompatibilityPreview;

  factory CompatibilityPreview.fromJson(Map<String, dynamic> json) =>
      _$CompatibilityPreviewFromJson(json);
}
