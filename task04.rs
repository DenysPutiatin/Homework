fn main() {
    const DIAMOND_HEIGHT: usize = 7; // висота ромба (непарне число)
    const DIAMOND_WIDTH: usize = 7;  // ширина ромба (має бути однаковою з висотою)

    let mut diamond = String::new();
    let center = DIAMOND_HEIGHT / 2;

    // Верхня половина ромба (включаючи центральний рядок)
    for i in 0..=center {
        let stars = 1 + 2 * i;
        let spaces = (DIAMOND_WIDTH - stars) / 2;
        // Додаємо пробіли, потім зірочки, потім перехід на новий рядок
        for _ in 0..spaces {
            diamond.push(' ');
        }
        for _ in 0..stars {
            diamond.push('*');
        }
        diamond.push('\n');
    }
    // Нижня половина ромба
    for i in (0..center).rev() {
        let stars = 1 + 2 * i;
        let spaces = (DIAMOND_WIDTH - stars) / 2;
        for _ in 0..spaces {
            diamond.push(' ');
        }
        for _ in 0..stars {
            diamond.push('*');
        }
        diamond.push('\n');
    }

    // Виводимо весь ромб одним викликом print! та один println! для заверщального переносу
    print!("{}", diamond);
    println!();
}
