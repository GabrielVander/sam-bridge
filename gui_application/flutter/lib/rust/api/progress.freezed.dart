// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'progress.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$AssessStudentProgressOutcome {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AssessStudentProgressOutcome);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AssessStudentProgressOutcome()';
}


}

/// @nodoc
class $AssessStudentProgressOutcomeCopyWith<$Res>  {
$AssessStudentProgressOutcomeCopyWith(AssessStudentProgressOutcome _, $Res Function(AssessStudentProgressOutcome) __);
}


/// Adds pattern-matching-related methods to [AssessStudentProgressOutcome].
extension AssessStudentProgressOutcomePatterns on AssessStudentProgressOutcome {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( AssessStudentProgressOutcome_Success value)?  success,TResult Function( AssessStudentProgressOutcome_NoInstrumentAssigned value)?  noInstrumentAssigned,TResult Function( AssessStudentProgressOutcome_UnknownLevel value)?  unknownLevel,TResult Function( AssessStudentProgressOutcome_NotAMusician value)?  notAMusician,TResult Function( AssessStudentProgressOutcome_Failure value)?  failure,required TResult orElse(),}){
final _that = this;
switch (_that) {
case AssessStudentProgressOutcome_Success() when success != null:
return success(_that);case AssessStudentProgressOutcome_NoInstrumentAssigned() when noInstrumentAssigned != null:
return noInstrumentAssigned(_that);case AssessStudentProgressOutcome_UnknownLevel() when unknownLevel != null:
return unknownLevel(_that);case AssessStudentProgressOutcome_NotAMusician() when notAMusician != null:
return notAMusician(_that);case AssessStudentProgressOutcome_Failure() when failure != null:
return failure(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( AssessStudentProgressOutcome_Success value)  success,required TResult Function( AssessStudentProgressOutcome_NoInstrumentAssigned value)  noInstrumentAssigned,required TResult Function( AssessStudentProgressOutcome_UnknownLevel value)  unknownLevel,required TResult Function( AssessStudentProgressOutcome_NotAMusician value)  notAMusician,required TResult Function( AssessStudentProgressOutcome_Failure value)  failure,}){
final _that = this;
switch (_that) {
case AssessStudentProgressOutcome_Success():
return success(_that);case AssessStudentProgressOutcome_NoInstrumentAssigned():
return noInstrumentAssigned(_that);case AssessStudentProgressOutcome_UnknownLevel():
return unknownLevel(_that);case AssessStudentProgressOutcome_NotAMusician():
return notAMusician(_that);case AssessStudentProgressOutcome_Failure():
return failure(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( AssessStudentProgressOutcome_Success value)?  success,TResult? Function( AssessStudentProgressOutcome_NoInstrumentAssigned value)?  noInstrumentAssigned,TResult? Function( AssessStudentProgressOutcome_UnknownLevel value)?  unknownLevel,TResult? Function( AssessStudentProgressOutcome_NotAMusician value)?  notAMusician,TResult? Function( AssessStudentProgressOutcome_Failure value)?  failure,}){
final _that = this;
switch (_that) {
case AssessStudentProgressOutcome_Success() when success != null:
return success(_that);case AssessStudentProgressOutcome_NoInstrumentAssigned() when noInstrumentAssigned != null:
return noInstrumentAssigned(_that);case AssessStudentProgressOutcome_UnknownLevel() when unknownLevel != null:
return unknownLevel(_that);case AssessStudentProgressOutcome_NotAMusician() when notAMusician != null:
return notAMusician(_that);case AssessStudentProgressOutcome_Failure() when failure != null:
return failure(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( ProgressAssessmentDto assessment)?  success,TResult Function()?  noInstrumentAssigned,TResult Function( String rawLevel)?  unknownLevel,TResult Function()?  notAMusician,TResult Function( ErrorReportDto report)?  failure,required TResult orElse(),}) {final _that = this;
switch (_that) {
case AssessStudentProgressOutcome_Success() when success != null:
return success(_that.assessment);case AssessStudentProgressOutcome_NoInstrumentAssigned() when noInstrumentAssigned != null:
return noInstrumentAssigned();case AssessStudentProgressOutcome_UnknownLevel() when unknownLevel != null:
return unknownLevel(_that.rawLevel);case AssessStudentProgressOutcome_NotAMusician() when notAMusician != null:
return notAMusician();case AssessStudentProgressOutcome_Failure() when failure != null:
return failure(_that.report);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( ProgressAssessmentDto assessment)  success,required TResult Function()  noInstrumentAssigned,required TResult Function( String rawLevel)  unknownLevel,required TResult Function()  notAMusician,required TResult Function( ErrorReportDto report)  failure,}) {final _that = this;
switch (_that) {
case AssessStudentProgressOutcome_Success():
return success(_that.assessment);case AssessStudentProgressOutcome_NoInstrumentAssigned():
return noInstrumentAssigned();case AssessStudentProgressOutcome_UnknownLevel():
return unknownLevel(_that.rawLevel);case AssessStudentProgressOutcome_NotAMusician():
return notAMusician();case AssessStudentProgressOutcome_Failure():
return failure(_that.report);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( ProgressAssessmentDto assessment)?  success,TResult? Function()?  noInstrumentAssigned,TResult? Function( String rawLevel)?  unknownLevel,TResult? Function()?  notAMusician,TResult? Function( ErrorReportDto report)?  failure,}) {final _that = this;
switch (_that) {
case AssessStudentProgressOutcome_Success() when success != null:
return success(_that.assessment);case AssessStudentProgressOutcome_NoInstrumentAssigned() when noInstrumentAssigned != null:
return noInstrumentAssigned();case AssessStudentProgressOutcome_UnknownLevel() when unknownLevel != null:
return unknownLevel(_that.rawLevel);case AssessStudentProgressOutcome_NotAMusician() when notAMusician != null:
return notAMusician();case AssessStudentProgressOutcome_Failure() when failure != null:
return failure(_that.report);case _:
  return null;

}
}

}

/// @nodoc


class AssessStudentProgressOutcome_Success extends AssessStudentProgressOutcome {
  const AssessStudentProgressOutcome_Success({required this.assessment}): super._();
  

 final  ProgressAssessmentDto assessment;

/// Create a copy of AssessStudentProgressOutcome
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AssessStudentProgressOutcome_SuccessCopyWith<AssessStudentProgressOutcome_Success> get copyWith => _$AssessStudentProgressOutcome_SuccessCopyWithImpl<AssessStudentProgressOutcome_Success>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AssessStudentProgressOutcome_Success&&(identical(other.assessment, assessment) || other.assessment == assessment));
}


@override
int get hashCode => Object.hash(runtimeType,assessment);

@override
String toString() {
  return 'AssessStudentProgressOutcome.success(assessment: $assessment)';
}


}

