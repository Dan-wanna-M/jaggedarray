pub mod jagged_array;
pub mod vec_like;

#[cfg(test)]
mod tests {
    use jagged_array::JaggedArrayMutViewTrait;

    use super::*;
    use crate::jagged_array::JaggedArray;
    use crate::jagged_array::JaggedArray1DViewTrait;
    use crate::jagged_array::JaggedArrayViewTrait;
    #[test]
    fn push_1d_test() {
        let mut data = jagged_array::JaggedArray::<i32, Vec<u16>, 1>::new();
        // data.new_row::<0>();
        // data.view();
        data.push_to_last_row(1);
        assert!(data[[0]] == 1);
        data.push_to_last_row(4);
        data.push_to_last_row(5);
        data.push_to_last_row(6);
        assert!(data[[1]] == 4);
        assert!(data[[2]] == 5);
        assert!(data[[3]] == 6);
        data.push_to_last_row(7);
        assert!(data[[4]] == 7);
    }

    #[test]
    fn push_2d_test() {
        let mut data = jagged_array::JaggedArray::<i32, Vec<u16>, 2>::new();
        data.new_row::<0>();
        data.push_to_last_row(1);
        assert!(data[[0, 0]] == 1);
        assert!(data.view::<1, 1>([0])[[0]] == 1);
        let view = data.view_mut::<1, 1>([0]);
        let slice = view.as_slice();
        assert!(slice == [1]);
        unsafe {
            assert!(data.view_unchecked::<1, 1>([0])[[0]] == 1);
        }
        unsafe {
            assert!(*data.get_unchecked([0, 0]) == 1);
        }
        let a = data.view::<1, 1>([0]).as_slice();
        assert!(a[0] == 1);
        data.new_row::<0>();
        data.extend_last_row([4, 5, 6].into_iter());
        assert!(data[[1, 0]] == 4);
        assert!(data.view::<1, 1>([1])[[0]] == 4);
        unsafe {
            assert!(data.view_unchecked::<1, 1>([1])[[0]] == 4);
        }
        assert!(data[[1, 1]] == 5);
        assert!(data.view::<1, 1>([1])[[1]] == 5);
        unsafe {
            assert!(data.view_unchecked::<1, 1>([1])[[1]] == 5);
        }
        assert!(data[[1, 2]] == 6);
        assert!(data.view::<1, 1>([1])[[2]] == 6);
        unsafe {
            assert!(data.view_unchecked::<1, 1>([1])[[2]] == 6);
        }
        data.new_row::<0>();
        data.new_row::<0>();
        data.push_to_last_row(7);
        assert!(data[[3, 0]] == 7);
        assert!(data.view::<1, 1>([3])[[0]] == 7);
        unsafe {
            assert!(data.view_unchecked::<1, 1>([3])[[0]] == 7);
        }
        assert!(data.pop_from_last_row() == Some(7));
        data.remove_last_row::<0>();
        assert!(data[[0, 0]] == 1);
        data.remove_last_row::<0>();
        data.remove_last_row::<0>();
        data.remove_last_row::<0>();
        println!("{:?}", data);
        assert!(data.is_empty());
    }

