
fn show(a:f32) -> f32 {
    return a+3.4;
}

fn changeit(a:&mut f32){
    *a=*a+3.0;
}

fn main() {
    println!("Hello, world!");
    let mut _a:i32=34;
    _a=45;
    let b=_a;
    println!("value of a: {} b:{}",_a,b);
    let mut arr:[f32;4]=[1.2,3.4,1.4,4.5];
    for num in 0..=3{
        arr[num]=arr[num]+2.0;
        println!("arr-{} {}",num,arr[num]);
    }
    for mut num in arr{
        num = num+1.0;
        println!("arr-{}",num);
    }
    let (b,c):(i32,i32)=(3,6);
    println!("b:{} c:{}",b,c);
    let d:f32 = show(3.4);
    println!("d:{}",d);
    let (e,f):(f64,f64)=(1.0,3.0);
    let g=vars::multiply(e,f);
    println!("g:{}",g);
    println!("is even 64:{}",vars::checkifeven(64));
    println!("is even 61:{}",vars::checkifeven(61));
    let mut h:String="hello".to_string();
    h=h+" eri";
    println!("h:{}",h);
    let i=h.clone();
    println!("h:{} i:{}",h,i);
    vars::append_some(&mut h);
    vars::print_str(h.clone());
    println!("modified h:{} i:{}",h,i);

    // Create a new empty vector of integers
    let mut vec = Vec::new();
    
    // Push elements into the vector
    vec.push(1);
    vec.push(2);
    vec.push(3);
    
    // Print the vector
    println!("{:?}", vec); // Output: [1, 2, 3]

    let s1:String = "abc".to_string();
    let s2 = s1.clone();
    println!("{} {}",s1,s2);

    let mut temp:f32 = 3.4;
    changeit(&mut temp);
    println!("{}",temp); 
}
