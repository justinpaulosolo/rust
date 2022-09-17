fn main() {
    /*
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let x = 5;

    let x = x + 1;
    
    {
        let x = x * 2;
        println!("The value of x in the inner schope is: {x}"); // 12
    }
    println!("The value of x is: {x}"); // 6

    let mut num = 2.0 / 3.0;
    println!("{}", num);
    num = 2.0 % 3.0; 
    println!("{}", num);

    */

    // Tuple
    let tup: (i32, f64, u8) = (1000, 2.4, 1);
    println!("{:?}", tup);

    let one_thousand = tup.0;
    let two_point_two = tup.1;
    let one = tup.2;

    println!("{one_thousand}");
    println!("{two_point_two}");
    println!("{one}");
}
