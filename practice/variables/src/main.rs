const STARTING_MISSILES:i32 = 8;
const READY_AMOUNT:i32 =2;

fn main() {
    // let mut missiles:i32 = STARTING_MISSILES;
    // let ready:i32 = READY_AMOUNT;
    let (missiles,ready):(i32,i32)=(STARTING_MISSILES,READY_AMOUNT);
    let _a:i32=3;
    println!("Firing {} of my {} missiles...", ready, missiles);
    //missiles = missiles-ready; //if we use this then we need to mark missiles as mutable "mut"
    println!("{} missiles left", missiles-ready);

}