    #[test]
    fn push_3d_test() {
        let mut data = jagged_array::JaggedArray::<i32, Vec<u16>, 3>::new();
        data.new_row::<0>();
        data.new_row::<1>();
        data.push_to_last_row(1);
        assert!(data[[0, 0, 0]] == 1);
        assert!(data.view::<1, 2>([0]).view::<1, 1>([0])[[0]] == 1);
        assert!(unsafe{data.view_unchecked::<1, 2>([0]).len()==1});
        unsafe {
            assert!(*data.get_unchecked([0, 0, 0]) == 1);
        }
        data.new_row::<1>();
        data.push_to_last_row(4);
        data.push_to_last_row(5);
        assert!(data[[0, 1, 0]] == 4);
        assert!(data[[0, 1, 1]] == 5);
        data.new_row::<1>();
        data.push_to_last_row(7);
        data.push_to_last_row(8);
        data.push_to_last_row(9);
        assert!(data[[0, 2, 0]] == 7);
        assert!(data[[0, 2, 1]] == 8);
        assert!(data[[0, 2, 2]] == 9);

        data.new_row::<0>();
        data.new_row::<1>();
        data.push_to_last_row(10);
        data.push_to_last_row(11);
        data.push_to_last_row(12);
        data.push_to_last_row(13);
        assert!(data[[1, 0, 0]] == 10);
        assert!(data[[1, 0, 1]] == 11);
        assert!(data[[1, 0, 2]] == 12);
        assert!(data[[1, 0, 3]] == 13);
        data.new_row::<0>();
        data.new_row::<0>();
        data.new_row::<1>();
        data.push_to_last_row(100);
        // assert!(data[[2, 0, 0]] == 100);
        assert!(data[[3, 0, 0]] == 100);
        unsafe {
            assert!(*data.get_unchecked([3, 0, 0]) == 100);
        }
        data.append(data.clone());
        assert!(data[[7, 0, 0]] == 100);
        assert!(data.view::<1, 2>([7]).view::<1, 1>([0])[[0]] == 100);
        data.remove_last_row::<0>();
        assert!(data[[0, 0, 0]] == 1);
        assert!(data[[0, 1, 0]] == 4);
        assert!(data[[0, 1, 1]] == 5);
        assert!(data[[0, 2, 0]] == 7);
        assert!(data[[0, 2, 1]] == 8);
        assert!(data[[0, 2, 2]] == 9);
    }
    #[test]
    fn push_4d_test() {
        let mut data = jagged_array::JaggedArray::<i32, Vec<u16>, 4>::new();
        data.new_row::<0>();
        data.new_row::<1>();
        data.new_row::<2>();
        data.push_to_last_row(1);
        data.new_row::<2>();
        data.push_to_last_row(4);
        data.push_to_last_row(5);
        data.new_row::<2>();
        data.push_to_last_row(7);
        data.push_to_last_row(8);
        data.push_to_last_row(9);
        data.new_row::<1>();
        data.new_row::<2>();
        data.push_to_last_row(10);
        data.push_to_last_row(11);
        data.push_to_last_row(12);
        data.push_to_last_row(13);
        data.new_row::<1>();
        data.new_row::<1>();
        data.new_row::<2>();
        data.push_to_last_row(100);
        assert!(data[[0, 0, 0, 0]] == 1);
        assert!(data[[0, 0, 1, 0]] == 4);
        assert!(data[[0, 0, 1, 1]] == 5);
        assert!(data[[0, 0, 2, 0]] == 7);
        assert!(data[[0, 0, 2, 1]] == 8);
        assert!(data[[0, 0, 2, 2]] == 9);
        assert!(data[[0, 1, 0, 0]] == 10);
        assert!(data[[0, 1, 0, 1]] == 11);
        assert!(data[[0, 1, 0, 2]] == 12);
        assert!(data[[0, 1, 0, 3]] == 13);
        assert!(data[[0, 3, 0, 0]] == 100);
        data.new_row::<0>();
        data.new_row::<0>();
        data.new_row::<1>();
        data.new_row::<2>();
        data.new_row::<2>();
        data.new_row::<0>();
        data.new_row::<1>();
        data.new_row::<2>();
        data.new_row::<2>();
        data.push_to_last_row(1000);
        data.push_to_last_row(1100);
        data.push_to_last_row(1200);
        data.push_to_last_row(1300);
        assert!(data[[3, 0, 1, 0]] == 1000);
        assert!(data[[3, 0, 1, 1]] == 1100);
        assert!(data[[3, 0, 1, 2]] == 1200);
        assert!(data[[3, 0, 1, 3]] == 1300);
    }
}
