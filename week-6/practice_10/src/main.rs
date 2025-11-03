fn main() {
    let a = 20.0;
    let b = 30.0;

    if (a > 10.0) && (b > 10.0) {
        println!("true");
    }

    let c = 0.0;
    let d = 30.0;

     if (c > 10.0) || (d > 10.0) {
        println!("true");
    }
    let is_elder = false;

    if !is_elder{
        println!("Not Elder");
    }
}
