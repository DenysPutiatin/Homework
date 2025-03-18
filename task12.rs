use rand::Rng;

// 1. Функція підрахунку кількості переміщень вантажу
fn count_permutation(shipments: &[u32]) -> Option<usize> {
    let total: u32 = shipments.iter().sum();
    let ships_count = shipments.len() as u32;

    if total % ships_count != 0 {
        return None; // Неможливо розподілити вантаж однаково
    }

    let target_load = total / ships_count;
    let mut moves = 0;

    for &load in shipments.iter() {
        if load > target_load {
            moves += load - target_load;
        }
    }

    Some(moves as usize)
}

// 2. Генерує вантажі, які можна однаково розподілити
fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();

    // Генеруємо випадковий target load
    let target_load = rng.gen_range(20..50);

    // Кожен корабель отримає target_load
    let mut shipments = vec![target_load; n];

    // Робимо випадкові переноси вантажу між кораблями
    for _ in 0..n {
        let from = rng.gen_range(0..n);
        let to = rng.gen_range(0..n);
        let move_load = rng.gen_range(0..target_load / 2);

        if shipments[from] >= move_load {
            shipments[from] -= move_load;
            shipments[to] += move_load;
        }
    }

    shipments
}

// 3. Функція виводу результатів
fn print_shipments(shipments: &[u32]) {
    println!("Поточний вантаж на кораблях: {:?}", shipments);
    match count_permutation(shipments) {
        Some(moves) => println!("Потрібно переміщень: {}", moves),
        None => println!("Неможливо рівномірно розподілити вантаж!"),
    }
}

// Приклади з поясненнями
fn main() {
    // Приклад, коли можливо рівномірно розподілити
    let shipments_possible = vec![10, 20, 30, 40];
    print_shipments(&shipments_possible);
    // Сума = 100, середнє = 25, кількість переміщень: (40-25)+(30-25)=20

    // Приклад, коли неможливо рівномірно розподілити
    let shipments_impossible = vec![10, 20, 25];
    print_shipments(&shipments_impossible);
    // Сума = 55, середнє = 55/3 ≈ 18.33 (не ціле), отже розподіл неможливий

    // Генеруємо випадковий розподілений вантаж
    let random_shipments = gen_shipments(5);
    print_shipments(&random_shipments);
}
