extern crate masuda;

use masuda::pokemon;
use masuda::pokemon::Pokemon;

fn main() {
    let cli_args = std::env::args().collect::<Vec<String>>();
    let pid_str = cli_args
        .get(1)
        .expect("USAGE: pid_calculator <pid> [<tid> <sid>]");

    let pid = u32::from_str_radix(pid_str, 16).unwrap();
    let tid = cli_args.get(2);
    let sid = cli_args.get(3);

    let tid_sid = match (tid, sid) {
        (Some(tid), Some(sid)) => Some((tid.parse::<u16>().unwrap(), sid.parse::<u16>().unwrap())),
        _ => None,
    };

    print_pid_info(pid, tid_sid);
}

fn print_pid_info(pid: u32, tid_sid: Option<(u16, u16)>) {
    let pokemon = Pokemon::new(pid, pokemon::IndividualValues::default());
    println!("nature: {:?}", pokemon.get_nature());
    println!("ability: {:?}", pokemon.get_ability());
    println!("gender (50/50): {:?}", pokemon.get_gender_50_f());

    match tid_sid {
        Some((tid, sid)) => {
            let shiny = pokemon.get_shininess(tid, sid);
            println!("shiny: {}", shiny);
        }
        None => println!("shiny: unknown"),
    }
}
