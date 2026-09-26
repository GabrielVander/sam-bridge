// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'roster.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$RetrieveAllAvailableStudentsOutcomeDto {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is RetrieveAllAvailableStudentsOutcomeDto);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'RetrieveAllAvailableStudentsOutcomeDto()';
}


}

/// @nodoc
class $RetrieveAllAvailableStudentsOutcomeDtoCopyWith<$Res>  {
$RetrieveAllAvailableStudentsOutcomeDtoCopyWith(RetrieveAllAvailableStudentsOutcomeDto _, $Res Function(RetrieveAllAvailableStudentsOutcomeDto) __);
}


/// Adds pattern-matching-related methods to [RetrieveAllAvailableStudentsOutcomeDto].
extension RetrieveAllAvailableStudentsOutcomeDtoPatterns on RetrieveAllAvailableStudentsOutcomeDto {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( RetrieveAllAvailableStudentsOutcomeDto_Success value)?  success,TResult Function( RetrieveAllAvailableStudentsOutcomeDto_Failure value)?  failure,required TResult orElse(),}){
final _that = this;
switch (_that) {
case RetrieveAllAvailableStudentsOutcomeDto_Success() when success != null:
return success(_that);case RetrieveAllAvailableStudentsOutcomeDto_Failure() when failure != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( RetrieveAllAvailableStudentsOutcomeDto_Success value)  success,required TResult Function( RetrieveAllAvailableStudentsOutcomeDto_Failure value)  failure,}){
final _that = this;
switch (_that) {
case RetrieveAllAvailableStudentsOutcomeDto_Success():
return success(_that);case RetrieveAllAvailableStudentsOutcomeDto_Failure():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( RetrieveAllAvailableStudentsOutcomeDto_Success value)?  success,TResult? Function( RetrieveAllAvailableStudentsOutcomeDto_Failure value)?  failure,}){
final _that = this;
switch (_that) {
case RetrieveAllAvailableStudentsOutcomeDto_Success() when success != null:
return success(_that);case RetrieveAllAvailableStudentsOutcomeDto_Failure() when failure != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( List<StudentSummaryDto> students)?  success,TResult Function( ErrorReportDto report)?  failure,required TResult orElse(),}) {final _that = this;
switch (_that) {
case RetrieveAllAvailableStudentsOutcomeDto_Success() when success != null:
return success(_that.students);case RetrieveAllAvailableStudentsOutcomeDto_Failure() when failure != null:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( List<StudentSummaryDto> students)  success,required TResult Function( ErrorReportDto report)  failure,}) {final _that = this;
switch (_that) {
case RetrieveAllAvailableStudentsOutcomeDto_Success():
return success(_that.students);case RetrieveAllAvailableStudentsOutcomeDto_Failure():
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( List<StudentSummaryDto> students)?  success,TResult? Function( ErrorReportDto report)?  failure,}) {final _that = this;
switch (_that) {
case RetrieveAllAvailableStudentsOutcomeDto_Success() when success != null:
return success(_that.students);case RetrieveAllAvailableStudentsOutcomeDto_Failure() when failure != null:
return failure(_that.report);case _:
  return null;

}
}

}

/// @nodoc


class RetrieveAllAvailableStudentsOutcomeDto_Success extends RetrieveAllAvailableStudentsOutcomeDto {
  const RetrieveAllAvailableStudentsOutcomeDto_Success({required final  List<StudentSummaryDto> students}): _students = students,super._();
  

 final  List<StudentSummaryDto> _students;
 List<StudentSummaryDto> get students {
  if (_students is EqualUnmodifiableListView) return _students;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_students);
}


/// Create a copy of RetrieveAllAvailableStudentsOutcomeDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$RetrieveAllAvailableStudentsOutcomeDto_SuccessCopyWith<RetrieveAllAvailableStudentsOutcomeDto_Success> get copyWith => _$RetrieveAllAvailableStudentsOutcomeDto_SuccessCopyWithImpl<RetrieveAllAvailableStudentsOutcomeDto_Success>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is RetrieveAllAvailableStudentsOutcomeDto_Success&&const DeepCollectionEquality().equals(other._students, _students));
}


