import 'package:freezed_annotation/freezed_annotation.dart';

part 'consultation.freezed.dart';
part 'consultation.g.dart';

@freezed
class Consultation with _$Consultation {
  const Consultation._();

  const factory Consultation({
    required String id,
    required ConsultationStatus status,
    @Default([]) List<String> resultImages,
    String? analysisSummary,
    @Default(50) int chatTurnsRemaining,
    DateTime? expiresAt,
    @Default(CheckpointStatus.none) CheckpointStatus checkpointStatus,
  }) = _Consultation;

  factory Consultation.fromJson(Map<String, dynamic> json) =>
      _$ConsultationFromJson(json);

  bool get isActive =>
      status == ConsultationStatus.ready &&
      expiresAt != null &&
      expiresAt!.isAfter(DateTime.now());

  bool get isExpired =>
      status == ConsultationStatus.expired ||
      (expiresAt != null && expiresAt!.isBefore(DateTime.now()));

  Duration? get remainingTime {
    if (expiresAt == null) return null;
    final diff = expiresAt!.difference(DateTime.now());
    return diff.isNegative ? Duration.zero : diff;
  }
}

@JsonEnum(valueField: 'value')
enum ConsultationStatus {
  generating('generating'),
  ready('ready'),
  expired('expired'),
  failed('failed');

  final String value;
  const ConsultationStatus(this.value);

  static ConsultationStatus fromValue(String v) =>
      values.firstWhere((e) => e.value == v, orElse: () => generating);
}

@JsonEnum(valueField: 'value')
enum CheckpointStatus {
  none('none'),
  analysisDone('analysis_done'),
  imagesDone('images_done'),
  complete('complete');

  final String value;
  const CheckpointStatus(this.value);

  static CheckpointStatus fromValue(String v) =>
      values.firstWhere((e) => e.value == v, orElse: () => none);
}
