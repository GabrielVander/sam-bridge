import 'package:flutter_application/roster/student_list_item.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('StudentListItem', () {
    StudentListItem item({String? instrument}) => StudentListItem(
      id: '1',
      name: 'Jane Doe',
      location: 'Some Location',
      position: 'Ensaio',
      instrument: instrument,
    );

    test('is equal when every field is equal, instrument included', () {
      expect(item(instrument: 'Violino'), item(instrument: 'Violino'));
      expect(
        item(instrument: 'Violino').hashCode,
        item(instrument: 'Violino').hashCode,
      );
      expect(item(), item());
    });

    test('differs when only the instrument differs', () {
      expect(item(instrument: 'Violino'), isNot(item(instrument: 'Viola')));
      expect(item(instrument: 'Violino'), isNot(item()));
    });
  });
}
