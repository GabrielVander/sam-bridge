import 'package:flutter_application/lessons/widgets/category_lessons_view.dart';
import 'package:flutter_test/flutter_test.dart';

import '../../support/builders.dart';

void main() {
  group('CategoryLessonsView', () {
    testWidgets('explains that there is nothing to show when empty', (
      tester,
    ) async {
      await pumpInApp(
        tester,
        const CategoryLessonsView(
          lessons: [],
          emptyMessage: 'Nenhuma lição de MSA registrada.',
        ),
      );

      expect(find.text('Nenhuma lição de MSA registrada.'), findsOneWidget);
    });

    testWidgets('lists every lesson in the order given', (tester) async {
      await pumpInApp(
        tester,
        CategoryLessonsView(
          lessons: [
            lessonItem(id: '1', date: '01/02/2024'),
            lessonItem(id: '2', date: '08/02/2024'),
          ],
          emptyMessage: 'vazio',
        ),
      );

      expect(find.text('vazio'), findsNothing);
      expect(find.text('01/02/2024'), findsOneWidget);
      expect(find.text('08/02/2024'), findsOneWidget);
      expect(
        tester.getTopLeft(find.text('01/02/2024')).dy,
        lessThan(tester.getTopLeft(find.text('08/02/2024')).dy),
      );
    });
  });
}
