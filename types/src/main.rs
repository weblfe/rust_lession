use std::format;
use std::any::type_name;

fn main() {
    /// 类型 primitives
    /// 标量类型（scalar type）
    // 字符
    let ch = 'a';
    // 字符串
    let str  = format!("{} {}","hello","world!");
    // 数字
    let num = 12;
    let i : isize = 123;
    let i128 : i128 = 1228;
    let x : i64  = 990;
    let f : f64 =  100.00;
    let fx : f32 = 1.00;
    let us : usize = 123;
    let u32 : u32 = 122;
    let u64: u64 = 12333;
    let u128 : u128 = 122222;
    // 布尔
    let yes  = true;
    let no  = false;

    /// 复合类型（compound type）
    // 单元类型
    let null = ();
    // 元组
    let tu = (1,"123",true);
    // 数组
    let arr = [1,2,3,4];
    let str_arr = ["123","hee","1222"];
    // 结构体struct
    #[derive(Debug)]
    pub struct Obj {
        age : isize,
        name : &'static str
    }
    let obj_new = Obj{
        age: 5,
        name: "rust"
    };
    // enum 枚举
    #[derive(Debug)]
    enum Color {
        Red,
        Green,
        Blue,
    }

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
    println!("typeof={},tu={:?}",type_of_name(&tu),tu);
    println!("typeof={},struct={:?},{name},{age}",type_of_name(&obj_new),obj_new,name=obj_new.name,age=obj_new.age);
    println!("typeof={},{red:?},{green:?},{blue:?}",type_of_name(&Color::Red),red=Color::Red,green=Color::Green,blue=Color::Blue);
}

fn type_of_name<T>(_:&T) -> &str {
     type_name::<T>()
}