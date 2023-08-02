// Reference pointers point to resource in memory

pub fn run () {
    // primitive array
    let arr1 : [i32; 3] = [1, 2, 3];
    let arr2 = arr1;
    println!("{:?} {:?}", arr1, arr2);

    // with non-primitives, if you assign another variable to a piece of data, the first variable will no longer hold that value.
    // You will need to use a reference (&) to point to the same resource

    // vector is not primitive, lets do the above with a vector

    let vec1 : Vec<i32> = vec!(1, 2, 3);
    // below line will not work
    //let vec2 : Vec<i32> = vec1;
    // instead
    let vec2 : &Vec<i32> = &vec1;
    println!("{:?} {:?}", vec1, vec2);

}