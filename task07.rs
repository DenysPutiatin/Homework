def swap_case(text: str) -> str:
    # Для кожного символу, якщо він нижнього регістру – робимо верхнім,
    # якщо верхнього – нижнім; інші символи залишаємо без змін.
    return ''.join(char.upper() if char.islower() else char.lower() if char.isupper() else char for char in text)

def main():
    user_input = input("Введіть текст: ")
    result = swap_case(user_input)
    print("Результат:", result)

if __name__ == '__main__':
    main()
