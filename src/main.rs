use std::io;

fn main() {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    let mut user_int = 0;
    match trimmed.parse::<u32>() {
        Ok(i) => user_int = i,
        Err(..) => println!("this was not an integer: {}", trimmed),
    };

    loop {
        let user_int_to_vec = number_to_vec(user_int);
        println!("{:?}", user_int_to_vec);
        let vec_sum = summ_in_vec(user_int_to_vec);
        println!("{:?}", vec_sum);
        let num_to_check = vec_sum;

        if num_to_check > 9 {
            let user_int_to_vec = number_to_vec(num_to_check);
            println!("{:?}", user_int_to_vec);
            let vec_sum = summ_in_vec(user_int_to_vec);
            println!("{:?}", vec_sum);
            break;
        }
    }
}

fn number_to_vec(n: u32) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn summ_in_vec(v: Vec<u32>) -> u32 {
    let sum_vec = v.iter().sum();
    sum_vec
}