@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(_students));

@override
String toString() {
  return 'RetrieveAllAvailableStudentsOutcomeDto.success(students: $students)';
}


}

/// @nodoc
abstract mixin class $RetrieveAllAvailableStudentsOutcomeDto_SuccessCopyWith<$Res> implements $RetrieveAllAvailableStudentsOutcomeDtoCopyWith<$Res> {
  factory $RetrieveAllAvailableStudentsOutcomeDto_SuccessCopyWith(RetrieveAllAvailableStudentsOutcomeDto_Success value, $Res Function(RetrieveAllAvailableStudentsOutcomeDto_Success) _then) = _$RetrieveAllAvailableStudentsOutcomeDto_SuccessCopyWithImpl;
@useResult
$Res call({
 List<StudentSummaryDto> students
});




}
/// @nodoc
class _$RetrieveAllAvailableStudentsOutcomeDto_SuccessCopyWithImpl<$Res>
    implements $RetrieveAllAvailableStudentsOutcomeDto_SuccessCopyWith<$Res> {
  _$RetrieveAllAvailableStudentsOutcomeDto_SuccessCopyWithImpl(this._self, this._then);

  final RetrieveAllAvailableStudentsOutcomeDto_Success _self;
  final $Res Function(RetrieveAllAvailableStudentsOutcomeDto_Success) _then;

/// Create a copy of RetrieveAllAvailableStudentsOutcomeDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? students = null,}) {
  return _then(RetrieveAllAvailableStudentsOutcomeDto_Success(
students: null == students ? _self._students : students // ignore: cast_nullable_to_non_nullable
as List<StudentSummaryDto>,
  ));
}


}

/// @nodoc


class RetrieveAllAvailableStudentsOutcomeDto_Failure extends RetrieveAllAvailableStudentsOutcomeDto {
  const RetrieveAllAvailableStudentsOutcomeDto_Failure({required this.report}): super._();
  

 final  ErrorReportDto report;

/// Create a copy of RetrieveAllAvailableStudentsOutcomeDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$RetrieveAllAvailableStudentsOutcomeDto_FailureCopyWith<RetrieveAllAvailableStudentsOutcomeDto_Failure> get copyWith => _$RetrieveAllAvailableStudentsOutcomeDto_FailureCopyWithImpl<RetrieveAllAvailableStudentsOutcomeDto_Failure>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is RetrieveAllAvailableStudentsOutcomeDto_Failure&&(identical(other.report, report) || other.report == report));
}


@override
int get hashCode => Object.hash(runtimeType,report);

@override
String toString() {
  return 'RetrieveAllAvailableStudentsOutcomeDto.failure(report: $report)';
}


}

/// @nodoc
abstract mixin class $RetrieveAllAvailableStudentsOutcomeDto_FailureCopyWith<$Res> implements $RetrieveAllAvailableStudentsOutcomeDtoCopyWith<$Res> {
  factory $RetrieveAllAvailableStudentsOutcomeDto_FailureCopyWith(RetrieveAllAvailableStudentsOutcomeDto_Failure value, $Res Function(RetrieveAllAvailableStudentsOutcomeDto_Failure) _then) = _$RetrieveAllAvailableStudentsOutcomeDto_FailureCopyWithImpl;
@useResult
$Res call({
 ErrorReportDto report
});




}
/// @nodoc
class _$RetrieveAllAvailableStudentsOutcomeDto_FailureCopyWithImpl<$Res>
    implements $RetrieveAllAvailableStudentsOutcomeDto_FailureCopyWith<$Res> {
  _$RetrieveAllAvailableStudentsOutcomeDto_FailureCopyWithImpl(this._self, this._then);

  final RetrieveAllAvailableStudentsOutcomeDto_Failure _self;
  final $Res Function(RetrieveAllAvailableStudentsOutcomeDto_Failure) _then;

/// Create a copy of RetrieveAllAvailableStudentsOutcomeDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? report = null,}) {
  return _then(RetrieveAllAvailableStudentsOutcomeDto_Failure(
report: null == report ? _self.report : report // ignore: cast_nullable_to_non_nullable
as ErrorReportDto,
  ));
}


}

