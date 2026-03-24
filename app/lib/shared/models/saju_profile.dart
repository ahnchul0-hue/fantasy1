import 'package:freezed_annotation/freezed_annotation.dart';
import 'birth_input.dart';

part 'saju_profile.freezed.dart';
part 'saju_profile.g.dart';

@freezed
class SajuProfile with _$SajuProfile {
  const factory SajuProfile({
    required String id,
    required BirthInput birthInput,
    required FourPillars fourPillars,
    required OhengBalance ohengBalance,
  }) = _SajuProfile;

  factory SajuProfile.fromJson(Map<String, dynamic> json) =>
      _$SajuProfileFromJson(json);
}

@freezed
class Pillar with _$Pillar {
  const factory Pillar({
    required String heavenlyStem,
    required String earthlyBranch,
    required String heavenlyStemHanja,
    required String earthlyBranchHanja,
  }) = _Pillar;

  factory Pillar.fromJson(Map<String, dynamic> json) =>
      _$PillarFromJson(json);
}

@freezed
class FourPillars with _$FourPillars {
  const factory FourPillars({
    required Pillar year,
    required Pillar month,
    required Pillar day,
    Pillar? hour,
  }) = _FourPillars;

  factory FourPillars.fromJson(Map<String, dynamic> json) =>
      _$FourPillarsFromJson(json);
}

@freezed
class OhengBalance with _$OhengBalance {
  const OhengBalance._();

  const factory OhengBalance({
    required double wood,
    required double fire,
    required double earth,
    required double metal,
    required double water,
  }) = _OhengBalance;

  factory OhengBalance.fromJson(Map<String, dynamic> json) =>
      _$OhengBalanceFromJson(json);

  double get total => wood + fire + earth + metal + water;

  List<OhengEntry> get entries => [
        OhengEntry('木', '목', wood, total),
        OhengEntry('火', '화', fire, total),
        OhengEntry('土', '토', earth, total),
        OhengEntry('金', '금', metal, total),
        OhengEntry('水', '수', water, total),
      ];
}

class OhengEntry {
  final String hanja;
  final String hangul;
  final double value;
  final double total;

  const OhengEntry(this.hanja, this.hangul, this.value, this.total);

  double get ratio => total > 0 ? value / total : 0;
}
