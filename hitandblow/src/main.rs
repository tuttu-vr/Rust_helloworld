use std::io;
use rand::Rng;

fn main() {
    println!("Start! Hit and Blow!");
    main_game();
}

const SIZE: usize = 3;

fn swap(arr: &mut [i32], i: usize, j: usize) {
    if i == j {
        return;
    }
    arr[i] ^= arr[j];
    arr[j] ^= arr[i];
    arr[i] ^= arr[j];
}

fn generate_initial() -> [i32; SIZE] {
    let mut base = [0; 10];
    for i in 0..10 {
        base[i] = i as i32;
        let change_idx = rand::thread_rng().gen_range(0, i + 1);
        swap(&mut base, i, change_idx);
    }
    let mut arr = [0; SIZE];
    for i in 0..SIZE {
        arr[i] = base[i];
    }
    // println!("DEBUG: initial={:?}", arr);
    // println!("DEBUG: base={:?}", base);
    arr
}

fn read_input() -> [i32; SIZE] {
    let mut num = String::new();
    loop {
        println!("Please input your guess");
        io::stdin().read_line(&mut num)
            .expect("Failed to read");
        if num.len() != SIZE + 1 {
            println!("Please input 3 numbers.");
            continue;
        }
        let mut num: i32 = match num.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Failed to parse: {}", num);
                continue;
            }
        };
        let mut arr = [0; SIZE];
        for i in 0..SIZE {
            arr[SIZE - 1 - i] = num % 10;
            num /= 10;
        }
        break arr;
    }
}

fn check(arr: &[i32], guess: &[i32]) -> bool {
    let mut hit = 0;
    let mut blow = 0;
    for (i, value) in arr.iter().enumerate() {
        for (j, guess_value) in guess.iter().enumerate() {
            if value != guess_value {
                continue;
            }
            if i == j {
                hit += 1;
            } else {
                blow += 1;
            }
        }
    }
    println!("HIT={} BLOW={}", hit, blow);
    hit == 3
}

fn main_game() {
    let arr = generate_initial();
    let mut turn = 0;
    loop {
        turn += 1;
        let guess = read_input();
        if check(&arr, &guess) {
            break;
        }
    }
    println!("Congratulations! Clear in {} turns.", turn);
}
