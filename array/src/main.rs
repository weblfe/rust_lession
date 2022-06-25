use std::mem;
use std::any::type_name;
/// 数组 和 切片
/// \[T;length] , &\[T]
fn main() {
    let arr : [i32; 5] = [0;5];
    let arr_values = &arr[1..4];
    let arr_all = &arr[..];
    let arr_str = ["rust","golang","javascript","java","python"];
    let mut arr_all_values :Vec<i64> = vec![];
    arr_all_values.push(1);
    arr_all_values.push(123);
    println!("values={:?}",arr_all_values);
    println!("typeof={:?}",type_of_name(&arr_all_values));
    println!("slice: arr_all={:?}",arr_all);
    println!("len={},len2={}",arr.len(),arr_values.len());
    println!("{:?}, size={}",arr_values,mem::size_of_val(&arr));
    println!("length={},debug={:?}",arr_str.len(),arr_str);
    for v in arr_str {
        println!("addr={:p},value={}",v,v);
    }

    println!("===========================");
    for v in arr_str.iter() {
        println!("addr={:p},str={},v={}",v,v.to_string(),v);
    }
    let mut i = 0;

    while  i<arr_str.len() {
        println!("i={},v={}",i,arr_str[i]);
        i = i+1;
    }
    let v = arr_str.to_vec();
    println!("arr_str.to_vec,typeof={}",type_of_name(&v));
    for v in v.iter() {
        println!("vec,v={}",v);
    }
    println!("arr[0]={}",arr[0]);
    let start_range = 1..10;

    println!("start_range,start={:?},end={:?},range=({:?})",
             start_range.start,start_range.end,start_range.clone());

    for v in start_range {
        println!("v={}",v)
    }

    let start_end_range = 1..=11;
    println!("start_end_range,start={:?},end={:?},range=({:?}),",
             start_end_range.start(),start_end_range.end(),start_end_range.clone());
    let mut as_str = String::new();
    for v in start_end_range {
        //as_str = as_str+format!("{},",v).as_str();
        if as_str != "" {
            as_str = as_str + ","
        }
        // let s= v.to_string();
        // as_str = as_str + v.to_string().as_str();
        // as_str = as_str + s.as_str();
        // as_str = as_str + &s;
        //as_str = format!("{}{}",as_str,v);
        as_str = format!("{as_str}{v}",as_str=as_str,v=v);
    }

    println!("{}",as_str);
}

fn type_of_name<T>(_:&T) -> &str {
    type_name::<T>()
}