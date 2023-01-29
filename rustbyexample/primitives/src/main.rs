fn main() {
    // // variables can be type annotated
    // let logical: bool = true;

    // let _a_float: f64 = 5.3; // regular annotation

    // let an_integer = 5i32; // suffic annotation

    // let default_float = 3.4;

    // println!("{default_float}");

    // Integer addition

    // println!("1 + 2 = {}", 1u32 + 2);

    // // short circuiting

    // println!("True and False is {}", true && false);
    // println!("True or False is {}", true || false);
    // println!("Not true is {}", !true)

    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // analyze_slice(&xs[1..3]);

    // let ys: [i32; 10] = [5; 10];

    // println!("{:?}", ys);

    // for i in 0..xs.len() + 1 {
    //     match xs.get(i) {
    //         Some(xval) => println!("{} : {}", i, xval),
    //         None => println!("OOPS STOP THERE"),
    //     }
    // }

    let mut new_nums = [0; 5];

    println!("{:?}", new_nums);

    for (i, element) in xs.iter().enumerate() {
        new_nums[i] = element * 2;
    }

    println!("{:?}", new_nums)
}

fn analyze_slice(slice: &[i32]) {
    println!("The first element of slice is {}", slice[0]);
    println!("The length of slice is {}", slice.len());
    println!("The slice is {:?}", slice);
}
