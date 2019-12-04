use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let masses: std::result::Result<Vec<_>, _> =
        fs::read_to_string("input")?
        .split_whitespace()
        .map(|x| x.parse::<u32>())
        .collect();
    let masses = masses?;

    part_1(&masses);
    part_2(&masses);

    Ok(())
}

fn fuel(mass: u32) -> Option<u32> {
    match (mass as i64 / 3) - 2 {
        f if f > 0 => Some(f as u32),
        _ => None
    }
}

fn part_1(masses: &[u32]) {
    let mut total_fuel = 0;
    for &mass in masses {
        total_fuel += fuel(mass).unwrap();
    }
    println!("{}", total_fuel);
}

fn part_2(masses: &[u32]) {
    let mut total_fuel = 0;

    for &mass in masses {
        let mut contribution = 0;
        let mut mass = mass;

        while let Some(f) = fuel(mass) {
            contribution += f;
            mass = f;
        }
        total_fuel += contribution;
    }
    println!("{}", total_fuel);
}


//fn part_2(masses: &[u32]) {
//    let total_fuel: u32 = masses.iter()
//        .map(|&mass| {
//            let mut contribution = 0;
//            let mut mass = mass;
//            loop {
//                match fuel(mass) {
//                    Some(f) => {
//                        contribution += f;
//                        mass = f;
//                    },
//                    None => break
//                };
//            }
//            contribution
//        })
//        .sum();
//    println!("{}", total_fuel);
//}
