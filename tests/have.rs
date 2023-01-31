mod point;

struct StructWithNestedFields {
    field: i32,
    nested: NestedStruct,
}

struct NestedStruct {
    field: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::Point;
    use crate::point::Segment;
    use lets_expect::lets_expect;

    lets_expect! {
        expect(segment) {
            let segment = Segment { start: Point { x: 0, y: 0 }, end: Point { x: 1, y: 1 } };

            to have_valid_coordinates {
                have(start) equal(Point { x: 0, y: 0 }),
                have(end) equal(Point { x: 1, y: 1 })
            }

            to access_the_same_value_twice {
                have(start.clone()) equal(Point { x: 0, y: 0 }),
                have(start) equal(Point { x: 0, y: 0 })
            }
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

        expect(struct_with_nested_fields) {
            let struct_with_nested_fields = StructWithNestedFields {
                field: 1,
                nested: NestedStruct { field: 2 }
            };

            to have_valid_fields {
                have(field) equal(1),
                have(nested) {
                    have(field) equal(2)
                }
            }

            to have(nested.field) equal(2)
        }
    }
}
