fn main (){
/*beginning of assignment */
/* write all code in the main function */
    let wonder: &str= "Pius";
    let path: &str="🙃";
    let iron: &str="ENG2300045";
    println!("what is your name: \n {}\nwhat is your Mat. Number \n {}\nmy favourite is your emoji:\n {} \n  ",wonder,iron, path);
    /* the end of part of one */
    /*beginning part two #using of functions */

test_one("Pius Onwualu");
test_two("ENG2204444");
test(32,2);
test_three("🙃");
}


/*function used in division of numbers*/
fn test(x: i32, y: i32){
    println!("division of two numbers {}",x/y);
}
/*function contains name */
fn test_one(flash: &str){
    println!("name: {flash}");
}
/*function contains matriculation number */
fn test_two(bat: &str){
    println!("Matriculation Number: {bat}");
}

fn test_three(thor: &str){
    println!("my favourite is: {}", thor);
}
