use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 1 {
        return;
    }

    let nflag = args[1] == "-n";

    let start = if nflag {
        2
    } else {
        1
    };

    if start > args.len() {
        return;
    }

    print!("{}", args[start]);

    if !nflag {
        print!("\n");
    }

}
