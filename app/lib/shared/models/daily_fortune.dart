import 'package:freezed_annotation/freezed_annotation.dart';

part 'daily_fortune.freezed.dart';
part 'daily_fortune.g.dart';

@freezed
class DailyFortune with _$DailyFortune {
  const factory DailyFortune({
    required String date,
    required String ilju,
    required String fortuneText,
    required String luckyColor,
    required int luckyNumber,
    required int overallScore,
  }) = _DailyFortune;

  factory DailyFortune.fromJson(Map<String, dynamic> json) =>
      _$DailyFortuneFromJson(json);
}
