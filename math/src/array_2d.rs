pub trait Array2D<T> {
    fn coords_to_index(&self, x:usize, y: usize, width: usize) -> usize {
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

//#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
//pub enum Type<'a> {
//    Pixels(&'a [usize]),
//    Colors(&'a [(f32, f32, f32, f32)]),
//    Positions(&'a [f64]),
//    Indices(&'a [usize])
//}
//
//pub struct Array2dInfo<'a> {
//    array: &'a Type<'a>,
//    width: usize,
//}
//
//pub fn array_2d_value<'a>(x: usize, y: usize, info: &Array2dInfo) -> Result<Type, Type> {
//    let index: usize = x + y * info.width;
//    let array = info.array;
//
//    let result;
//    match array {
//        Type::Pixels(array) => result = array[index]
//    }
//
//    result
//}
//
//#[cfg(test)]
//mod tests {
//    use array_2d::*;
//
//    #[test]
//    #[should_panic]
//    fn it_does_not_allow_out_of_bounds() {
//        let array = &Type::Indices(&[1]);
//        let width = 1;
//        let (x, y) = (1, 2);
//        let array_2d_info = Array2dInfo { array: &array, width };
//
//        array_2d_value(x, y, &array_2d_info);
//    }
//
//
//    #[test]
//    fn it_retrieves_results_from_coord() {
//        let array = &Type::Pixels(&[1, 2, 3, 4, 5, 6]);
//        let width = 2;
//        let x = 0;
//        let y = 1;
//        let array_2d_info = Array2dInfo { array: &array, width };
//
//        let expected = &3;
//
//        let result = array_2d_value(x, y, &array_2d_info);
//
//        assert_eq!(expected, result);
//    }
//}
