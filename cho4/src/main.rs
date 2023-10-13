use std::fmt::Debug;

fn main() {
    /*    let w = String::from("world");
        let s = "hello";
        let t = s;
        let x = w;
        println!("{}", w);
        println!("{}", s);
        println!("{:p}", s);
        println!("{:p}", t);*/
    /* let x = "sd";
     println!("addr of s={:p} on stack", &x);
     println!("addr of data 'hello'={:p} on heap", x.as_ptr());


     let w = sayHi();
     println!("addr of s={:p} on stack", &w);
     println!("addr of data 'hello'={:p} on heap", w.as_ptr());

     let s = w;
     println!("addr of s={:p} on stack", &s);
     println!("addr of data 'hello'={:p} on heap", s.as_ptr());

     println!("{}", s);


     println!("============================");

     let q1 = String::from("hello world");
     println!("addr of q={:p} on stack", &q1);
     println!("addr of data 'hello'={:p} on heap", q1.as_ptr());


     let q2 = String::from("hello world");
     println!("addr of q={:p} on stack", &q2);
     println!("addr of data 'hello'={:p} on heap", q2.as_ptr());
 */

    /*    let s = 3;
        let x = s;
        println!("{}", s);

        let a1 = "a1";
        let a2 ="a2";*/
    /*    let hi = String::from("hello");
        let hello = sayHi(hi);

        println!("{}", hello);

        let a = 1;
        let b = 2;
        let sc = num_plus(a, b);
        println!("{}", sc);
        println!("{}", a);
        println!("{}", b);*/
   /* let s1 = say_hi();
    let hi = String::from("hello");
    let hello_world = sayHi(hi);
    println!("{}", s1);
    println!("{}", hello_world);

    println!("{}", s3);*/

    let mut  s1 = String::from("张三");

/*    let s2 = &mut s1;

    let mut  s3 = String::from("张三");

    let s4 = &mut s1;*/

    let s2 = &s1;
    let s3 = &s1;




    /*    let info = user_info( &mut s1);*/

  /*  println!("{}", info);*/
    println!("{:p}", &s1);
    println!("{:p}", &s2);
    println!("{:p}", &s3);




}


fn user_info(name:&mut String)->String {
    name.push_str("SDFASD");
    name.to_string()
}


fn say_hi() -> String {
    let hello = String::from("hello world");
    hello
}


/*fn num_plus(a: i32, b: i32)->i32{
    a + b
}
*/
fn sayHi(hi: String) -> String {
    hi + " world"
}
