fn main() {
    let mut number = String::new();
    println!("enter the last number");
     std::io::stdin().read_line(&mut number);

    let my_int = number.trim_end().parse::<u32>().unwrap();
    let  vector = primary_method(my_int);
    println!("{:?}",vector);
}

fn primary_method(x: u32) -> Vec<u32>{
    // create a list 
    let mut v : Vec<u32> = Vec::new();
    for a  in 1..=x{
     if is_primary(a){
        v.push(a) 
        }
    }
    return v;
}

fn is_primary(x : u32) -> bool{
 for i in 2..=x/2{
  if x % i == 0{
  return false;
  } 
 } 
    return true;
}
