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
mixin _$AssessStudentProgressOutcomeDto {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AssessStudentProgressOutcomeDto);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AssessStudentProgressOutcomeDto()';
}


}

/// @nodoc
class $AssessStudentProgressOutcomeDtoCopyWith<$Res>  {
$AssessStudentProgressOutcomeDtoCopyWith(AssessStudentProgressOutcomeDto _, $Res Function(AssessStudentProgressOutcomeDto) __);
}


/// Adds pattern-matching-related methods to [AssessStudentProgressOutcomeDto].
extension AssessStudentProgressOutcomeDtoPatterns on AssessStudentProgressOutcomeDto {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( AssessStudentProgressOutcomeDto_Success value)?  success,TResult Function( AssessStudentProgressOutcomeDto_NoInstrumentAssigned value)?  noInstrumentAssigned,TResult Function( AssessStudentProgressOutcomeDto_UnknownLevel value)?  unknownLevel,TResult Function( AssessStudentProgressOutcomeDto_NotAMusician value)?  notAMusician,TResult Function( AssessStudentProgressOutcomeDto_Failure value)?  failure,required TResult orElse(),}){
final _that = this;
switch (_that) {
case AssessStudentProgressOutcomeDto_Success() when success != null:
return success(_that);case AssessStudentProgressOutcomeDto_NoInstrumentAssigned() when noInstrumentAssigned != null:
return noInstrumentAssigned(_that);case AssessStudentProgressOutcomeDto_UnknownLevel() when unknownLevel != null:
return unknownLevel(_that);case AssessStudentProgressOutcomeDto_NotAMusician() when notAMusician != null:
return notAMusician(_that);case AssessStudentProgressOutcomeDto_Failure() when failure != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( AssessStudentProgressOutcomeDto_Success value)  success,required TResult Function( AssessStudentProgressOutcomeDto_NoInstrumentAssigned value)  noInstrumentAssigned,required TResult Function( AssessStudentProgressOutcomeDto_UnknownLevel value)  unknownLevel,required TResult Function( AssessStudentProgressOutcomeDto_NotAMusician value)  notAMusician,required TResult Function( AssessStudentProgressOutcomeDto_Failure value)  failure,}){
final _that = this;
switch (_that) {
case AssessStudentProgressOutcomeDto_Success():
return success(_that);case AssessStudentProgressOutcomeDto_NoInstrumentAssigned():
return noInstrumentAssigned(_that);case AssessStudentProgressOutcomeDto_UnknownLevel():
return unknownLevel(_that);case AssessStudentProgressOutcomeDto_NotAMusician():
return notAMusician(_that);case AssessStudentProgressOutcomeDto_Failure():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( AssessStudentProgressOutcomeDto_Success value)?  success,TResult? Function( AssessStudentProgressOutcomeDto_NoInstrumentAssigned value)?  noInstrumentAssigned,TResult? Function( AssessStudentProgressOutcomeDto_UnknownLevel value)?  unknownLevel,TResult? Function( AssessStudentProgressOutcomeDto_NotAMusician value)?  notAMusician,TResult? Function( AssessStudentProgressOutcomeDto_Failure value)?  failure,}){
final _that = this;
switch (_that) {
case AssessStudentProgressOutcomeDto_Success() when success != null:
return success(_that);case AssessStudentProgressOutcomeDto_NoInstrumentAssigned() when noInstrumentAssigned != null:
return noInstrumentAssigned(_that);case AssessStudentProgressOutcomeDto_UnknownLevel() when unknownLevel != null:
return unknownLevel(_that);case AssessStudentProgressOutcomeDto_NotAMusician() when notAMusician != null:
return notAMusician(_that);case AssessStudentProgressOutcomeDto_Failure() when failure != null:
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
case AssessStudentProgressOutcomeDto_Success() when success != null:
return success(_that.assessment);case AssessStudentProgressOutcomeDto_NoInstrumentAssigned() when noInstrumentAssigned != null:
return noInstrumentAssigned();case AssessStudentProgressOutcomeDto_UnknownLevel() when unknownLevel != null:
return unknownLevel(_that.rawLevel);case AssessStudentProgressOutcomeDto_NotAMusician() when notAMusician != null:
return notAMusician();case AssessStudentProgressOutcomeDto_Failure() when failure != null:
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
case AssessStudentProgressOutcomeDto_Success():
return success(_that.assessment);case AssessStudentProgressOutcomeDto_NoInstrumentAssigned():
return noInstrumentAssigned();case AssessStudentProgressOutcomeDto_UnknownLevel():
return unknownLevel(_that.rawLevel);case AssessStudentProgressOutcomeDto_NotAMusician():
return notAMusician();case AssessStudentProgressOutcomeDto_Failure():
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
case AssessStudentProgressOutcomeDto_Success() when success != null:
return success(_that.assessment);case AssessStudentProgressOutcomeDto_NoInstrumentAssigned() when noInstrumentAssigned != null:
return noInstrumentAssigned();case AssessStudentProgressOutcomeDto_UnknownLevel() when unknownLevel != null:
return unknownLevel(_that.rawLevel);case AssessStudentProgressOutcomeDto_NotAMusician() when notAMusician != null:
return notAMusician();case AssessStudentProgressOutcomeDto_Failure() when failure != null:
return failure(_that.report);case _:
  return null;

}
}

}

