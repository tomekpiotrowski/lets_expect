mod point;

mod expect {
    use lets_expect::*;
    use crate::point::Point;

    lets_expect! {
        expect(point.x = 5) {
            let mut point = Point { x: 1, y: 2 };

            to have_valid_coordinates {
                make(point.x) equal(5),
                make(point.x) { not_equal(1) },
                make(point.y) { not_equal(1), equal(2) }
            }
        }

        expect(Point { x: 1, y: 2 }) {
            to make(subject_result.x + subject_result.y) equal(3)
        }
    }
}