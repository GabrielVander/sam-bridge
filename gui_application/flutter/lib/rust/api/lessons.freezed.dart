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
mixin _$RetrieveStudentLessonsOutcome {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is RetrieveStudentLessonsOutcome);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'RetrieveStudentLessonsOutcome()';
}


}

/// @nodoc
class $RetrieveStudentLessonsOutcomeCopyWith<$Res>  {
$RetrieveStudentLessonsOutcomeCopyWith(RetrieveStudentLessonsOutcome _, $Res Function(RetrieveStudentLessonsOutcome) __);
}


/// Adds pattern-matching-related methods to [RetrieveStudentLessonsOutcome].
extension RetrieveStudentLessonsOutcomePatterns on RetrieveStudentLessonsOutcome {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( RetrieveStudentLessonsOutcome_Success value)?  success,TResult Function( RetrieveStudentLessonsOutcome_Failure value)?  failure,required TResult orElse(),}){
final _that = this;
switch (_that) {
case RetrieveStudentLessonsOutcome_Success() when success != null:
return success(_that);case RetrieveStudentLessonsOutcome_Failure() when failure != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( RetrieveStudentLessonsOutcome_Success value)  success,required TResult Function( RetrieveStudentLessonsOutcome_Failure value)  failure,}){
final _that = this;
switch (_that) {
case RetrieveStudentLessonsOutcome_Success():
return success(_that);case RetrieveStudentLessonsOutcome_Failure():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( RetrieveStudentLessonsOutcome_Success value)?  success,TResult? Function( RetrieveStudentLessonsOutcome_Failure value)?  failure,}){
final _that = this;
switch (_that) {
case RetrieveStudentLessonsOutcome_Success() when success != null:
return success(_that);case RetrieveStudentLessonsOutcome_Failure() when failure != null:
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
case RetrieveStudentLessonsOutcome_Success() when success != null:
return success(_that.lessons);case RetrieveStudentLessonsOutcome_Failure() when failure != null:
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
case RetrieveStudentLessonsOutcome_Success():
return success(_that.lessons);case RetrieveStudentLessonsOutcome_Failure():
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
case RetrieveStudentLessonsOutcome_Success() when success != null:
return success(_that.lessons);case RetrieveStudentLessonsOutcome_Failure() when failure != null:
return failure(_that.report);case _:
  return null;

}
}

}

/// @nodoc


class RetrieveStudentLessonsOutcome_Success extends RetrieveStudentLessonsOutcome {
  const RetrieveStudentLessonsOutcome_Success({required this.lessons}): super._();
  

 final  StudentLessonsDto lessons;

/// Create a copy of RetrieveStudentLessonsOutcome
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$RetrieveStudentLessonsOutcome_SuccessCopyWith<RetrieveStudentLessonsOutcome_Success> get copyWith => _$RetrieveStudentLessonsOutcome_SuccessCopyWithImpl<RetrieveStudentLessonsOutcome_Success>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is RetrieveStudentLessonsOutcome_Success&&(identical(other.lessons, lessons) || other.lessons == lessons));
}


@override
int get hashCode => Object.hash(runtimeType,lessons);

@override
String toString() {
  return 'RetrieveStudentLessonsOutcome.success(lessons: $lessons)';
}


}

/// @nodoc
abstract mixin class $RetrieveStudentLessonsOutcome_SuccessCopyWith<$Res> implements $RetrieveStudentLessonsOutcomeCopyWith<$Res> {
  factory $RetrieveStudentLessonsOutcome_SuccessCopyWith(RetrieveStudentLessonsOutcome_Success value, $Res Function(RetrieveStudentLessonsOutcome_Success) _then) = _$RetrieveStudentLessonsOutcome_SuccessCopyWithImpl;
@useResult
$Res call({
 StudentLessonsDto lessons
});




}
/// @nodoc
class _$RetrieveStudentLessonsOutcome_SuccessCopyWithImpl<$Res>
    implements $RetrieveStudentLessonsOutcome_SuccessCopyWith<$Res> {
  _$RetrieveStudentLessonsOutcome_SuccessCopyWithImpl(this._self, this._then);

  final RetrieveStudentLessonsOutcome_Success _self;
  final $Res Function(RetrieveStudentLessonsOutcome_Success) _then;

/// Create a copy of RetrieveStudentLessonsOutcome
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? lessons = null,}) {
  return _then(RetrieveStudentLessonsOutcome_Success(
lessons: null == lessons ? _self.lessons : lessons // ignore: cast_nullable_to_non_nullable
as StudentLessonsDto,
  ));
}


}

/// @nodoc


class RetrieveStudentLessonsOutcome_Failure extends RetrieveStudentLessonsOutcome {
  const RetrieveStudentLessonsOutcome_Failure({required this.report}): super._();
  

 final  ErrorReportDto report;

/// Create a copy of RetrieveStudentLessonsOutcome
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$RetrieveStudentLessonsOutcome_FailureCopyWith<RetrieveStudentLessonsOutcome_Failure> get copyWith => _$RetrieveStudentLessonsOutcome_FailureCopyWithImpl<RetrieveStudentLessonsOutcome_Failure>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is RetrieveStudentLessonsOutcome_Failure&&(identical(other.report, report) || other.report == report));
}


@override
int get hashCode => Object.hash(runtimeType,report);

@override
String toString() {
  return 'RetrieveStudentLessonsOutcome.failure(report: $report)';
}


}

/// @nodoc
abstract mixin class $RetrieveStudentLessonsOutcome_FailureCopyWith<$Res> implements $RetrieveStudentLessonsOutcomeCopyWith<$Res> {
  factory $RetrieveStudentLessonsOutcome_FailureCopyWith(RetrieveStudentLessonsOutcome_Failure value, $Res Function(RetrieveStudentLessonsOutcome_Failure) _then) = _$RetrieveStudentLessonsOutcome_FailureCopyWithImpl;
@useResult
$Res call({
 ErrorReportDto report
});




}
/// @nodoc
class _$RetrieveStudentLessonsOutcome_FailureCopyWithImpl<$Res>
    implements $RetrieveStudentLessonsOutcome_FailureCopyWith<$Res> {
  _$RetrieveStudentLessonsOutcome_FailureCopyWithImpl(this._self, this._then);

  final RetrieveStudentLessonsOutcome_Failure _self;
  final $Res Function(RetrieveStudentLessonsOutcome_Failure) _then;

/// Create a copy of RetrieveStudentLessonsOutcome
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? report = null,}) {
  return _then(RetrieveStudentLessonsOutcome_Failure(
report: null == report ? _self.report : report // ignore: cast_nullable_to_non_nullable
as ErrorReportDto,
  ));
}


}

// dart format on
