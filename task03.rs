fn main() {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 6;

    let mut envelope = String::new();

    // Формуємо "конверт" у змінній envelope
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            // Кути
            if (i == 0 || i == HEIGHT - 1) && (j == 0 || j == WIDTH - 1) {
                envelope.push('+');
            }
            // Верхня та нижня межа
            else if i == 0 || i == HEIGHT - 1 {
                envelope.push('-');
            }
            // Ліва та права межа
            else if j == 0 || j == WIDTH - 1 {
                envelope.push('|');
            }
            // Діагоналі (умовно позначають "згин" конверта)
            else if j == i {
                envelope.push('\\');
            } 
            else if j == (WIDTH - 1 - i) {
                envelope.push('/');
            }
            // Порожній простір
            else {
                envelope.push(' ');
            }
        }
        // Переходимо на новий рядок
        envelope.push('\n');
    }

    // Використовуємо print! один раз
    print!("{}", envelope);
    // І робимо додатковий перехід на новий рядок через один println!
    println!();
}
