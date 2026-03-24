import 'package:freezed_annotation/freezed_annotation.dart';

part 'birth_input.freezed.dart';
part 'birth_input.g.dart';

@freezed
class BirthInput with _$BirthInput {
  const factory BirthInput({
    required int year,
    required int month,
    required int day,
    required CalendarType calendarType,
    @Default(false) bool isLeapMonth,
    required BirthHour birthHour,
    required Gender gender,
  }) = _BirthInput;

  factory BirthInput.fromJson(Map<String, dynamic> json) =>
      _$BirthInputFromJson(json);
}

@JsonEnum(valueField: 'value')
enum CalendarType {
  solar('solar'),
  lunar('lunar');

  final String value;
  const CalendarType(this.value);

  static CalendarType fromValue(String v) =>
      values.firstWhere((e) => e.value == v, orElse: () => solar);
}

@JsonEnum(valueField: 'value')
enum Gender {
  male('male'),
  female('female');

  final String value;
  const Gender(this.value);

  static Gender fromValue(String v) =>
      values.firstWhere((e) => e.value == v, orElse: () => male);
}

/// 12시진 + unknown
@JsonEnum(valueField: 'value')
enum BirthHour {
  ja('ja', '자시', '23:00~01:00'),
  chuk('chuk', '축시', '01:00~03:00'),
  @JsonValue('in')
  in_('in', '인시', '03:00~05:00'),
  myo('myo', '묘시', '05:00~07:00'),
  jin('jin', '진시', '07:00~09:00'),
  sa('sa', '사시', '09:00~11:00'),
  o('o', '오시', '11:00~13:00'),
  mi('mi', '미시', '13:00~15:00'),
  sin('sin', '신시', '15:00~17:00'),
  yu('yu', '유시', '17:00~19:00'),
  sul('sul', '술시', '19:00~21:00'),
  hae('hae', '해시', '21:00~23:00'),
  unknown('unknown', '모르겠습니다', '');

  final String value;
  final String label;
  final String timeRange;
  const BirthHour(this.value, this.label, this.timeRange);

  static BirthHour fromValue(String v) =>
      values.firstWhere((e) => e.value == v, orElse: () => unknown);

  /// The 12 시진 only (excluding unknown)
  static List<BirthHour> get siJin =>
      values.where((e) => e != unknown).toList();
}
