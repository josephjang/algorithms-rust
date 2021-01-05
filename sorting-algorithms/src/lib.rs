#![warn(clippy::all)]

/// The function implements the optimized bubble sort.
/// It sorts the element in the input slice by mutating it.
///
/// This is a exact implementation of the pseudo code from
/// [Wikipedia](https://en.wikipedia.org/wiki/Bubble_sort):
///
/// ```text
/// procedure bubbleSort(A : list of sortable items)
///     n := length(A)
///     repeat
///         newn := 0
///         for i := 1 to n - 1 inclusive do
///             if A[i - 1] > A[i] then
///                 swap(A[i - 1], A[i])
///                 newn := i
///             end if
///         end for
///         n := newn
///     until n ≤ 1
/// end procedure
/// ```
///
/// # Arguments
///
/// * list - The list to be sorted
///
/// # Examples
///
/// ```
/// use bubble_sort::bubble_sort;
///
/// let mut list = [ 2, 3, 5, 4, 1 ];
/// bubble_sort(&mut list);
/// assert_eq!(list, [ 1, 2, 3, 4, 5 ]);
/// ```
///
pub fn bubble_sort<T: PartialOrd>(list: &mut[T]) {
    let mut n = list.len();
    loop {
        let mut lastly_swapped= 0;
        for i in 1..n {
            if list[i - 1] > list[i] {
                list.swap(i - 1, i);
                lastly_swapped = i;
            }
        }
        n = lastly_swapped;
        if n <= 1 {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_bubble_sort() {
        use super::bubble_sort;

        let mut list = [4, 8, 9, 2, 3];
        bubble_sort(&mut list);
        assert_eq!(list, [2, 3, 4, 8, 9]);
    }

    #[test]
    fn test_bubble_sort_with_ordered_array() {
        use super::bubble_sort;

        let mut list = [2, 3, 4, 8, 9];
        bubble_sort(&mut list);
        assert_eq!(list, [2, 3, 4, 8, 9]);
    }

    #[test]
    fn test_bubble_sort_with_reverse_ordered_array() {
        use super::bubble_sort;

        let mut list = [9, 8, 4, 3, 2];
        bubble_sort(&mut list);
        assert_eq!(list, [2, 3, 4, 8, 9]);
    }

    #[test]
    fn test_bubble_sort_with_single_element_array() {
        use super::bubble_sort;

        let mut list= [ 42 ];
        bubble_sort(&mut list);
        assert_eq!(list, [ 42 ]);
    }

    #[test]
    fn test_bubble_sort_with_empty_array() {
        use super::bubble_sort;

        let mut list: [usize; 0] = [];
        bubble_sort(&mut list);
        assert_eq!(list, []);
    }

    #[test]
    fn test_bubble_sort_with_vec() {
        use super::bubble_sort;

        let mut list = vec![4, 8, 9, 2, 3];
        bubble_sort(&mut list);
        assert_eq!(list, [2, 3, 4, 8, 9]);
    }
}
