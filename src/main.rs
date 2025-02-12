use std::{io::{self, Write}, str::FromStr};

const R_ATM: f32 = 0.082057338;
const R_TORR: f32 = 62.363577;

fn main() {
    let p_in: Option<f32> = input("Pressure").ok();
    let v_in: Option<f32> = input("Volume (L)").ok();
    let n_in: Option<f32> = input("Particles (mol)").ok();
    let r_select: usize = input("R (0 for atm, 1 for torr)").expect("expected integer input");
    let (r, unit) = match r_select {
        0 => (R_ATM, "atm"),
        1 => (R_TORR, "torr"),
        invalid => panic!("invalid R selection: {invalid}"),
    };

    let t_in: Option<f32> = input("Temperature (K)").ok();

    println!();

    match (p_in, v_in, n_in, t_in) {
        (None, Some(v), Some(n), Some(t)) => println!("Pressure: {} {unit}", n*r*t/v),
        (Some(p), None, Some(n), Some(t)) => println!("Volume: {} L", n*r*t/p),
        (Some(p), Some(v), None, Some(t)) => println!("Particles: {} mol", p*v/r/t),
        (Some(p), Some(v), Some(n), None) => println!("Temperature: {} K", p*v/n/r),
        (Some(p), None, None, Some(t)) => println!("Density: {} mol/L", p/r/t),
        _ => panic!("Must only be missing one input!"),
    }
}

fn input<T: FromStr>(prompt: &str) -> Result<T, T::Err> {
    print!("{prompt}: ");
    io::stdout().flush().expect("should be able to flush `stdout`");

    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("should be able to read line from `stdin`");
    return ret.trim().parse();
}
