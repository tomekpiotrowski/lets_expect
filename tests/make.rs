mod point;

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use crate::point::Segment;
    use lets_expect::lets_expect;

    lets_expect! {
        expect(point.x = 5) {
            let mut point = Point { x: 1, y: 2 };
            let unrelated = 5;

            to have_valid_coordinates {
                make(point.x) equal(5),
                make(point.x) { not_equal(1) },
                make(point.y) { not_equal(1), equal(2) },
                make(unrelated) equal(5)
            }
        }

        expect(Segment { start: Point { x: 1, y: 2 }, end: Point { x: 3, y: 4 } }) {
            to pass_the_same_make_assertion_twice {
                make(subject.start.clone()) equal(Point { x: 1, y: 2 }),
                make(subject.start) equal(Point { x: 1, y: 2 })
            }
        }
    }
}
