fn main() {
    use std::io;

    let mut input = String::new();
    let mut input_2 = String::new();
    let mut input_3 = String::new();
    println!("enter a number");
    io::stdin().read_line(&mut input).expect("an error");
    let ben: i32 = input.trim().parse().expect("wrong data type");

    println!("enter your calculation symbol");
    io::stdin()
        .read_line(&mut input_3)
        .expect("failed experiment");
    let thor:char = input_3.trim().parse().unwrap();

    println!("enter a number");
    io::stdin()
        .read_line(&mut input_2)
        .expect("failed experiment");
    let jim: i32 = input_2.trim().parse().expect("wrong date type in input2");
    let mut result :f32 = 0.0;

    if thor == '+' {
     result = ben as f32 + jim as f32;
    } else if thor == '-' {
     result = ben as f32  - jim as f32 ;
    } else if thor == '/' {
     result = ben as f32  / jim as f32 ;
    } else if thor == '*' {
     result = ben as f32  * jim as f32 ;
    }

    println!("{} {} {} is equal to {}", jim, thor, ben, result);
}
