#![allow(unused)]
fn main() {
    let arr1 = [3, 5, 7, 9, 12];
    let arr2 = [3, 5, 8, 10, 12];
    let size = 5;

    let mut res = dif_sum(&arr1, &arr2, size);
    println!("{:?}", res);
}

fn dif_sum(a1:&[i32],a2:&[i32],size:usize)->i32{
    
    if size == 0{
        return 0
    };
    let mut last_ele_diff = (a1[size-1] as i32 - a2[size-1] as i32).abs();

    return dif_sum(a1, a2, size-1) + last_ele_diff 
}
