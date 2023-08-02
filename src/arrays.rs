// Arrays are fixed length with same data type elements (Vectors are grow-able)

pub fn run() {
    let mut nums : [i32; 6] = [0,2,4,6,8,9];
    println!("{:?} length = {}, space required {}", nums, nums.len(), std::mem::size_of_val(&nums));
    nums[5] = 10;
    println!("{:?}", nums);
    println!("nums 2 => {}", nums[2]);
    let slice_start = 1;
    let slice_end = 3;
    let slice : &[i32] = &nums[slice_start..slice_end];
    println!("{:?}", slice);
}