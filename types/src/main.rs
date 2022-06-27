use std::{format};
use std::any::type_name;
use std::fmt::Debug;
//use debugtrace::Trace;
//use stdext::function_name;

/// 类型 primitives
/// 标量类型（scalar type）
fn main() {

    // 字符
    let ch = 'a';
    // 字符串
    let  str  = format!("{} {}","hello","world!");
    let  str_from = String::from("测试 rust 字符串");
    let mut i  = 0;
    for x in str_from.chars() {
        println!("index={}, value={}",i,x);
        i +=1 ;
    }
    // 过滤
    let new_str= str_from.chars().filter(|&x| x!=' ');

    for x in new_str.clone().into_iter() {
        println!("value={}",x);
    }
    for x in new_str.clone().into_iter() {
        println!("value={}",x);
    }
    let mut str_new =String::new();
    // 过滤
    new_str.clone().for_each( |x| str_new.push(x));

    //new_str.for_each( |x| str_new.push(x)); // 不clone 则new_str 被move, new_str 变量已消失
    println!("new_str={}",str_new);

    let str_ref = str_from.as_str();
    println!("str.len {}",str.chars().count());
    let rep = str_from.replace("rust","12");
    println!("before : {},replace after: {}",str_ref,rep);
    // 数字
    let num = 12;
    let i : isize = 123;
    let i128 : i128 = 1228;
    let x : i64  = 990;
    let f : f64 =  100.00;
    let fx : f32 = 1.00;
    let us : usize = 123;
    // 取反
    let  zero_not: usize = !0;
    let mut zero_not2 : usize  =1111;
    let xor : usize = 9;
    // 取系统位数
    let bit : usize = 32<<((!0 as usize) >>63);
    let u32 : u32 = 122;
    let u64: u64 = 12333;
    let u128 : u128 = 122222;
    // 布尔
    let yes  = true;
    let no  = false;

    // 复合类型（compound type）
    // 单元类型
    let null = ();
    // 元组
    let tuple = (1, "123", true);
    // 数组
    let arr = [1,2,3,4];
    let str_arr = ["123","hee","1222"];
    // 结构体struct
    #[derive(Debug)]
    pub struct Obj {
        age : isize,
        name : &'static str
    } // C 结构
    #[derive(Debug)]
    struct Pair(usize,usize); // 元组结构
    let p = Pair(0,1);
    #[derive(Debug)]
    struct Unit; // 单元结构

    let obj_new = Obj{
        age: 5,
        name: "rust"
    };
    let u = Unit;
    // enum 枚举, 无参枚举
    #[derive(Debug)]
    enum RGB {
        Red,
        Green,
        Blue,
    }
    // 代参枚举
    #[derive(Debug)]
    enum Color {
        Red   = 0xfff0000,
        Green = 0x00f0000,
        Blue  = 0x00000ff,
    }
    //// 带参数 元组枚举
    //enum IpAddr {
    //    Ipv4(u8,u8,u8,u8),
    //    Ipv6(usize,usize,usize,usize),
    //    IpNaN,
    //}
    //// 元组枚举
    //enum Options {
    //    Ok(String),
    //    Err(io::Error),
    //}

    println!("-----------------------");
    println!("typeof={},u={:?}",type_of_name(&u),u);
    println!("typeof={},p={:?},p.0={},p.1={}",type_of_name(&p),p,p.0,p.1);

    println!("typeof={},red=0x{red:0>width$X},greed=0x{green:0>width$X},blue=0x{blue:0>width$X}",type_of_name(&Color::Red),red=(Color::Red as usize),blue=(Color::Blue as usize),green=(Color::Green as usize),width=7);
    // 按位异或与赋值
    println!("typeof={},zero_not2={}",type_of_name(&zero_not2),zero_not2);
    zero_not2 ^=xor;
    println!("typeof={},after xor {},zero_not2={}",type_of_name(&zero_not2),xor,zero_not2);
    println!("typeof={},zero_not={}",type_of_name(&zero_not),zero_not);
    println!("typeof={},bit={}",type_of_name(&bit),bit);
    println!("typeof={},char={}",type_of_name(&ch),ch);
    println!("typeof={},i128={}",type_of_name(&i128),i128);
    println!("typeof={},us={}",type_of_name(&us),us);
    println!("typeof={},u32={}",type_of_name(&u32),u32);
    println!("typeof={},u64={}",type_of_name(&u64),u64);
    println!("typeof={},u128={}",type_of_name(&u128),u128);
    println!("typeof={},yes={}",type_of_name(&yes),yes);
    println!("typeof={},no={}",type_of_name(&no),no);
    println!("typeof={},null={:?}",type_of_name(&null),null);
    println!("typeof={},str={}",type_of_name(&str),str);
    println!("typeof={},num={}",type_of_name(&num),num);
    println!("typeof={},i={}",type_of_name(&i),i);
    println!("typeof={},x={}",type_of_name(&x),x);
    println!("typeof={},f={}",type_of_name(&f),f);
    println!("typeof={},fx={}",type_of_name(&fx),fx);
    println!("typeof={},arr={:?}",type_of_name(&arr),arr);
    println!("typeof={},str_arr={:?}",type_of_name(&str_arr),str_arr);
    println!("typeof={},tuple={:?}", type_of_name(&tuple), tuple);
    println!("typeof={},struct={:?},{name},{age}",type_of_name(&obj_new),obj_new,name=obj_new.name,age=obj_new.age);
    println!("typeof={},{red:?},{green:?},{blue:?}", type_of_name(&RGB::Red), red= RGB::Red, green= RGB::Green, blue= RGB::Blue);
}

fn type_of_name<T>(_:&T) -> &str {
     type_name::<T>()
}
