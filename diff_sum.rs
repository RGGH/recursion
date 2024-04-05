fn main() {
    let arr1 = [3, 5, 7, 9, 12];
    let arr2 = [3, 5, 8, 10, 12];
    let size = 5;

    let mut counter = 0;

    for i in 0..size {
        counter += (arr1[i] as i32 - arr2[i]as i32).abs();
    }

    println!("Counter: {}", counter);
}
