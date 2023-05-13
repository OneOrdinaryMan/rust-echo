use std::env;
use std::process;

struct Args {
    value: String,
}

impl Args {
    fn values(value: String) -> Args {
        Args { value }
    }

    fn stdout_print(&self) {
        println!("{}", &self.value);
    }
}

fn main() {
    let value: Vec<String> = env::args().skip(1).collect();

    if value.is_empty() {
        println!("No arguments Passed!");
        process::exit(0);
    }

    let joined_value = value.join(" ").to_string();
    let arg_1 = Args::values(joined_value);
    arg_1.stdout_print();
}
