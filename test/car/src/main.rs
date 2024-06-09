#[derive(Clone,Debug)]
struct Car{
    fuel:i32,
    isengineoff:bool,
    noise:String,
}

impl Car{
    //associated function is same as static method in class.
    fn new(fuel:i32,isengineoff:bool,noise:String)->Car{
        return Car{
            fuel:fuel,
            isengineoff:isengineoff,
            noise:noise,
        };
    }
    //normal methods that can be called on Car obj
    fn add_fuel(&mut self,f:i32){
        self.fuel = self.fuel + f;
    }
    fn turn_on_engine(&mut self){
        self.isengineoff=true;
    }
    fn get_fuel(&self)->i32{
        return self.fuel;
    }
    fn print_details(&self){
        println!("fuel-{} engine-{}",self.fuel,self.isengineoff);
    }
}

//traits are like interfaces.
//traits cannot have members it can only have functions
trait Noisy{
    fn get_car_noise(&self)->&str;
}

//inheritance in traits
trait MoreNoisy:Noisy{
    fn get_more_car_noise(&self)->String;
}

impl Noisy for Car{
    fn get_car_noise(&self)->&str{
        return &self.noise;
    }
}

impl MoreNoisy for Car{
    fn get_more_car_noise(&self)->String{
        return format!("{}{}", self.noise, self.fuel);
    }
}
//Type1 = how to create generic function (accepting reference here because otherwise if we accept T instead of &T then it moves maruti obj into this fucntion instead of copying it)
fn print_noise<T:Noisy>(item:&T){
    println!("print_noise:{}",item.get_car_noise());
}
//Type2 = how to create generic function
fn print_more_noise(item:&dyn MoreNoisy){
    println!("print_more_noise1:{}",item.get_car_noise());
    println!("print_more_noise2:{}",item.get_more_car_noise());
}

fn main() {
    let maruti:Car=Car::new(20,false,"bambam".to_string());
    print_noise(&maruti);
    print_more_noise(&maruti);

    let benz:Car=Car::new(40,false,"binbin".to_string());
    print_noise(&benz);
    print_more_noise(&benz);

    println!("maruti-{:?}",maruti);
}

