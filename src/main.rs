// use std::io;
fn main() {

    //slice 切片 是引用，没有所有权，允许你引用集合中一段连续的元素序列

    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
    



    // let  s = String::from("hello world");
    // let s ="hello world";
    //let word = first_word(&s[6..10]);
    // let word = first_word(s);
    // println!("{}",word);


    // let word = first_word(&s);
    // println!("{}",word);
    // let hello =&s[0..5]; //左闭右开
    // let world = &s[6..11]; //左闭右开
    // println!("{},{}",hello,world);

    //let word = first_word(&s);


    // 悬垂引用
    // let reference_to_nothing = dangle();

    //可变引用
    // let mut s = String::from("hello");
    // change(&mut s);
    // println!("{}", s);


    //reference 引用 &
    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // println!("The length of {} is {}.", s1, len);




    // 所有权 ownership
    // Rust 中的每一个值都有一个被称为其所有者（owner）的变量,值在任何时刻有且只有一个所有者，
    //当所有者离开作用域，这个值将被丢弃

    // let s = String::from("hello");

    // takes_ownership(s);
    // // println!("s={}",s);

    // let x = 5;

    // makes_copy(x);

    // println!("{s}",s);
    // println!("{}",x);
    


    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("s2 = {}, s1 = {}",s2,s1);
    // s.push_str(", world!");
    // println!("{}",s);





    //练习2 生成n阶斐波那契数列
    // println!("请输入斐波那契数列第及阶n");
    // let mut n = String::new();
    // io::stdin().read_line(&mut n).expect("输入错误");
    // let n = n.trim().parse().expect("输入的值不是一个数字");
    // let f = fb(n);
    // println!("第{n}阶斐波那契数列的值是：{f}");


    //练习1 转换摄氏度与华氏温度
    // let wendu = 33.0;
    // let huashi = sheshi_to_huashi(wendu);
    // println!("当前华氏温度是：{huashi}");


    //循环 2022.8.10
    // for number in (1..4).rev(){
    //     println!("{number}");
    // }
    // println!("LIFTOFF!!!");




    //let a = [1,2,3,4,5];
    // let mut index = 0;
    // while index < a.len() {
    //     println!("the value is: {}",a[index]);
    //     index += 1;
    // }
    
    // for element in a {
    //     println!("the value is: {element}");
    // }



    // let mut counter = 0;
    
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("result is {result}");



    // 函数  2022.8.10
    // test();
    // let a = five();
    // println!("a = {a}");



    // let  a = 3;
    // // a = 5;
    // println!("the value of a is :{}", a);
    // //println!("Hello, world!");


    //数组类型  array
    // let arr = [1,2,3,4,5];
    // // let arr:[i32;5] = [1,2,3,4,5];
    // let a = arr[0];
    // let b = arr[1];
    // println!("a = {a}, b = {b}");    

    //使用用户输入来访问数组中的值
    // let arr  = [1,2,3,4,5];
    // println!("请输入您要访问的数组index");
    // let mut index = String::new();
    // io::stdin().read_line(&mut index).expect("输入错误");
    // let index:usize = index.trim().parse().expect("输入的值不是一个数字");
    // let element = arr[index];
    // println!("你访问的第{index}个数的值是: {element}");

}

// fn first_word(s:&str) -> &str  {
//     let bytes = s.as_bytes();
//     for(i, &item) in bytes.iter().enumerate(){
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//    &s[..]
// }

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }


// fn change(some_thing:&mut String){
//     some_thing.push_str(", world!");
// }


// fn calculate_length(s:&String) -> usize{
//     s.len()
// }



// fn takes_ownership(some_string:String){
//     println!("{}",some_string);
// }

// fn makes_copy(some_interger:i32){
//     println!("{}",some_interger);
// }


//斐波那契数列
// fn fb(n:u64) -> u64{
//     if n <= 0 {
//         panic!("不能小于0");
//     }
//     if n <=2 {
//         1
//     }else {
//         fb(n - 1) + fb(n -2)
//     }

// }

// fn sheshi_to_huashi(num:f64) -> f64{
//    num * 1.8 + 32.0
// }

// 表达式结尾没有分号，表达式有值
// 语句结尾有分号，语句没有返回值，不能赋值给变量
// fn test(){
//     let y = {
//         let x = 3;
//         x +1
//     };
//     println!("y = {y}");
// }

// // 函数的返回值等同于函数体最后一个表达式的值
// fn five() -> i32{
//     5
// }
