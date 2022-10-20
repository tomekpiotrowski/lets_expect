mod point;

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use lets_expect::*;

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
