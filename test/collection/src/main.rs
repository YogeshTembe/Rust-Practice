use rand::Rng;
use std::collections::HashMap;
use std::fs::File;

fn divide(x:i32,y:i32)->Option<i32>{
    if y==0 {
        return None;
    }
    return Some(x/y);
}

#[derive(Debug)]
enum color{
    red,
    blue,
    green
}

fn main() {
    // Create a random number generator
    let mut rng = rand::thread_rng();

    //Vector
    let mut rollnos:Vec<i32>=Vec::new();
    rollnos.push(rng.gen_range(1..=100));// Generate a random number between 1 and 100
    rollnos.push(rng.gen_range(1..=10));
    rollnos.push(rng.gen_range(1..=23));

    for num in 0..rollnos.len(){
        println!("Random number: {}", rollnos[num]);
    }
    
    //HashMap
    let mut info:HashMap<i32,String>=HashMap::new();
    info.insert(25,"abc".to_string());
    info.insert(34,"def".to_string());
    let h=info.get(&25);
    match h{
        Some(x)=>println!("found-{}",x),
        None=>println!("not found"),
    }

    //clousure
    let add=|x,y|{x+y};
    println!("add={}",add(3,4));

    //Option enum
    let mut res=divide(4,2);
    match res{
        Some(x)=>println!("some {}",x),
        None=>println!("None"),
    }
    res=divide(3,0);
    match res{
        Some(x)=>println!("some {}",x),
        None=>println!("None"),
    }

    //Result enum
    let file=File::open("foo");
    match file{
        Ok(f)=>println!("file ok"),
        Err(e)=>println!("file error"),
    }

    //Enum
    let ballon:color=color::red;
    println!("ballon {:?}",ballon);
}
