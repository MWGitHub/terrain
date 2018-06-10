/// Generic Array 2D structure which does not copy data.
pub mod array_2d {
    /// Converts coordinates to an index.
    /// ```
    /// let result = coords_to_index(1, 2, 2)
    /// assert_eq!(result, 5)
    /// ```
    pub fn coords_to_index(x: usize, y: usize, width: usize) -> usize {
        x + y * width
    }

    pub fn index_to_coords(index: usize, width: usize) -> (usize, usize) {
        (index % width, index / width)
    }

    pub trait Array2DTest<'a, T> {
        fn new(array: &'a [T], width: usize) -> Self;
        fn value_at_index(&self, index: usize) -> &T;
        fn value_at_coords(&self, x: usize, y: usize) -> &T;
    }

    pub struct Array2D<'a, T: 'a> {
        pub array: &'a [T],
        pub width: usize
    }

    impl<'a, T> Array2DTest<'a, T> for Array2D<'a, T> {
        fn new(array: &'a [T], width: usize) -> Self {
            Array2D { array, width }
        }

        fn value_at_index(&self, index: usize) -> &T {
            &self.array[index]
        }

        fn value_at_coords(&self, x: usize, y: usize) -> &T {
            let index = coords_to_index(x, y, self.width);

            self.value_at_index(index)
        }
    }
}


#[cfg(test)]
mod tests {
    use array_2d::array_2d;
    use array_2d::array_2d::Array2DTest;

    #[test]
    #[should_panic]
    fn it_does_not_allow_out_of_bounds() {
        let a = array_2d::Array2D { array: &[1], width: 1 };

        a.value_at_coords(1, 2);
    }

    #[test]
    fn it_retrieves_the_value() {
        let a = array_2d::Array2D::new(&[1, 2], 2);
        let expected = &2;

        let result = a.value_at_coords(1, 0);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_retrieves_the_value_at_index() {
        let a = array_2d::Array2D { array: &[1, 2], width: 2 };
        let expected = &2;

        let result = a.value_at_index(1);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_converts_coords_to_index() {
        let x = 2;
        let y = 3;
        let width = 5;
        let expected = 17;

        let result = array_2d::coords_to_index(x, y, width);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_converts_index_to_coords() {
        let index = 17;
        let width = 5;
        let expected = (2, 3);

        let result = array_2d::index_to_coords(index, width);

        assert_eq!(result, expected);
    }
}
