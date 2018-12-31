use std::io;

fn wild(line: &mut String) {

    io::stdin().read_line(line)
        .expect("Failed to read line");

}

fn compatible(line: &mut String, not_msg: &str) -> Option<u32> {

    wild(line);

    match line.trim().parse() {

        Ok(line) => Some(line),

        Err(_) => {

            line.clear();
            println!("{}", not_msg);

            None

        }

    }

}

pub fn force(line: &mut String, not_msg: &str) -> Option<u32> {

    return compatible(line, not_msg);

}