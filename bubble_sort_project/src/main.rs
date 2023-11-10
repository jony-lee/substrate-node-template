// 固定类型的数组排序
fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

// 使用泛型实现数字排序
fn generic_bubble_sort<T: PartialOrd, const N: usize>(arr: &mut [T; N]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    // 测试固定类型的数组排序
    let mut arr1 = [4, 2, 8, 5, 1];
    bubble_sort(&mut arr1);
    println!("排序后结果: {:?}", arr1);

    // 测试范型和PartialOrd实现的排序
    let mut arr2 = ["apple", "banana", "orange", "grape", "kiwi"];
    generic_bubble_sort(&mut arr2);
    println!("排序后结果 {:?}", arr2);
}
