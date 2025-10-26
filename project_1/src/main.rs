// Rust program to find the roots of a quadratic equation 
use std::io;

fn main(){
   println!("quadratic equation solver");
    println!("using the formula: ax^2+bx+c=0");

    // input number
    println!("\nplease enter an integer");
  let mut in1 = String::new(); // inside main
  println!("enter the first value 'a' ");
  io::stdin().read_line(&mut in1).expect("please try again");
    let a :f32=in1.trim().parse().expect("please input an integer");

    let mut in2 = String::new();// inside main
    println!("enter the second value 'b' ");
    io::stdin().read_line(&mut in2).expect("please try again");
    let b :f32=in2.trim().parse().expect("please input an integer");

 let mut in3 = String::new();// inside main
 println!("enter the third value 'c' ");
 io::stdin().read_line(&mut in3).expect("please try again");
 let c:f32=in3.trim().parse().expect("please input an integer");

 let discriminant=b*b-4.0*a*c;
 let first_root = (-b+discriminant.sqrt())/2.0*a;
 let second_root=(-b-discriminant.sqrt())/2.0*a;
 println!("the roots of the equation {}x^2+{}x+{} are: {} and {}", a, b,c,first_root,second_root );

  // solve discriminant
if discriminant<0.0{
    println!("The discriminant is negative and there are no real roots");
    }else if discriminant>0.0{
        println!("The discriminant is positive and there are two real roots ");
        }else if discriminant==0.0{
            println!("The discriminant is negative and there is exactly one real roots");
           } else{
                println!("the is a mathematical error");
            }
        
    


}

     
 
 
 
  
  


 


  


  

