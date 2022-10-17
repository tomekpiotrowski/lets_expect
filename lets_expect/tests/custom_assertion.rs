mod point;
use lets_expect::{AssertionResult, AssertionError};

use crate::point::Point;

fn have_positive_coordinates(point: Point) -> AssertionResult {
    if point.x > 0 && point.y > 0 {
        Ok(())
    } else {
        Err(AssertionError::new(vec![format!("Expected ({}, {}) to be positive coordinates", point.x, point.y)]))
    }
}

fn have_x_coordinate_equal(x: i32) -> impl Fn(Point) -> AssertionResult {
    move |point| {
        if point.x == x {
            Ok(())
        } else {
            Err(AssertionError::new(vec![format!("Expected x coordinate to be {}, but it was {}", x, point.x)]))
        }
    }
}

mod expect {
    use lets_expect::*;
    use crate::point::Point;
    use super::*;

    lets_expect! {
        expect(Point { x: 2, y: 22 }) {
            to have_valid_coordinates {
                have_positive_coordinates,
                have_x_coordinate_equal(2)
            }
        }
    }
}