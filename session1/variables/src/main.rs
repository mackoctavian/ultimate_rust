
fn main() {
    //mutable
    let mut n: i32 = 5;
    n += 1;
    println!("{n}");

    //Shadowing
    let m = 5;
    let m = m + 1;
    println!("{n}");

    //Scope 
    {
        let n: i32 = 6;
        println!("{n}");
        //This inner n ends here
    }

    //Returning from a scope
    let n = {
        6
    };

    //Returning a unit type ()
    let n = {

    };
}