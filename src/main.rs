use std::io; // для получения введеных данных
use std::cmp::Ordering;
use rand::Rng; // зависимость для рандомного числа 

fn main() {
    guess_the_number()
}

fn guess_the_number() {
    println!("Угадай число");
    
    let secret_number = rand::thread_rng().gen_range(1..100);
    
    loop {
        println!("Пожалуйста, введите загаднное число");
    
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess).expect("Не удалось прочитать строку!");
        
        println!("Ваше число: {}", guess);
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Пожалуйста введите число от 1 до 100");
                continue; 
            }
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Маленькое"),
            Ordering::Equal => {
                println!("Вы угадали!");
                break;
            },
            Ordering::Greater => println!("Большое"),
        };
    }
}