/// @nodoc
abstract mixin class $AssessStudentProgressOutcome_SuccessCopyWith<$Res> implements $AssessStudentProgressOutcomeCopyWith<$Res> {
  factory $AssessStudentProgressOutcome_SuccessCopyWith(AssessStudentProgressOutcome_Success value, $Res Function(AssessStudentProgressOutcome_Success) _then) = _$AssessStudentProgressOutcome_SuccessCopyWithImpl;
@useResult
$Res call({
 ProgressAssessmentDto assessment
});




}
/// @nodoc
class _$AssessStudentProgressOutcome_SuccessCopyWithImpl<$Res>
    implements $AssessStudentProgressOutcome_SuccessCopyWith<$Res> {
  _$AssessStudentProgressOutcome_SuccessCopyWithImpl(this._self, this._then);

  final AssessStudentProgressOutcome_Success _self;
  final $Res Function(AssessStudentProgressOutcome_Success) _then;

/// Create a copy of AssessStudentProgressOutcome
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? assessment = null,}) {
  return _then(AssessStudentProgressOutcome_Success(
assessment: null == assessment ? _self.assessment : assessment // ignore: cast_nullable_to_non_nullable
as ProgressAssessmentDto,
  ));
}


}

/// @nodoc


class AssessStudentProgressOutcome_NoInstrumentAssigned extends AssessStudentProgressOutcome {
  const AssessStudentProgressOutcome_NoInstrumentAssigned(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AssessStudentProgressOutcome_NoInstrumentAssigned);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AssessStudentProgressOutcome.noInstrumentAssigned()';
}


}




