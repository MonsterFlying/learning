use std::io;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::Add;

fn main() {
    /*   let mut x = 5;
       println!("the value of x is:{}", x);
       x = 3;
       println!("the value of x is：{}", x);

       const THREAD_HOUR_IN_SECONDS: u32 = 60 * 60 * 3;
       println!("the value of x is：{}", THREAD_HOUR_IN_SECONDS);

       let  w = 1;
       let w = w + 2;
       {
           let w = w * 2;
           println!("LINE 1: {w}");

       }
       println!("LINE 2: {w}");


       let mut y = "   ";
       let y = y.len();

       println!("y value: {y}");



       //数据类型

       let a: u8 = 223;
       println!("{}", a);

       let mut s: &str = "sd";
       s = "sdfsd";
       println!("{}",s);

       let t: f32 = 2.2;
       let t: f64 = 2.2;

       let s = true;
       let c = 'z';
       let smail = '😂';
       println!("{}", smail);

       let tup = (1, 23, 3);
       println!("{}", tup.1);

       let array1 = [1, 2, 3, 4];

       let a = [3; 4];

       println!("{}",array1.len());
       println!("{},{}", a[0], a.len());
   */

    /*    let num = [1, 2, 3, 4, 5, 6, 7];
        let mut index = String::new();
        loop {
            io::stdin()
                .read_line(&mut index)
                .expect("failed to read line");


            let index: usize = index.trim().parse().expect("请输入正确的index1");
            if index > 6 {
                println!("请输入正确的index2");
                continue;
            }
            let show_num = num[index];

            println!("这个index对应的数字是{}", show_num);
        }
    */

    let s = {
        let w = 2;
        w + 2
    };
    println!("{}", s);

    let y = {
        let x = 3;
        x + 1
    };
    /*
        println!("The value of y is: {}", y);


        println!("{}", printMoreAgr(21, "张三"));
        let five=five();
        println!("{}",five);


        let condition=true;
        let number = if condition { 4 } else { 3 };
        println!("{}", number);*/

   /*

   // loop 外层标签
   let mut start = 1;
    let mut end = 1;
    'S: loop {
        loop {
            if (end == 10) {
                break;
            }
            print!("{}*{}={} ", start, end, start * end);
            end += 1;
        }
        println!();
        if (start == 10) {
            break 'S;
        }
        start += 1;
        end = start;
    }
*/

/*
    //loop 返回值
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if (counter == 10) {
            break counter * 2;
        }
    };
    println!("result {}", result);*/


  /*
  //loop循环条件
    let mut number = 10;
    while number!=0 {
        println!("number value:{}", number);
        number -= 1;
    }
*/
  /*  let number = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut index = 0;
    /*while index<number.len() {
        println!("index value:{}", number[index]);
        index += 1;
    }
*/
    for el in number{
        println!("index value:{}",el);
    }



    for number in(0..20).rev(){
        println!("{}", number);
    }

    */


}

/*fn another_function(x: i32){
    println!("sdfasdfasd {}", x);
}
*/


fn five() -> i32 {
    2
}

fn printMoreAgr(age: i32, name: &str) -> bool {
    println!("name:{}, age:{} ", name, age);
    return true;
}
