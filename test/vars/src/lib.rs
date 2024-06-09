pub fn multiply(a:f64,b:f64)->f64{
    return a*b;
}

pub fn checkifeven(a:i32)->bool{
    if a%2 == 0 {
        return true;
    }
    return false;
}

pub fn append_some(s:&mut String){
    s.push_str("some");
}