/// @nodoc


class AssessStudentProgressOutcome_UnknownLevel extends AssessStudentProgressOutcome {
  const AssessStudentProgressOutcome_UnknownLevel({required this.rawLevel}): super._();
  

 final  String rawLevel;

/// Create a copy of AssessStudentProgressOutcome
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AssessStudentProgressOutcome_UnknownLevelCopyWith<AssessStudentProgressOutcome_UnknownLevel> get copyWith => _$AssessStudentProgressOutcome_UnknownLevelCopyWithImpl<AssessStudentProgressOutcome_UnknownLevel>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AssessStudentProgressOutcome_UnknownLevel&&(identical(other.rawLevel, rawLevel) || other.rawLevel == rawLevel));
}


@override
int get hashCode => Object.hash(runtimeType,rawLevel);

@override
String toString() {
  return 'AssessStudentProgressOutcome.unknownLevel(rawLevel: $rawLevel)';
}


}

/// @nodoc
abstract mixin class $AssessStudentProgressOutcome_UnknownLevelCopyWith<$Res> implements $AssessStudentProgressOutcomeCopyWith<$Res> {
  factory $AssessStudentProgressOutcome_UnknownLevelCopyWith(AssessStudentProgressOutcome_UnknownLevel value, $Res Function(AssessStudentProgressOutcome_UnknownLevel) _then) = _$AssessStudentProgressOutcome_UnknownLevelCopyWithImpl;
@useResult
$Res call({
 String rawLevel
});




}
/// @nodoc
class _$AssessStudentProgressOutcome_UnknownLevelCopyWithImpl<$Res>
    implements $AssessStudentProgressOutcome_UnknownLevelCopyWith<$Res> {
  _$AssessStudentProgressOutcome_UnknownLevelCopyWithImpl(this._self, this._then);

  final AssessStudentProgressOutcome_UnknownLevel _self;
  final $Res Function(AssessStudentProgressOutcome_UnknownLevel) _then;

/// Create a copy of AssessStudentProgressOutcome
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? rawLevel = null,}) {
  return _then(AssessStudentProgressOutcome_UnknownLevel(
rawLevel: null == rawLevel ? _self.rawLevel : rawLevel // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class AssessStudentProgressOutcome_NotAMusician extends AssessStudentProgressOutcome {
  const AssessStudentProgressOutcome_NotAMusician(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AssessStudentProgressOutcome_NotAMusician);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AssessStudentProgressOutcome.notAMusician()';
}


}




/// @nodoc


class AssessStudentProgressOutcome_Failure extends AssessStudentProgressOutcome {
  const AssessStudentProgressOutcome_Failure({required this.report}): super._();
  

 final  ErrorReportDto report;

/// Create a copy of AssessStudentProgressOutcome
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AssessStudentProgressOutcome_FailureCopyWith<AssessStudentProgressOutcome_Failure> get copyWith => _$AssessStudentProgressOutcome_FailureCopyWithImpl<AssessStudentProgressOutcome_Failure>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AssessStudentProgressOutcome_Failure&&(identical(other.report, report) || other.report == report));
}


@override
int get hashCode => Object.hash(runtimeType,report);

@override
String toString() {
  return 'AssessStudentProgressOutcome.failure(report: $report)';
}


}

/// @nodoc
abstract mixin class $AssessStudentProgressOutcome_FailureCopyWith<$Res> implements $AssessStudentProgressOutcomeCopyWith<$Res> {
  factory $AssessStudentProgressOutcome_FailureCopyWith(AssessStudentProgressOutcome_Failure value, $Res Function(AssessStudentProgressOutcome_Failure) _then) = _$AssessStudentProgressOutcome_FailureCopyWithImpl;
@useResult
$Res call({
 ErrorReportDto report
});




}
/// @nodoc
class _$AssessStudentProgressOutcome_FailureCopyWithImpl<$Res>
    implements $AssessStudentProgressOutcome_FailureCopyWith<$Res> {
  _$AssessStudentProgressOutcome_FailureCopyWithImpl(this._self, this._then);

  final AssessStudentProgressOutcome_Failure _self;
  final $Res Function(AssessStudentProgressOutcome_Failure) _then;

/// Create a copy of AssessStudentProgressOutcome
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? report = null,}) {
  return _then(AssessStudentProgressOutcome_Failure(
report: null == report ? _self.report : report // ignore: cast_nullable_to_non_nullable
as ErrorReportDto,
  ));
}


}

