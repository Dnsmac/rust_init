use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {

    let range  = rand::thread_rng().gen_range(1, 101);

    println!("随机100 猜数游戏");
    println!("猜一个数字");


 

    loop {
        
        let mut guess = String::new();
        //读取玩家输入的文字 赋值可改变的 guess ，mut 代表可变  没加 都是constant 常量
        io::stdin().read_line(&mut guess).expect("无法读取");

        println!("你猜的数字是{}", guess);
 
        
        let guess:u32 =  match guess.trim().parse(){
            Ok(num) =>num,
            Err(_) =>continue,
        };

        match guess.cmp(&range) {
            Ordering::Equal =>{
                println!("you win");
                break;
            } 
            
    
            Ordering::Less  => println!("you less"),
            Ordering::Greater => println!("you win greater")
    
            
        };
    }
}
