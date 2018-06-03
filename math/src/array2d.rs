pub struct Array2dInfo<'a, T: 'a> {
    array: &'a [T],
    width: usize,
}

pub fn array_2d_value<'a, T>(x: usize, y: usize, info: &'a Array2dInfo<T>) -> &'a T {
    let index: usize = x + y * info.width;

    &info.array[index]
}

#[cfg(test)]
mod tests {
    use array2d::*;

    #[test]
    fn it_retrieves_results_from_coord() {
        let array = [1, 2, 3, 4, 5, 6];
        let width = 2;
        let x = 0;
        let y = 1;
        let array_2d_info = Array2dInfo { array: &array, width};

        let expected = &3;

        let result = array_2d_value(x, y, &array_2d_info);

        assert_eq!(result, expected);
    }
}