/// @nodoc


class AssessStudentProgressOutcomeDto_Success extends AssessStudentProgressOutcomeDto {
  const AssessStudentProgressOutcomeDto_Success({required this.assessment}): super._();
  

 final  ProgressAssessmentDto assessment;

/// Create a copy of AssessStudentProgressOutcomeDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AssessStudentProgressOutcomeDto_SuccessCopyWith<AssessStudentProgressOutcomeDto_Success> get copyWith => _$AssessStudentProgressOutcomeDto_SuccessCopyWithImpl<AssessStudentProgressOutcomeDto_Success>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AssessStudentProgressOutcomeDto_Success&&(identical(other.assessment, assessment) || other.assessment == assessment));
}


@override
int get hashCode => Object.hash(runtimeType,assessment);

@override
String toString() {
  return 'AssessStudentProgressOutcomeDto.success(assessment: $assessment)';
}


}

/// @nodoc
abstract mixin class $AssessStudentProgressOutcomeDto_SuccessCopyWith<$Res> implements $AssessStudentProgressOutcomeDtoCopyWith<$Res> {
  factory $AssessStudentProgressOutcomeDto_SuccessCopyWith(AssessStudentProgressOutcomeDto_Success value, $Res Function(AssessStudentProgressOutcomeDto_Success) _then) = _$AssessStudentProgressOutcomeDto_SuccessCopyWithImpl;
@useResult
$Res call({
 ProgressAssessmentDto assessment
});




}
/// @nodoc
class _$AssessStudentProgressOutcomeDto_SuccessCopyWithImpl<$Res>
    implements $AssessStudentProgressOutcomeDto_SuccessCopyWith<$Res> {
  _$AssessStudentProgressOutcomeDto_SuccessCopyWithImpl(this._self, this._then);

  final AssessStudentProgressOutcomeDto_Success _self;
  final $Res Function(AssessStudentProgressOutcomeDto_Success) _then;

/// Create a copy of AssessStudentProgressOutcomeDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? assessment = null,}) {
  return _then(AssessStudentProgressOutcomeDto_Success(
assessment: null == assessment ? _self.assessment : assessment // ignore: cast_nullable_to_non_nullable
as ProgressAssessmentDto,
  ));
}


}

/// @nodoc


class AssessStudentProgressOutcomeDto_NoInstrumentAssigned extends AssessStudentProgressOutcomeDto {
  const AssessStudentProgressOutcomeDto_NoInstrumentAssigned(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AssessStudentProgressOutcomeDto_NoInstrumentAssigned);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AssessStudentProgressOutcomeDto.noInstrumentAssigned()';
}


}




/// @nodoc


class AssessStudentProgressOutcomeDto_UnknownLevel extends AssessStudentProgressOutcomeDto {
  const AssessStudentProgressOutcomeDto_UnknownLevel({required this.rawLevel}): super._();
  