/// @nodoc
mixin _$StudentPositionDto {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is StudentPositionDto);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'StudentPositionDto()';
}


}

/// @nodoc
class $StudentPositionDtoCopyWith<$Res>  {
$StudentPositionDtoCopyWith(StudentPositionDto _, $Res Function(StudentPositionDto) __);
}


/// Adds pattern-matching-related methods to [StudentPositionDto].
extension StudentPositionDtoPatterns on StudentPositionDto {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( StudentPositionDto_Candidate value)?  candidate,TResult Function( StudentPositionDto_Practice value)?  practice,TResult Function( StudentPositionDto_YouthService value)?  youthService,TResult Function( StudentPositionDto_OfficialService value)?  officialService,TResult Function( StudentPositionDto_Officialized value)?  officialized,TResult Function( StudentPositionDto_HalfHour value)?  halfHour,TResult Function( StudentPositionDto_YouthServiceHalfHour value)?  youthServiceHalfHour,TResult Function( StudentPositionDto_YouthServicePractice value)?  youthServicePractice,TResult Function( StudentPositionDto_YouthServiceOfficialService value)?  youthServiceOfficialService,TResult Function( StudentPositionDto_YouthServiceOfficialized value)?  youthServiceOfficialized,TResult Function( StudentPositionDto_GemSecretary value)?  gemSecretary,TResult Function( StudentPositionDto_MusicSecretary value)?  musicSecretary,TResult Function( StudentPositionDto_Invalid value)?  invalid,required TResult orElse(),}){
final _that = this;
switch (_that) {
case StudentPositionDto_Candidate() when candidate != null:
return candidate(_that);case StudentPositionDto_Practice() when practice != null:
return practice(_that);case StudentPositionDto_YouthService() when youthService != null:
return youthService(_that);case StudentPositionDto_OfficialService() when officialService != null:
return officialService(_that);case StudentPositionDto_Officialized() when officialized != null:
return officialized(_that);case StudentPositionDto_HalfHour() when halfHour != null:
return halfHour(_that);case StudentPositionDto_YouthServiceHalfHour() when youthServiceHalfHour != null:
return youthServiceHalfHour(_that);case StudentPositionDto_YouthServicePractice() when youthServicePractice != null:
return youthServicePractice(_that);case StudentPositionDto_YouthServiceOfficialService() when youthServiceOfficialService != null:
return youthServiceOfficialService(_that);case StudentPositionDto_YouthServiceOfficialized() when youthServiceOfficialized != null:
return youthServiceOfficialized(_that);case StudentPositionDto_GemSecretary() when gemSecretary != null:
return gemSecretary(_that);case StudentPositionDto_MusicSecretary() when musicSecretary != null:
return musicSecretary(_that);case StudentPositionDto_Invalid() when invalid != null:
return invalid(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( StudentPositionDto_Candidate value)  candidate,required TResult Function( StudentPositionDto_Practice value)  practice,required TResult Function( StudentPositionDto_YouthService value)  youthService,required TResult Function( StudentPositionDto_OfficialService value)  officialService,required TResult Function( StudentPositionDto_Officialized value)  officialized,required TResult Function( StudentPositionDto_HalfHour value)  halfHour,required TResult Function( StudentPositionDto_YouthServiceHalfHour value)  youthServiceHalfHour,required TResult Function( StudentPositionDto_YouthServicePractice value)  youthServicePractice,required TResult Function( StudentPositionDto_YouthServiceOfficialService value)  youthServiceOfficialService,required TResult Function( StudentPositionDto_YouthServiceOfficialized value)  youthServiceOfficialized,required TResult Function( StudentPositionDto_GemSecretary value)  gemSecretary,required TResult Function( StudentPositionDto_MusicSecretary value)  musicSecretary,required TResult Function( StudentPositionDto_Invalid value)  invalid,}){
final _that = this;
switch (_that) {
case StudentPositionDto_Candidate():
return candidate(_that);case StudentPositionDto_Practice():
return practice(_that);case StudentPositionDto_YouthService():
return youthService(_that);case StudentPositionDto_OfficialService():
return officialService(_that);case StudentPositionDto_Officialized():
return officialized(_that);case StudentPositionDto_HalfHour():
return halfHour(_that);case StudentPositionDto_YouthServiceHalfHour():
return youthServiceHalfHour(_that);case StudentPositionDto_YouthServicePractice():
return youthServicePractice(_that);case StudentPositionDto_YouthServiceOfficialService():
return youthServiceOfficialService(_that);case StudentPositionDto_YouthServiceOfficialized():
return youthServiceOfficialized(_that);case StudentPositionDto_GemSecretary():
return gemSecretary(_that);case StudentPositionDto_MusicSecretary():
return musicSecretary(_that);case StudentPositionDto_Invalid():
return invalid(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( StudentPositionDto_Candidate value)?  candidate,TResult? Function( StudentPositionDto_Practice value)?  practice,TResult? Function( StudentPositionDto_YouthService value)?  youthService,TResult? Function( StudentPositionDto_OfficialService value)?  officialService,TResult? Function( StudentPositionDto_Officialized value)?  officialized,TResult? Function( StudentPositionDto_HalfHour value)?  halfHour,TResult? Function( StudentPositionDto_YouthServiceHalfHour value)?  youthServiceHalfHour,TResult? Function( StudentPositionDto_YouthServicePractice value)?  youthServicePractice,TResult? Function( StudentPositionDto_YouthServiceOfficialService value)?  youthServiceOfficialService,TResult? Function( StudentPositionDto_YouthServiceOfficialized value)?  youthServiceOfficialized,TResult? Function( StudentPositionDto_GemSecretary value)?  gemSecretary,TResult? Function( StudentPositionDto_MusicSecretary value)?  musicSecretary,TResult? Function( StudentPositionDto_Invalid value)?  invalid,}){
final _that = this;
switch (_that) {
case StudentPositionDto_Candidate() when candidate != null:
return candidate(_that);case StudentPositionDto_Practice() when practice != null:
return practice(_that);case StudentPositionDto_YouthService() when youthService != null:
return youthService(_that);case StudentPositionDto_OfficialService() when officialService != null:
return officialService(_that);case StudentPositionDto_Officialized() when officialized != null:
return officialized(_that);case StudentPositionDto_HalfHour() when halfHour != null:
return halfHour(_that);case StudentPositionDto_YouthServiceHalfHour() when youthServiceHalfHour != null:
return youthServiceHalfHour(_that);case StudentPositionDto_YouthServicePractice() when youthServicePractice != null:
return youthServicePractice(_that);case StudentPositionDto_YouthServiceOfficialService() when youthServiceOfficialService != null:
return youthServiceOfficialService(_that);case StudentPositionDto_YouthServiceOfficialized() when youthServiceOfficialized != null:
return youthServiceOfficialized(_that);case StudentPositionDto_GemSecretary() when gemSecretary != null:
return gemSecretary(_that);case StudentPositionDto_MusicSecretary() when musicSecretary != null:
return musicSecretary(_that);case StudentPositionDto_Invalid() when invalid != null:
return invalid(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function()?  candidate,TResult Function()?  practice,TResult Function()?  youthService,TResult Function()?  officialService,TResult Function()?  officialized,TResult Function()?  halfHour,TResult Function()?  youthServiceHalfHour,TResult Function()?  youthServicePractice,TResult Function()?  youthServiceOfficialService,TResult Function()?  youthServiceOfficialized,TResult Function()?  gemSecretary,TResult Function()?  musicSecretary,TResult Function( String raw)?  invalid,required TResult orElse(),}) {final _that = this;
switch (_that) {
case StudentPositionDto_Candidate() when candidate != null:
return candidate();case StudentPositionDto_Practice() when practice != null:
return practice();case StudentPositionDto_YouthService() when youthService != null:
return youthService();case StudentPositionDto_OfficialService() when officialService != null:
return officialService();case StudentPositionDto_Officialized() when officialized != null:
return officialized();case StudentPositionDto_HalfHour() when halfHour != null:
return halfHour();case StudentPositionDto_YouthServiceHalfHour() when youthServiceHalfHour != null:
return youthServiceHalfHour();case StudentPositionDto_YouthServicePractice() when youthServicePractice != null:
return youthServicePractice();case StudentPositionDto_YouthServiceOfficialService() when youthServiceOfficialService != null:
return youthServiceOfficialService();case StudentPositionDto_YouthServiceOfficialized() when youthServiceOfficialized != null:
return youthServiceOfficialized();case StudentPositionDto_GemSecretary() when gemSecretary != null:
return gemSecretary();case StudentPositionDto_MusicSecretary() when musicSecretary != null:
return musicSecretary();case StudentPositionDto_Invalid() when invalid != null:
return invalid(_that.raw);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function()  candidate,required TResult Function()  practice,required TResult Function()  youthService,required TResult Function()  officialService,required TResult Function()  officialized,required TResult Function()  halfHour,required TResult Function()  youthServiceHalfHour,required TResult Function()  youthServicePractice,required TResult Function()  youthServiceOfficialService,required TResult Function()  youthServiceOfficialized,required TResult Function()  gemSecretary,required TResult Function()  musicSecretary,required TResult Function( String raw)  invalid,}) {final _that = this;
switch (_that) {
case StudentPositionDto_Candidate():
return candidate();case StudentPositionDto_Practice():
return practice();case StudentPositionDto_YouthService():
return youthService();case StudentPositionDto_OfficialService():
return officialService();case StudentPositionDto_Officialized():
return officialized();case StudentPositionDto_HalfHour():
return halfHour();case StudentPositionDto_YouthServiceHalfHour():
return youthServiceHalfHour();case StudentPositionDto_YouthServicePractice():
return youthServicePractice();case StudentPositionDto_YouthServiceOfficialService():
return youthServiceOfficialService();case StudentPositionDto_YouthServiceOfficialized():
return youthServiceOfficialized();case StudentPositionDto_GemSecretary():
return gemSecretary();case StudentPositionDto_MusicSecretary():
return musicSecretary();case StudentPositionDto_Invalid():
return invalid(_that.raw);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function()?  candidate,TResult? Function()?  practice,TResult? Function()?  youthService,TResult? Function()?  officialService,TResult? Function()?  officialized,TResult? Function()?  halfHour,TResult? Function()?  youthServiceHalfHour,TResult? Function()?  youthServicePractice,TResult? Function()?  youthServiceOfficialService,TResult? Function()?  youthServiceOfficialized,TResult? Function()?  gemSecretary,TResult? Function()?  musicSecretary,TResult? Function( String raw)?  invalid,}) {final _that = this;
switch (_that) {
case StudentPositionDto_Candidate() when candidate != null:
return candidate();case StudentPositionDto_Practice() when practice != null:
return practice();case StudentPositionDto_YouthService() when youthService != null:
return youthService();case StudentPositionDto_OfficialService() when officialService != null:
return officialService();case StudentPositionDto_Officialized() when officialized != null:
return officialized();case StudentPositionDto_HalfHour() when halfHour != null:
return halfHour();case StudentPositionDto_YouthServiceHalfHour() when youthServiceHalfHour != null:
return youthServiceHalfHour();case StudentPositionDto_YouthServicePractice() when youthServicePractice != null:
return youthServicePractice();case StudentPositionDto_YouthServiceOfficialService() when youthServiceOfficialService != null:
return youthServiceOfficialService();case StudentPositionDto_YouthServiceOfficialized() when youthServiceOfficialized != null:
return youthServiceOfficialized();case StudentPositionDto_GemSecretary() when gemSecretary != null:
return gemSecretary();case StudentPositionDto_MusicSecretary() when musicSecretary != null:
return musicSecretary();case StudentPositionDto_Invalid() when invalid != null:
return invalid(_that.raw);case _:
  return null;

}
}

}

/// @nodoc


class StudentPositionDto_Candidate extends StudentPositionDto {
  const StudentPositionDto_Candidate(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is StudentPositionDto_Candidate);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'StudentPositionDto.candidate()';
}


}




/// @nodoc


class StudentPositionDto_Practice extends StudentPositionDto {
  const StudentPositionDto_Practice(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is StudentPositionDto_Practice);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'StudentPositionDto.practice()';
}


}




