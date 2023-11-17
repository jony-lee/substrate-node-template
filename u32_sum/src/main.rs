fn sum_u32_collection(numbers: &[u32]) -> Option<u32> {
    // 使用 iter() 方法获取迭代器，并调用 fold 方法进行求和
    let sum = numbers.iter().try_fold(0u32, |acc, &num| acc.checked_add(num));

    // 检查是否存在溢出
    sum
}

fn main() {
    // 示例用法
    let numbers = vec![1, 2, 3, 4, 5,std::u32::MAX];
    match sum_u32_collection(&numbers) {
        Some(result) => println!("Sum: {}", result),
        None => println!("Overflow occurred!"),
    }
}