/// @nodoc
mixin _$MusicianLevelDto {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is MusicianLevelDto);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'MusicianLevelDto()';
}


}

/// @nodoc
class $MusicianLevelDtoCopyWith<$Res>  {
$MusicianLevelDtoCopyWith(MusicianLevelDto _, $Res Function(MusicianLevelDto) __);
}


/// Adds pattern-matching-related methods to [MusicianLevelDto].
extension MusicianLevelDtoPatterns on MusicianLevelDto {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( MusicianLevelDto_Candidate value)?  candidate,TResult Function( MusicianLevelDto_Practice value)?  practice,TResult Function( MusicianLevelDto_YouthService value)?  youthService,TResult Function( MusicianLevelDto_OfficialService value)?  officialService,TResult Function( MusicianLevelDto_Officialized value)?  officialized,TResult Function( MusicianLevelDto_Unknown value)?  unknown,required TResult orElse(),}){
final _that = this;
switch (_that) {
case MusicianLevelDto_Candidate() when candidate != null:
return candidate(_that);case MusicianLevelDto_Practice() when practice != null:
return practice(_that);case MusicianLevelDto_YouthService() when youthService != null:
return youthService(_that);case MusicianLevelDto_OfficialService() when officialService != null:
return officialService(_that);case MusicianLevelDto_Officialized() when officialized != null:
return officialized(_that);case MusicianLevelDto_Unknown() when unknown != null:
return unknown(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( MusicianLevelDto_Candidate value)  candidate,required TResult Function( MusicianLevelDto_Practice value)  practice,required TResult Function( MusicianLevelDto_YouthService value)  youthService,required TResult Function( MusicianLevelDto_OfficialService value)  officialService,required TResult Function( MusicianLevelDto_Officialized value)  officialized,required TResult Function( MusicianLevelDto_Unknown value)  unknown,}){
final _that = this;
switch (_that) {
case MusicianLevelDto_Candidate():
return candidate(_that);case MusicianLevelDto_Practice():
return practice(_that);case MusicianLevelDto_YouthService():
return youthService(_that);case MusicianLevelDto_OfficialService():
return officialService(_that);case MusicianLevelDto_Officialized():
return officialized(_that);case MusicianLevelDto_Unknown():
return unknown(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( MusicianLevelDto_Candidate value)?  candidate,TResult? Function( MusicianLevelDto_Practice value)?  practice,TResult? Function( MusicianLevelDto_YouthService value)?  youthService,TResult? Function( MusicianLevelDto_OfficialService value)?  officialService,TResult? Function( MusicianLevelDto_Officialized value)?  officialized,TResult? Function( MusicianLevelDto_Unknown value)?  unknown,}){
final _that = this;
switch (_that) {
case MusicianLevelDto_Candidate() when candidate != null:
return candidate(_that);case MusicianLevelDto_Practice() when practice != null:
return practice(_that);case MusicianLevelDto_YouthService() when youthService != null:
return youthService(_that);case MusicianLevelDto_OfficialService() when officialService != null:
return officialService(_that);case MusicianLevelDto_Officialized() when officialized != null:
return officialized(_that);case MusicianLevelDto_Unknown() when unknown != null:
return unknown(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function()?  candidate,TResult Function()?  practice,TResult Function()?  youthService,TResult Function()?  officialService,TResult Function()?  officialized,TResult Function( String raw)?  unknown,required TResult orElse(),}) {final _that = this;
switch (_that) {
case MusicianLevelDto_Candidate() when candidate != null:
return candidate();case MusicianLevelDto_Practice() when practice != null:
return practice();case MusicianLevelDto_YouthService() when youthService != null:
return youthService();case MusicianLevelDto_OfficialService() when officialService != null:
return officialService();case MusicianLevelDto_Officialized() when officialized != null:
return officialized();case MusicianLevelDto_Unknown() when unknown != null:
return unknown(_that.raw);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function()  candidate,required TResult Function()  practice,required TResult Function()  youthService,required TResult Function()  officialService,required TResult Function()  officialized,required TResult Function( String raw)  unknown,}) {final _that = this;
switch (_that) {
case MusicianLevelDto_Candidate():
return candidate();case MusicianLevelDto_Practice():
return practice();case MusicianLevelDto_YouthService():
return youthService();case MusicianLevelDto_OfficialService():
return officialService();case MusicianLevelDto_Officialized():
return officialized();case MusicianLevelDto_Unknown():
return unknown(_that.raw);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function()?  candidate,TResult? Function()?  practice,TResult? Function()?  youthService,TResult? Function()?  officialService,TResult? Function()?  officialized,TResult? Function( String raw)?  unknown,}) {final _that = this;
switch (_that) {
case MusicianLevelDto_Candidate() when candidate != null:
return candidate();case MusicianLevelDto_Practice() when practice != null:
return practice();case MusicianLevelDto_YouthService() when youthService != null:
return youthService();case MusicianLevelDto_OfficialService() when officialService != null:
return officialService();case MusicianLevelDto_Officialized() when officialized != null:
return officialized();case MusicianLevelDto_Unknown() when unknown != null:
return unknown(_that.raw);case _:
  return null;

}
}

}

/// @nodoc


class MusicianLevelDto_Candidate extends MusicianLevelDto {
  const MusicianLevelDto_Candidate(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is MusicianLevelDto_Candidate);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'MusicianLevelDto.candidate()';
}


}




/// @nodoc


class MusicianLevelDto_Practice extends MusicianLevelDto {
  const MusicianLevelDto_Practice(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is MusicianLevelDto_Practice);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'MusicianLevelDto.practice()';
}


}