/// @nodoc


class StudentPositionDto_YouthService extends StudentPositionDto {
  const StudentPositionDto_YouthService(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is StudentPositionDto_YouthService);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'StudentPositionDto.youthService()';
}


}




/// @nodoc


class StudentPositionDto_OfficialService extends StudentPositionDto {
  const StudentPositionDto_OfficialService(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is StudentPositionDto_OfficialService);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'StudentPositionDto.officialService()';
}


}




/// @nodoc


class StudentPositionDto_Officialized extends StudentPositionDto {
  const StudentPositionDto_Officialized(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is StudentPositionDto_Officialized);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'StudentPositionDto.officialized()';
}


}




/// @nodoc


class StudentPositionDto_HalfHour extends StudentPositionDto {
  const StudentPositionDto_HalfHour(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is StudentPositionDto_HalfHour);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'StudentPositionDto.halfHour()';
}


}




/// @nodoc


class StudentPositionDto_YouthServiceHalfHour extends StudentPositionDto {
  const StudentPositionDto_YouthServiceHalfHour(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is StudentPositionDto_YouthServiceHalfHour);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'StudentPositionDto.youthServiceHalfHour()';
}


}




/// @nodoc


class StudentPositionDto_YouthServicePractice extends StudentPositionDto {
  const StudentPositionDto_YouthServicePractice(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is StudentPositionDto_YouthServicePractice);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'StudentPositionDto.youthServicePractice()';
}


}




