// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'lessons.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$RetrieveStudentLessonsOutcomeDto {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is RetrieveStudentLessonsOutcomeDto);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'RetrieveStudentLessonsOutcomeDto()';
}


}

/// @nodoc
class $RetrieveStudentLessonsOutcomeDtoCopyWith<$Res>  {
$RetrieveStudentLessonsOutcomeDtoCopyWith(RetrieveStudentLessonsOutcomeDto _, $Res Function(RetrieveStudentLessonsOutcomeDto) __);
}


/// Adds pattern-matching-related methods to [RetrieveStudentLessonsOutcomeDto].
extension RetrieveStudentLessonsOutcomeDtoPatterns on RetrieveStudentLessonsOutcomeDto {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( RetrieveStudentLessonsOutcomeDto_Success value)?  success,TResult Function( RetrieveStudentLessonsOutcomeDto_Failure value)?  failure,required TResult orElse(),}){
final _that = this;
switch (_that) {
case RetrieveStudentLessonsOutcomeDto_Success() when success != null:
return success(_that);case RetrieveStudentLessonsOutcomeDto_Failure() when failure != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( RetrieveStudentLessonsOutcomeDto_Success value)  success,required TResult Function( RetrieveStudentLessonsOutcomeDto_Failure value)  failure,}){
final _that = this;
switch (_that) {
case RetrieveStudentLessonsOutcomeDto_Success():
return success(_that);case RetrieveStudentLessonsOutcomeDto_Failure():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( RetrieveStudentLessonsOutcomeDto_Success value)?  success,TResult? Function( RetrieveStudentLessonsOutcomeDto_Failure value)?  failure,}){
final _that = this;
switch (_that) {
case RetrieveStudentLessonsOutcomeDto_Success() when success != null:
return success(_that);case RetrieveStudentLessonsOutcomeDto_Failure() when failure != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( StudentLessonsDto lessons)?  success,TResult Function( ErrorReportDto report)?  failure,required TResult orElse(),}) {final _that = this;
switch (_that) {
case RetrieveStudentLessonsOutcomeDto_Success() when success != null:
return success(_that.lessons);case RetrieveStudentLessonsOutcomeDto_Failure() when failure != null:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( StudentLessonsDto lessons)  success,required TResult Function( ErrorReportDto report)  failure,}) {final _that = this;
switch (_that) {
case RetrieveStudentLessonsOutcomeDto_Success():
return success(_that.lessons);case RetrieveStudentLessonsOutcomeDto_Failure():
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( StudentLessonsDto lessons)?  success,TResult? Function( ErrorReportDto report)?  failure,}) {final _that = this;
switch (_that) {
case RetrieveStudentLessonsOutcomeDto_Success() when success != null:
return success(_that.lessons);case RetrieveStudentLessonsOutcomeDto_Failure() when failure != null:
return failure(_that.report);case _:
  return null;

}
}

}

/// @nodoc


class RetrieveStudentLessonsOutcomeDto_Success extends RetrieveStudentLessonsOutcomeDto {
  const RetrieveStudentLessonsOutcomeDto_Success({required this.lessons}): super._();
  

 final  StudentLessonsDto lessons;

/// Create a copy of RetrieveStudentLessonsOutcomeDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$RetrieveStudentLessonsOutcomeDto_SuccessCopyWith<RetrieveStudentLessonsOutcomeDto_Success> get copyWith => _$RetrieveStudentLessonsOutcomeDto_SuccessCopyWithImpl<RetrieveStudentLessonsOutcomeDto_Success>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is RetrieveStudentLessonsOutcomeDto_Success&&(identical(other.lessons, lessons) || other.lessons == lessons));
}


@override
int get hashCode => Object.hash(runtimeType,lessons);

@override
String toString() {
  return 'RetrieveStudentLessonsOutcomeDto.success(lessons: $lessons)';
}


}

/// @nodoc
abstract mixin class $RetrieveStudentLessonsOutcomeDto_SuccessCopyWith<$Res> implements $RetrieveStudentLessonsOutcomeDtoCopyWith<$Res> {
  factory $RetrieveStudentLessonsOutcomeDto_SuccessCopyWith(RetrieveStudentLessonsOutcomeDto_Success value, $Res Function(RetrieveStudentLessonsOutcomeDto_Success) _then) = _$RetrieveStudentLessonsOutcomeDto_SuccessCopyWithImpl;
@useResult
$Res call({
 StudentLessonsDto lessons
});




}
/// @nodoc
class _$RetrieveStudentLessonsOutcomeDto_SuccessCopyWithImpl<$Res>
    implements $RetrieveStudentLessonsOutcomeDto_SuccessCopyWith<$Res> {
  _$RetrieveStudentLessonsOutcomeDto_SuccessCopyWithImpl(this._self, this._then);

  final RetrieveStudentLessonsOutcomeDto_Success _self;
  final $Res Function(RetrieveStudentLessonsOutcomeDto_Success) _then;

/// Create a copy of RetrieveStudentLessonsOutcomeDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? lessons = null,}) {
  return _then(RetrieveStudentLessonsOutcomeDto_Success(
lessons: null == lessons ? _self.lessons : lessons // ignore: cast_nullable_to_non_nullable
as StudentLessonsDto,
  ));
}


}

/// @nodoc


class RetrieveStudentLessonsOutcomeDto_Failure extends RetrieveStudentLessonsOutcomeDto {
  const RetrieveStudentLessonsOutcomeDto_Failure({required this.report}): super._();
  

 final  ErrorReportDto report;

/// Create a copy of RetrieveStudentLessonsOutcomeDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$RetrieveStudentLessonsOutcomeDto_FailureCopyWith<RetrieveStudentLessonsOutcomeDto_Failure> get copyWith => _$RetrieveStudentLessonsOutcomeDto_FailureCopyWithImpl<RetrieveStudentLessonsOutcomeDto_Failure>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is RetrieveStudentLessonsOutcomeDto_Failure&&(identical(other.report, report) || other.report == report));
}


@override
int get hashCode => Object.hash(runtimeType,report);

@override
String toString() {
  return 'RetrieveStudentLessonsOutcomeDto.failure(report: $report)';
}


}

/// @nodoc
abstract mixin class $RetrieveStudentLessonsOutcomeDto_FailureCopyWith<$Res> implements $RetrieveStudentLessonsOutcomeDtoCopyWith<$Res> {
  factory $RetrieveStudentLessonsOutcomeDto_FailureCopyWith(RetrieveStudentLessonsOutcomeDto_Failure value, $Res Function(RetrieveStudentLessonsOutcomeDto_Failure) _then) = _$RetrieveStudentLessonsOutcomeDto_FailureCopyWithImpl;
@useResult
$Res call({
 ErrorReportDto report
});




}
/// @nodoc
class _$RetrieveStudentLessonsOutcomeDto_FailureCopyWithImpl<$Res>
    implements $RetrieveStudentLessonsOutcomeDto_FailureCopyWith<$Res> {
  _$RetrieveStudentLessonsOutcomeDto_FailureCopyWithImpl(this._self, this._then);

  final RetrieveStudentLessonsOutcomeDto_Failure _self;
  final $Res Function(RetrieveStudentLessonsOutcomeDto_Failure) _then;

/// Create a copy of RetrieveStudentLessonsOutcomeDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? report = null,}) {
  return _then(RetrieveStudentLessonsOutcomeDto_Failure(
report: null == report ? _self.report : report // ignore: cast_nullable_to_non_nullable
as ErrorReportDto,
  ));
}


}

// dart format on
