use rand::Rng;

// Генерує рандомний вектор заданої довжини (значення 10..99)
fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..=99)).collect()
}

// Знаходить пару сусідніх елементів із найменшою сумою
fn min_adjacent_sum(data: &[i32]) -> (i32, i32, i32) {
    let mut min_sum = i32::MAX;
    let mut pair = (0, 0);

    for win in data.windows(2) {
        let sum = win[0] + win[1];
        if sum < min_sum {
            min_sum = sum;
            pair = (win[0], win[1]);
        }
    }

    (pair.0, pair.1, min_sum)
}

// Виводить результат в консоль зрозуміло для користувача
fn print_result(data: &[i32], pair: (i32, i32, i32)) {
    println!("Згенерований вектор: {:?}", data);
    println!(
        "Пара з мінімальною сумою: ({}, {}), сума = {}",
        pair.0, pair.1, pair.2
    );
}

fn main() {
    let vec = gen_random_vector(20);
    let pair = min_adjacent_sum(&vec);
    print_result(&vec, pair);
}
