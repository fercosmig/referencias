fn main() {

    // exemplo 1
    {
        let x: i32 = 10;
        let y = &x;
        let z = &x;
    
        println!("{}", x);
        println!("{}", y);
        println!("{}", z);
    }
    
    //exemplo 2
    {
        let mut x: i32 = 10;
        let y = &mut x;

        *y += 1;

        println!("{}", y);
    }
}