/// @nodoc


class StudentPositionDto_YouthServiceOfficialService extends StudentPositionDto {
  const StudentPositionDto_YouthServiceOfficialService(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is StudentPositionDto_YouthServiceOfficialService);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'StudentPositionDto.youthServiceOfficialService()';
}


}




/// @nodoc


class StudentPositionDto_YouthServiceOfficialized extends StudentPositionDto {
  const StudentPositionDto_YouthServiceOfficialized(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is StudentPositionDto_YouthServiceOfficialized);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'StudentPositionDto.youthServiceOfficialized()';
}


}




/// @nodoc


class StudentPositionDto_GemSecretary extends StudentPositionDto {
  const StudentPositionDto_GemSecretary(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is StudentPositionDto_GemSecretary);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'StudentPositionDto.gemSecretary()';
}


}




/// @nodoc


class StudentPositionDto_MusicSecretary extends StudentPositionDto {
  const StudentPositionDto_MusicSecretary(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is StudentPositionDto_MusicSecretary);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'StudentPositionDto.musicSecretary()';
}


}




/// @nodoc


class StudentPositionDto_Invalid extends StudentPositionDto {
  const StudentPositionDto_Invalid({required this.raw}): super._();
  

 final  String raw;

/// Create a copy of StudentPositionDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$StudentPositionDto_InvalidCopyWith<StudentPositionDto_Invalid> get copyWith => _$StudentPositionDto_InvalidCopyWithImpl<StudentPositionDto_Invalid>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is StudentPositionDto_Invalid&&(identical(other.raw, raw) || other.raw == raw));
}


@override
int get hashCode => Object.hash(runtimeType,raw);

@override
String toString() {
  return 'StudentPositionDto.invalid(raw: $raw)';
}


}

/// @nodoc
abstract mixin class $StudentPositionDto_InvalidCopyWith<$Res> implements $StudentPositionDtoCopyWith<$Res> {
  factory $StudentPositionDto_InvalidCopyWith(StudentPositionDto_Invalid value, $Res Function(StudentPositionDto_Invalid) _then) = _$StudentPositionDto_InvalidCopyWithImpl;
@useResult
$Res call({
 String raw
});




}
/// @nodoc
class _$StudentPositionDto_InvalidCopyWithImpl<$Res>
    implements $StudentPositionDto_InvalidCopyWith<$Res> {
  _$StudentPositionDto_InvalidCopyWithImpl(this._self, this._then);

  final StudentPositionDto_Invalid _self;
  final $Res Function(StudentPositionDto_Invalid) _then;

/// Create a copy of StudentPositionDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? raw = null,}) {
  return _then(StudentPositionDto_Invalid(
raw: null == raw ? _self.raw : raw // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

// dart format on
