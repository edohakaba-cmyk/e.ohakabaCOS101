use std::io;

fn main(){
    println!("A PROGRAM TO DETERMINE AN ANNUAL INCENTIVE OF PAU STUDENTS");
    let mut in1 =String::new();
    println!("Enter name of employees: ");
    io::stdin().read_line(&mut in1).expect("incorrect input");

    let mut in2 =String::new();
    println!("Enter age of employees: ");
    io::stdin().read_line(&mut in2).expect("enter correct input");
    let _age:i32=in2.trim().parse().expect("wrong input");

    let mut in3 =String::new();
    println!("Are you experienced or Not\n");
    println!("if your qualified input 'yes' if your not qualified input 'no'\n");
    io::stdin().read_line(&mut in3).expect("enter a valid input");
    let experience = in3.trim().to_lowercase();
    if experience == "yes" {
        println!("you are experienced!");
    } else if experience =="no"{
     println!("you are not experienced");
                } 
                else{ println!("please enter 'yes' or 'no'");
}
if experience =="yes" && _age>=40{
    println!("you incentive is N1_560_000");
}
else if experience== "yes" &&_age <=40 && _age >=30{
    println!("your incentive is N1_480_000");
}

else if experience== "yes" && _age <28 {
    println!("your incentive is N1_300_000");
}

else if experience =="no" {
    println!("your incentive is N100_000");
}

else{
    println!("your are valid to get an incentive.");
} 
 }   
     