pub trait Array2D<T> {
    fn coords_to_index(&self, x: usize, y: usize, width: usize) -> usize {
        x + y * width
    }

    fn index_to_coords(&self, index: usize, width: usize) -> (usize, usize) {
        (index % width, index / width)
    }

    fn value_at_index(&self, index: usize) -> T;
    fn value_at_coords(&self, x: usize, y: usize, width: usize) -> T {
        self.value_at_index(self.coords_to_index(x, y, width))
    }
}

#[cfg(test)]
mod tests {
    use array_2d::*;

    struct Foo<'a> {
        array: &'a [usize],
        width: usize
    }

    impl<'a> Array2D<usize> for Foo<'a> {
        fn value_at_index(&self, index: usize) -> usize {
            self.array[index]
        }
    }

    #[test]
    #[should_panic]
    fn it_does_not_allow_out_of_bounds() {
        let a = Foo { array: &[1], width: 1 };

        a.value_at_coords(1, 2, a.width);
    }
}