 final  String rawLevel;

/// Create a copy of AssessStudentProgressOutcomeDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AssessStudentProgressOutcomeDto_UnknownLevelCopyWith<AssessStudentProgressOutcomeDto_UnknownLevel> get copyWith => _$AssessStudentProgressOutcomeDto_UnknownLevelCopyWithImpl<AssessStudentProgressOutcomeDto_UnknownLevel>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AssessStudentProgressOutcomeDto_UnknownLevel&&(identical(other.rawLevel, rawLevel) || other.rawLevel == rawLevel));
}


@override
int get hashCode => Object.hash(runtimeType,rawLevel);

@override
String toString() {
  return 'AssessStudentProgressOutcomeDto.unknownLevel(rawLevel: $rawLevel)';
}


}

/// @nodoc
abstract mixin class $AssessStudentProgressOutcomeDto_UnknownLevelCopyWith<$Res> implements $AssessStudentProgressOutcomeDtoCopyWith<$Res> {
  factory $AssessStudentProgressOutcomeDto_UnknownLevelCopyWith(AssessStudentProgressOutcomeDto_UnknownLevel value, $Res Function(AssessStudentProgressOutcomeDto_UnknownLevel) _then) = _$AssessStudentProgressOutcomeDto_UnknownLevelCopyWithImpl;
@useResult
$Res call({
 String rawLevel
});




}
/// @nodoc
class _$AssessStudentProgressOutcomeDto_UnknownLevelCopyWithImpl<$Res>
    implements $AssessStudentProgressOutcomeDto_UnknownLevelCopyWith<$Res> {
  _$AssessStudentProgressOutcomeDto_UnknownLevelCopyWithImpl(this._self, this._then);

  final AssessStudentProgressOutcomeDto_UnknownLevel _self;
  final $Res Function(AssessStudentProgressOutcomeDto_UnknownLevel) _then;

/// Create a copy of AssessStudentProgressOutcomeDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? rawLevel = null,}) {
  return _then(AssessStudentProgressOutcomeDto_UnknownLevel(
rawLevel: null == rawLevel ? _self.rawLevel : rawLevel // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class AssessStudentProgressOutcomeDto_NotAMusician extends AssessStudentProgressOutcomeDto {
  const AssessStudentProgressOutcomeDto_NotAMusician(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AssessStudentProgressOutcomeDto_NotAMusician);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AssessStudentProgressOutcomeDto.notAMusician()';
}


}




/// @nodoc


class AssessStudentProgressOutcomeDto_Failure extends AssessStudentProgressOutcomeDto {
  const AssessStudentProgressOutcomeDto_Failure({required this.report}): super._();
  

 final  ErrorReportDto report;

/// Create a copy of AssessStudentProgressOutcomeDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AssessStudentProgressOutcomeDto_FailureCopyWith<AssessStudentProgressOutcomeDto_Failure> get copyWith => _$AssessStudentProgressOutcomeDto_FailureCopyWithImpl<AssessStudentProgressOutcomeDto_Failure>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AssessStudentProgressOutcomeDto_Failure&&(identical(other.report, report) || other.report == report));
}


@override
int get hashCode => Object.hash(runtimeType,report);

@override
String toString() {
  return 'AssessStudentProgressOutcomeDto.failure(report: $report)';
}


}

/// @nodoc
abstract mixin class $AssessStudentProgressOutcomeDto_FailureCopyWith<$Res> implements $AssessStudentProgressOutcomeDtoCopyWith<$Res> {
  factory $AssessStudentProgressOutcomeDto_FailureCopyWith(AssessStudentProgressOutcomeDto_Failure value, $Res Function(AssessStudentProgressOutcomeDto_Failure) _then) = _$AssessStudentProgressOutcomeDto_FailureCopyWithImpl;
@useResult
$Res call({
 ErrorReportDto report
});




}
/// @nodoc
class _$AssessStudentProgressOutcomeDto_FailureCopyWithImpl<$Res>
    implements $AssessStudentProgressOutcomeDto_FailureCopyWith<$Res> {
  _$AssessStudentProgressOutcomeDto_FailureCopyWithImpl(this._self, this._then);

  final AssessStudentProgressOutcomeDto_Failure _self;
  final $Res Function(AssessStudentProgressOutcomeDto_Failure) _then;

/// Create a copy of AssessStudentProgressOutcomeDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? report = null,}) {
  return _then(AssessStudentProgressOutcomeDto_Failure(
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
