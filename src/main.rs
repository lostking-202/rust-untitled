mod test;

use std::collections::HashMap;
use std::{io, thread};
use rand::Rng;
use std::cmp::Ordering;
use std::ops::Index;
use std::sync::mpsc;
use std::time::Duration;

fn guess_num() ->String{
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();
    // & 表示这个参数是一个引用（reference），这为你提供了一种方法，让代码的多个部分可以访问同一处数据，而无需在内存中多次拷贝。引用是一个复杂的特性
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    return guess;
}

fn compare_guess_num(){
    let secret_number=rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);



    loop{
        let guess:u32=match guess_num().trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };
        println!("Please input your guess.");

        match guess.cmp(&secret_number){
            Ordering::Less=>println!("Too small"),
            Ordering::Greater=>println!("Too big"),
            Ordering::Equal=>{
                println!("You win!");
                break;
            }
        }
    }
}

fn shadow_demo(){
    let x=5;
    println!("The value of x is: {}", x);
    // 这里不是重新赋值，而是重新申明变量
    let x=x+1;
    println!("The value of x is: {}", x);
    {
        let x=x*2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}

fn collection_demo(){
    // 可以放不同的数据类型
    let tup:(i32,f64,u8)=(500,6.4,1);
    // 首先创建一个元组并将其绑定到变量 tup 上。 然后它借助 let 来使用一个模式匹配 tup，并将它分解成三个单独的变量 x、y 和 z。 这过程称为解构（destructuring）
    let (x,y,z)=tup;

    let x: (i32, f64, u8) = (500, 6.4, 1);
    // 也可以通过索引获取
    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
    // 没有任何值的元组 () 是一种特殊的类型，只有一个值，也写成 ()。该类型被称为单元类型（unit type），该值被称为单元值（unit value）

    //将多个值组合在一起的另一种方式就是使用数组（array）。与元组不同，数组的每个元素必须具有相同的类型。与某些其他语言中的数组不同，Rust 中的数组具有固定长度。
    // 当你希望将数据分配到栈（stack）而不是堆（heap）时（我们将在第 4 章中进一步讨论栈和堆），或者当你希望确保始终具有固定数量的元素时，数组特别有用
    let a = [1, 2, 3, 4, 5];
    //使用方括号编写数组的类型，其中包含每个元素的类型、分号，然后是数组中的元素数，如下所示：
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // 变量名为 a 的数组将包含 5 个元素，这些元素的值初始化为 3。这种写法与 let a = [3, 3, 3, 3, 3]; 效果相同，但更简洁。
    let a = [3; 5];
    // 访问数组元素
    let first = a[0];
    let second = a[1];
}

fn code_block_demo(){
    let y={
        let x=3;
        x+1
    };
    println!("The value of y is {}",y)
}

fn if_demo(){
    let condition:bool=true;
    let number= if condition{5}else {6};
}

fn loop_demo(){
    for number in (1..4).rev(){
        println!("{}!",number);
    }
    println!("LIFTOFF!!!");
}

// 内存在拥有它的变量离开作用域后就被自动释放

// 之前我们提到过当变量离开作用域后，Rust 自动调用 drop 函数并清理变量的堆内存。不过图 4-2 展示了两个数据指针指向了同一位置。这就有了一个问题：当 s2 和 s1 离开作用域，他们都会尝试释放相同的内存。这是一个叫做 二次释放（double free）的错误，也是之前提到过的内存安全性 bug 之一。两次释放（相同）内存会导致内存污染，它可能会导致潜在的安全漏洞。
//
// 为了确保内存安全，这种场景下 Rust 的处理有另一个细节值得注意。在 let s2 = s1 之后，Rust 认为 s1 不再有效，因此 Rust 不需要在 s1 离开作用域后清理任何东西。看看在 s2 被创建之后尝试使用 s1 会发生什么；这段代码不能运行：

// 调用方法参数可以移动所有权,如果没有返回值，表示离开作用域，需要清理，后续也无法使用
fn borrow_demo(){
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);
    // 这行代码运行报错报错
    // println!("{}", s);  // value borrowed here after move
}

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

// 返回值转移所有权，虽然方法内引用结果，但变量通过返回值的形式重新定义并赋值
fn give_demo(){
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // 这行代码运行报错报错
    // println!("{}", s2); value borrowed here after move
    println!("{}", s3);
}
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    a_string  // 返回 a_string 并移出给调用的函数
}

// & 符号就是 引用，它们允许你使用值但不获取其所有权,借用和引用也不能修改原值
// 正如变量在默认情况下是不可变的一样，引用也是不可变的。我们无法通过引用修改内容。
fn ref_demo(){
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// 防止同一时间对同一数据进行多个可变引用的限制允许可变性,不过是以一种受限制的方式允许。

// 这个限制的好处是 Rust 可以在编译时就避免数据竞争。

fn mut_ref_dem1(){
    let mut s = String::from("hello");

    let r1 = &mut s;
    // 会报错，不能同时存在多个可变引用
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);
}

fn mut_ref_demo2(){
    // 可以使用大括号来创建一个新的作用域，以允许拥有多个可变引用
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;
}

//  也不能在拥有不可变引用的同时拥有可变引用
fn mut_ref_demo3(){
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    //let r3 = &mut s; // 大问题

    //println!("{}, {}, and {}", r1, r2, r3);
}

