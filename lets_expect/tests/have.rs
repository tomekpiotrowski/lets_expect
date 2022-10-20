mod point;

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use lets_expect::*;

    lets_expect! {
        expect(a) {
            let a = Point { x: 1, y: 2 };

            to have(x) equal(1)
        }

        expect(a + b) {
            let a = Point { x: 1, y: 2 };
            let b = Point { x: 3, y: 4 };

            to have_valid_coordinates {
                have(x) equal(4),
                have(y) { equal(6), not_equal(5) }
            }

            when(valid_sum = "(4, 6)".to_string()) {
                to have(to_string()) equal(valid_sum)
            }
        }
    }
}