/// @nodoc


class MusicianLevelDto_YouthService extends MusicianLevelDto {
  const MusicianLevelDto_YouthService(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is MusicianLevelDto_YouthService);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'MusicianLevelDto.youthService()';
}


}




/// @nodoc


class MusicianLevelDto_OfficialService extends MusicianLevelDto {
  const MusicianLevelDto_OfficialService(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is MusicianLevelDto_OfficialService);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'MusicianLevelDto.officialService()';
}


}




/// @nodoc


class MusicianLevelDto_Officialized extends MusicianLevelDto {
  const MusicianLevelDto_Officialized(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is MusicianLevelDto_Officialized);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'MusicianLevelDto.officialized()';
}


}




/// @nodoc


class MusicianLevelDto_Unknown extends MusicianLevelDto {
  const MusicianLevelDto_Unknown({required this.raw}): super._();
  

 final  String raw;

/// Create a copy of MusicianLevelDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$MusicianLevelDto_UnknownCopyWith<MusicianLevelDto_Unknown> get copyWith => _$MusicianLevelDto_UnknownCopyWithImpl<MusicianLevelDto_Unknown>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is MusicianLevelDto_Unknown&&(identical(other.raw, raw) || other.raw == raw));
}


@override
int get hashCode => Object.hash(runtimeType,raw);

@override
String toString() {
  return 'MusicianLevelDto.unknown(raw: $raw)';
}


}

/// @nodoc
abstract mixin class $MusicianLevelDto_UnknownCopyWith<$Res> implements $MusicianLevelDtoCopyWith<$Res> {
  factory $MusicianLevelDto_UnknownCopyWith(MusicianLevelDto_Unknown value, $Res Function(MusicianLevelDto_Unknown) _then) = _$MusicianLevelDto_UnknownCopyWithImpl;
@useResult
$Res call({
 String raw
});




}
/// @nodoc
class _$MusicianLevelDto_UnknownCopyWithImpl<$Res>
    implements $MusicianLevelDto_UnknownCopyWith<$Res> {
  _$MusicianLevelDto_UnknownCopyWithImpl(this._self, this._then);

  final MusicianLevelDto_Unknown _self;
  final $Res Function(MusicianLevelDto_Unknown) _then;

/// Create a copy of MusicianLevelDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? raw = null,}) {
  return _then(MusicianLevelDto_Unknown(
raw: null == raw ? _self.raw : raw // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

// dart format on
