use std::mem;
use std::any::type_name;
use rand::thread_rng;
use rand::seq::SliceRandom;

/// 数组 和 切片
/// \[T;length] , &\[T]
fn main() {
    let arr : [i32; 5] = [0;5];
    let arr_values = &arr[1..4];
    let arr_all = &arr[..];
    let mut arr_str = ["rust","golang","javascript","java","python","php","clang","vb"];
    let mut arr_all_values :Vec<i64> = vec![];
    let mut arr_vec = vec![1 as usize;10];
    arr_vec[1] = 0;
    println!("arr_vec={:?}",arr_vec);
    let mut vec_str :Vec<String> = Vec::new();
    vec_str.push("my".to_string());
    vec_str.push(arr_str[arr_str.len()-1].to_string());
    vec_str.push(String::from("rust"));

    arr_all_values.push(1);
    arr_all_values.push(123);
    arr_str.sort();
    println!("vec_str={:?},len={}",vec_str,vec_str.len());
    println!("values={:?}",arr_all_values);
    println!("typeof={:?}",type_of_name(&arr_all_values));
    println!("slice: arr_all={:?}",arr_all);
    println!("len={},len2={}",arr.len(),arr_values.len());
    println!("{:?}, size={}",arr_values,mem::size_of_val(&arr));
    println!("arr_str.length={},arr_str.debug={:?}",arr_str.len(),arr_str);

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
    let  mut num_arr = [1,2,3,4,5];
    //let mut num = [1,2,3,4,5];
    //num[3] = 0;
    //num[4] = 9;
    // Range  start...end (start,end]-> len=end-start 半闭半开区间 <-> 前闭后开区间
    let mut n1 = &num_arr[0..3];

    let n2  = &num_arr[3..5];
    println!("n1.len={},n1={:?}",n1.len(),n1);
    println!("n2.len={},n2={:?}",n2.len(),n2);
    n1 = n2;
    println!("num.len={},num={:?}", num_arr.len(), num_arr);
    println!("n1.len={},n1={:#?}",n1.len(),n2);

    // shadow delete ref for num_arr
    //let n1 = 0;
    //let n2 = 0;
    // 移除变量到匿名变量, 解除引用
    let _ = (n1 , n2);
    num_arr[3] = 10;
    num_arr[4] +=1;
    println!("num.len={},num={:?}", num_arr.len(), num_arr);
    // 数组打乱
    num_arr.shuffle(&mut thread_rng());
    println!("after shuffle,num.len={},num={:?}", num_arr.len(), num_arr);
    // 数组排序
    num_arr.sort();
    println!("after sort,num.len={},num={:?}", num_arr.len(), num_arr);

}

fn type_of_name<T>(_:&T) -> &str {
    type_name::<T>()
}