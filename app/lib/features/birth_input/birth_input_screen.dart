import 'package:flutter/material.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../core/theme/app_colors.dart';
import '../../core/theme/app_typography.dart';
import '../../core/theme/app_spacing.dart';
import '../../shared/models/birth_input.dart';
import '../../shared/widgets/widgets.dart';
import 'birth_hour_grid.dart';

/// Birth date input — reusable across card, consultation, compatibility
/// 양력/음력 toggle, 날짜 picker, 12시진 grid, gender
class BirthInputScreen extends ConsumerStatefulWidget {
  final String purpose; // 'card', 'consultation', 'compatibility'

  const BirthInputScreen({super.key, required this.purpose});

  @override
  ConsumerState<BirthInputScreen> createState() => _BirthInputScreenState();
}

class _BirthInputScreenState extends ConsumerState<BirthInputScreen> {
  CalendarType _calendarType = CalendarType.solar;
  bool _isLeapMonth = false;
  DateTime _selectedDate = DateTime(1995, 3, 15);
  Gender _gender = Gender.male;
  BirthHour _birthHour = BirthHour.unknown;
  String? _dateError;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => context.pop(),
          tooltip: '뒤로',
        ),
        title: Text(
          _titleForPurpose,
          style: AppTypography.bodySemiBold,
        ),
      ),
      body: SafeArea(
        child: Column(
          children: [
            Expanded(
              child: SingleChildScrollView(
                padding: const EdgeInsets.all(AppSpacing.md),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    // 날짜 유형 — 양력/음력 세그먼트 토글
                    Text('날짜 유형', style: AppTypography.bodySemiBold),
                    const SizedBox(height: AppSpacing.sm),
                    Row(
                      children: [
                        SegmentToggle<CalendarType>(
                          options: const [
                            SegmentOption(
                                value: CalendarType.solar, label: '양력'),
                            SegmentOption(
                                value: CalendarType.lunar, label: '음력'),
                          ],
                          selected: _calendarType,
                          onChanged: (v) =>
                              setState(() => _calendarType = v),
                        ),
                        if (_calendarType == CalendarType.lunar) ...[
                          const SizedBox(width: AppSpacing.md),
                          GestureDetector(
                            onTap: () => setState(
                                () => _isLeapMonth = !_isLeapMonth),
                            child: Row(
                              mainAxisSize: MainAxisSize.min,
                              children: [
                                SizedBox(
                                  width: 24,
                                  height: 24,
                                  child: Checkbox(
                                    value: _isLeapMonth,
                                    onChanged: (v) => setState(
                                        () => _isLeapMonth = v ?? false),
                                    activeColor: AppColors.accent,
                                  ),
                                ),
                                const SizedBox(width: AppSpacing.xs),
                                Text('윤달', style: AppTypography.body),
                              ],
                            ),
                          ),
                        ],
                      ],
                    ),

                    const SizedBox(height: AppSpacing.lg),

                    // 생년월일 — Native date picker
                    Text('생년월일', style: AppTypography.bodySemiBold),
                    const SizedBox(height: AppSpacing.sm),
                    _buildDatePicker(),
                    if (_dateError != null) ...[
                      const SizedBox(height: AppSpacing.xs),
                      Text(
                        _dateError!,
                        style: AppTypography.caption
                            .copyWith(color: AppColors.error),
                      ),
                    ],

                    const SizedBox(height: AppSpacing.lg),

                    // 성별 — 세그먼트 토글
                    Text('성별', style: AppTypography.bodySemiBold),
                    const SizedBox(height: AppSpacing.sm),
                    SegmentToggle<Gender>(
                      options: const [
                        SegmentOption(value: Gender.male, label: '남'),
                        SegmentOption(value: Gender.female, label: '여'),
                      ],
                      selected: _gender,
                      onChanged: (v) => setState(() => _gender = v),
                    ),

                    const SizedBox(height: AppSpacing.lg),

                    // 출생시간 — 12시진 그리드
                    Text('출생시간', style: AppTypography.bodySemiBold),
                    const SizedBox(height: AppSpacing.sm),
                    BirthHourGrid(
                      selected: _birthHour,
                      onChanged: (v) => setState(() => _birthHour = v),
                    ),

                    const SizedBox(height: AppSpacing.sm),
                    Text(
                      '가족에게 물어보세요. 출생시간이 있으면 더 정확한 분석을 받을 수 있습니다',
                      style: AppTypography.caption,
                    ),

                    const SizedBox(height: AppSpacing.lg),

                    // 확인 — 양력↔음력 양방향 표시
                    _buildConfirmation(),
                  ],
                ),
              ),
            ),

            // 하단 CTA
            Padding(
              padding: const EdgeInsets.all(AppSpacing.md),
              child: PrimaryButton(
                label: '다음',
                onPressed: _dateError == null ? _onSubmit : null,
              ),
            ),
          ],
        ),
      ),
    );
  }

  String get _titleForPurpose {
    switch (widget.purpose) {
      case 'consultation':
        return 'AI 사주 상담';
      case 'compatibility':
        return '궁합 확인';
      default:
        return '나의 사주 카드 만들기';
    }
  }

  Widget _buildDatePicker() {
    return SizedBox(
      height: 200,
      child: CupertinoDatePicker(
        mode: CupertinoDatePickerMode.date,
        initialDateTime: _selectedDate,
        minimumDate: DateTime(1900),
        maximumDate: DateTime(2100),
        onDateTimeChanged: (date) {
          setState(() {
            _selectedDate = date;
            _dateError = _validateDate(date);
          });
        },
      ),
    );
  }

  Widget _buildConfirmation() {
    final dateStr =
        '${_calendarType == CalendarType.solar ? "양력" : "음력"} '
        '${_selectedDate.year}년 ${_selectedDate.month}월 ${_selectedDate.day}일';
    final hourStr = _birthHour == BirthHour.unknown
        ? '출생시간 모름'
        : '${_birthHour.label}(${_birthHour.timeRange})';
    final genderStr = _gender == Gender.male ? '남성' : '여성';

    return Container(
      width: double.infinity,
      padding: const EdgeInsets.all(AppSpacing.md),
      decoration: BoxDecoration(
        color: AppColors.bannerInfoBg,
        borderRadius: BorderRadius.circular(AppRadius.button),
        border: Border.all(color: AppColors.divider),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text('확인', style: AppTypography.bodySemiBold),
          const SizedBox(height: AppSpacing.xs),
          Text(dateStr, style: AppTypography.body),
          Text('$hourStr · $genderStr', style: AppTypography.caption),
        ],
      ),
    );
  }

  /// 날짜 유효성 검사
  String? _validateDate(DateTime date) {
    final currentYear = DateTime.now().year;
    if (_calendarType == CalendarType.lunar) {
      // 백엔드 음력 변환 테이블 범위: 1920~2030
      if (date.year < 1920 || date.year > 2030) {
        return '음력은 1920년부터 2030년까지 입력 가능합니다';
      }
    } else {
      if (date.year < 1920 || date.year > currentYear) {
        return '1920년부터 ${currentYear}년까지 입력 가능합니다';
      }
    }
    return null;
  }

  /// 윤달 유효성 검사 — 음력이 아닌데 윤달 선택 시 에러
  String? _validateLeapMonth() {
    if (_isLeapMonth && _calendarType != CalendarType.lunar) {
      return '윤달은 음력에서만 선택할 수 있습니다';
    }
    return null;
  }

  void _onSubmit() {
    // 제출 시 전체 유효성 검사
    final dateValidation = _validateDate(_selectedDate);
    final leapValidation = _validateLeapMonth();
    final error = dateValidation ?? leapValidation;
    if (error != null) {
      setState(() => _dateError = error);
      return;
    }

    final input = BirthInput(
      year: _selectedDate.year,
      month: _selectedDate.month,
      day: _selectedDate.day,
      calendarType: _calendarType,
      isLeapMonth: _isLeapMonth,
      birthHour: _birthHour,
      gender: _gender,
    );

    switch (widget.purpose) {
      case 'consultation':
        context.push('/consultation/preview', extra: input);
      case 'compatibility':
        context.pop(input);
      default:
        // Saju card — create via provider and navigate
        context.push('/saju-card/new', extra: input);
    }
  }
}
