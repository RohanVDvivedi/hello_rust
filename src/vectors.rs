// vectors are resizable arrays

pub fn run() {
    let mut nums : Vec<i64> = vec![0,2,4,6,8,11];
    println!("{:?} length = {}", nums, nums.len());
    nums[5] = 10;
    println!("{:?}", nums);
    nums.push(12);
    println!("{:?}", nums);
    nums.pop();
    println!("{:?}", nums);
    nums.push(12);
    nums.push(12);
    let len = nums.len();
    nums[len - 1] = 14;
    println!("{:?}", nums);
    let slize = &nums[2..5];
    println!("{:?}", slize);

    for x in nums.iter() {
        println!("{}", x)
    }
    for x in nums.iter_mut() {
        (*x) /= 2;
    }
    println!("{:?}", nums);
}