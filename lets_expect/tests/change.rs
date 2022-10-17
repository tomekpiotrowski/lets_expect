mod point;

mod expect {
    use lets_expect::*;
    use crate::point::Point;

    lets_expect! {
        expect(point.x = 5) {
            let mut point = Point { x: 1, y: 2 };

            to change_only_x {
                change(point.x) { from(1), to(5), by(4) },
                not_change(point.y)
            }
        }
    }
}