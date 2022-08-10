use std::io;
fn main() {
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
    let arr  = [1,2,3,4,5];
    println!("请输入您要访问的数组index");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("输入错误");
    let index:usize = index.trim().parse().expect("输入的值不是一个数字");
    let element = arr[index];
    println!("你访问的第{index}个数的值是: {element}");

}