fn mut_ref_demo4(){
    let mut s = String::from("hello");

    // 错误发生在 change 函数调用之后。问题在于 Rust 的借用规则不允许在存在可变引用的情况下再次借用同一个变量
    // 这里相当于可变引用多次，是rust不允许的
    // let r1 = &mut s; // 没问题

    change(&mut s);

    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
// 悬垂指针
// 这里我的理解是，返回引用会使该变量的生命周期处于无法管理的状态 Missing lifetime specifier [E0106]
// 是指一个指针变量指向的内存已经被释放或超出了其有效作用域，而指针本身仍然存在并被使用。悬垂指针是许多其他编程语言中常见的问题
// ide会直接提示不能返回引用
// fn dangling_ref_demo(){
//     let reference_to_nothing = dangle();
// }
// fn dangle() -> &String {
//     let s = String::from("hello");
//
//     &s
// }
fn first_word(s:&String)->usize{
    let bytes=s.as_bytes();
    for(i,&item)in bytes.iter().enumerate(){
        if item==b' '{
            return i
        }
    }
    s.len()
}

fn slice_demo1(){
    let mut s=String::from("hello world");
    let word=first_word(&s);
    s.clear();
    println!("{}",word)
}

fn first_word2(s:&String)->&str{
    let bytes=s.as_bytes();
    for(i,&item)in bytes.iter().enumerate(){
        if item==b' '{
            return &s[0..i]
        }
    }
    &s[..]
}
// 当拥有某值的不可变引用时，就不能再获取一个可变引用。因为 clear 需要清空 String，它尝试获取一个可变引用。在调用 clear 之后的 println! 使用了 word 中的引用，所以这个不可变的引用在此时必须仍然有效。Rust 不允许 clear 中的可变引用和 word 中的不可变引用同时存在，因此编译失败。Rust 不仅使得我们的 API 简单易用，也在编译时就消除了一整类的错误！
fn slice_demo2(){
    let s=String::from("hello world");
    let word=first_word2(&s);
    println!("{}",word.len());
    println!("{}",s.len())
}


fn vec_demo1(){
    let v=vec![1,2,3,4,5];
    let mut num=v[1];
    num=2;
    println!("{:?}",v)
}

fn vec_demo2(){
    let mut v=Vec::new();
    let s=String::from("hello");
    v.push(s);
    // 集合操作都会导致所有权转移
    // println!("{}",s)
}

fn map_demo1(){
    let teams=vec![String::from("Blue"),String::from("Yellow")];
    let initial_scores=vec![10,50];
    let scores:HashMap<_,_>=teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}",scores)
}

fn map_demo2(){
    let mut field_name=String::from("Favorite color");
    let field_value=String::from("Blue");

    let mut map=HashMap::new();
    // 插入操作也会消耗掉数据的所有权
    map.insert(field_name,field_value);
    println!("{:?}",map);

    // String 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者
    //insert 操作也会导致所有权转移
    // println!("{}",field_name);
}


fn map_demo3(){
    let mut scores=HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);
    // 迭代类似于方法调用，会使用掉数据的所有权
    for (key,value) in &scores{
        println!("{},{}",key,value)
    }
    println!("{:?}",scores)
}

fn map_demo4(){
    let text="hello world wonderful world";
    let mut map=HashMap::new();
    for word in text.split_whitespace(){
        // or_insert 方法事实上会返回这个键的值的一个可变引用（&mut V）。这里我们将这个可变引用储存在 count 变量中，所以为了赋值必须首先使用星号（*）解引用 count
        let conut=map.entry(word).or_insert(0);
        *conut+=1;
    }
    println!("{:?}",map)
}

/*fn get_longer_string(s1:&String,s2:&String)->&String{
    if s1.len()>s2.len(){
        s1
    }else{
        s2
    }
}*/

// 定义 Student 结构体
#[derive(Clone, Debug)]
struct Student {

    age: u32,
}

// 编写自由函数 max_age，它接受两个 Student 实例的引用，并返回年龄较大的学生的引用
fn max_age(student1: Student, student2: Student) -> Student {
    if student1.age > student2.age {
        student1
    } else {
        student2
    }
}

fn thread_demo1(){
    let handle=thread::spawn(||{
        for i in 1..10{
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1))
        }
    });
    for i in 1..5{
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1))
    }
    handle.join().unwrap();
}

//使用 move 关键字强制获取它使用的值的所有权
fn thread_demo2(){
    let v=vec![1,2,3];
    let handle=thread::spawn(move || {
        println!("Here's a vector:{:?}",v);
    });
    handle.join().unwrap();
}

fn channel_demo1(){
    let (tx,rx)=mpsc::channel();
    thread::spawn(move || {
        let val=String::from("hi");
        tx.send(val).unwrap();
        // 发送完就不能使用了
        // println!("val is {}",val)
    });
    let received=rx.recv().unwrap();
    println!("Got:{}",received);
}

fn channel_demo2(){

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut v=Vec::new();
    let mut count:i32=0;
    for i in s.as_bytes(){
        if(v.contains(&i)){
            let index=v.iter().position(|&x| x==i).unwrap();
            v=v.split_off(index+1);
            v.push(i);
        }else{
            v.push(i);
        }
        count=if count>v.len() as i32{count}else { v.len() as i32 }

    }
    return count;
}

// 滑动窗口
// pub fn length_of_longest_substring2(s: String) -> i32 {
//     let mut count:i32=0;
//     let mut v=Vec::new();
//     for i in s.as_bytes(){
//         while(!v.contains(i)){
//             // v.push()
//         }
//
//     }
//     return 0;
// }

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max=nums[0];
    let mut sum=nums[0];
    for num in nums.iter(){
        if num< &0 {

        }
    };
    0
}

fn main() {
   println!("{}",length_of_longest_substring(String::from("abcabcbb")))
}

