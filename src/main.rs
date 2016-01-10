use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 1 {
        return;
    }

    let nflag = args[1] == "-n";

    let skip = if nflag {
        2
    } else {
        1
    };

    let output = args.iter()
                     .skip(skip)
                     .map(ToOwned::to_owned)
                     .collect::<Vec<String>>()
                     .join(" ");
    print!("{}", output);

    if !nflag {
        print!("\n");
    }

}
