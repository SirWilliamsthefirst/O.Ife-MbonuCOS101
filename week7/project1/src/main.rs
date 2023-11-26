use std::io;
use std::f64::consts::PI;
fn main() {
    println!("What would you like to calculate. \n 1.Area of a Trapezoid. \n2.Area of a Rhombus \n3. Area of a Parallelogram \n4. Area of a cube \n5. Volume of a cylinder");
    println!();
    println!("Use the numbers to choose an option");

    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Failed to read input");
    let mut response:i32 = response.trim().parse().expect("Invalid Integer");
    response = valid_response_check("Selection", response);
     calculator(response);
}

fn valid_response_check(response_type:&str, response:i32) ->i32 {
    let mut response = response;
    if response_type == "Selection"{
        loop{
            if response < 1 || response > 5{
                println!("The response you've entered is invalid. Pick a number from 1 to 5 to make a selection");
                let mut response_for_function = String::new();
                 io::stdin().read_line(&mut response_for_function).expect("Failed to read input");
                 response = response_for_function.trim().parse().expect("Not a valid integer");
             }else{
                break;
            }

        }

    }
    return response;
}
fn calculator(shape:i32){
    let mut parameter1 = String::new();
    let mut parameter2 = String::new();
    let mut parameter3 = String::new();
    let mut shape_parameters = [&mut parameter1, &mut parameter2, &mut parameter3];

    let mut parameters_int = [0.,0.,0.]; 
    let mut x = 0;
    



    if shape == 1{ 
    let parameter_names = ["height","First base", "Second Base"];

        for x in 0..3{//
        println!("Enter {} of the trapezoid", parameter_names[x]); 
        io::stdin().read_line(&mut shape_parameters[x]).expect("Invalid Input"); 
        parameters_int[x] = shape_parameters[x].trim().parse().expect("Invalid Integer");
    }
    println!( "The area of your trapezoid is {} units squared",parameters_int[0] * (parameters_int[1] + parameters_int[2]) * 0.5);//calculation


    }else if shape == 2{//calculate area of a Rhombus
        let parameter_names = ["First diagonal", "Second Diagonal"];
        for x in 0..2{
        println!("Enter {} of the Rhombus", parameter_names[x]);
        io::stdin().read_line(&mut shape_parameters[x]).expect("Invalid Input");
        parameters_int[x] = shape_parameters[x].trim().parse().expect("Invalid Integer");
    }
    println!( "The area of your rhombus is {} units squared",parameters_int[0] * parameters_int[1] * 0.5);

    }else if shape == 3{//Calculate Area of a Parallelogram

        let parameter_names = ["Base", "altitude"];
        for x in 0..2{
        println!("Enter {} of the parallelogram", parameter_names[x]);
        io::stdin().read_line(&mut shape_parameters[x]).expect("Invalid Input");
        parameters_int[x] = shape_parameters[x].trim().parse().expect("Invalid Integer");
    }
    println!( "The area of your paralleogram is {} units squared",parameters_int[1] * parameters_int[0]);

    }else if shape == 4{//Calculte Surface area of cube
        let parameter_names = ["Length"];
        println!("Enter {} of the cube", parameter_names[x]);
        io::stdin().read_line(&mut shape_parameters[x]).expect("Invalid Input");
        parameters_int[x] = shape_parameters[x].trim().parse().expect("Invalid Integer");
        println!( "The area of your cube is {} units squared",6. * (parameters_int[0] as f64).powf(2.));

    }else if shape ==5{//Calculate the volume of a cylinder
        let parameter_names = ["Radius", "height"];
        for x in 0..2{
        println!("Enter {} of the cylinder", parameter_names[x]);
        io::stdin().read_line(&mut shape_parameters[x]).expect("Invalid Input");
        parameters_int[x] = shape_parameters[x].trim().parse().expect("Invalid Integer");
    }
    println!( "The volume of yout cylinder  is {:.4}(4d.p) units cubed", PI * parameters_int[1] * (parameters_int[0] as f64).powf(2.));
     }
 }