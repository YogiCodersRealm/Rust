fn main() {
    let sum = 5 + 10 ;
    println!("sum of 2 number is!:{sum}");


    let t = true;

    let f:bool = true;

    let z: char = 'Z';


    let tup = (500, 6.5, 40);
    let (_x,_y,z) = tup;

    println!("The value of z is {z}")

    let x:(i32,f64,u8) = (500,6.00,700);
    let five_hundred = x.0;
    let six_hundred = x.1;
    let one = x.2;


    // Array
    let months = [
        "jan","feb","mar"
    ];

    println!{"months = {months}"}

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    
}
