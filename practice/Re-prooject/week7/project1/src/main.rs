use std::io;

fn parameters() {
    let mut h = String::new();
    let mut b = String::new();
    let mut l = String::new();

    println!("Value of h:");
    io::stdin().read_line(&mut h).expect("unable to read input");
    let h: f64 = h.trim().parse().expect("failed input");

    println!("Value of b:");
    io::stdin().read_line(&mut b).expect("unable to read input");
    let b: f64 = b.trim().parse().expect("failed input");

    println!("Value of l:");
    io::stdin().read_line(&mut l).expect("unable to read input");
    let l: f64 = l.trim().parse().expect("failed input");

    t(h, b, l);
    r(h, b, l);
    p(h, b, l);
    c(h, b, l);
    cf(h, b, l);
}

fn t(h: f64, b: f64, l: f64) {
    println!("Area of Trapezium (h: height, b: base1, l: base2):");
    let t = (h / 2.0) * (b + l);
    println!("The area is {}", t);
    parameters();
}

fn r(h: f64, b: f64, _l: f64) {
    println!("Area of Rhombus (h: height, b: base):");
    let r = 0.5 * h * b;
    println!("The area is {}", r);
    parameters();
}

fn p(h: f64, b: f64, _l: f64) {
    println!("Area of Parallelogram (h: altitude, b: base):");
    let p = h * b;
    println!("The area is {}", p);
    parameters();
}

fn c(_h: f64, _b: f64, l: f64) {
    println!("Area of Cube (l: length of the side):");
    let c = 6.0 * l.powf(2.0);
    println!("The area is {}", c);
    parameters();
}

fn cf(h: f64, b: f64, _l: f64) {
    println!("Area of Cylinder (h: height, b: base):");
    let cf = (22.0 / 7.0) * h * b.powf(2.0);
    println!("The area is {}", cf);
    parameters();
}

fn main() {
        println!("Which equation would you like to use?
        If area of trapezium: press t
        If area of rhombus: press r
        If area of parallelogram: press p
        If area of cube: press c 
        If area of cylinder: press cf");

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("unable to read input");
        let option = option.trim();

        match option {
            "t" => t(0.0, 0.0, 0.0),
            "r" => r(0.0, 0.0, 0.0),
            "p" => p(0.0, 0.0, 0.0),
            "c" => c(0.0, 0.0, 0.0),
            "cf" => cf(0.0, 0.0, 0.0),
            _ => todo!() 
    }
}
