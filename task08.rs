def is_prime(n: int) -> bool:
    """Перевіряє, чи є число n простим."""
    if n < 2:
        return False
    for i in range(2, int(n ** 0.5) + 1):
        if n % i == 0:
            return False
    return True

def main():
    try:
        number = int(input("Введіть число: "))
        if is_prime(number):
            print(f"{number} є простим числом.")
        else:
            print(f"{number} не є простим числом.")
    except ValueError:
        print("Будь ласка, введіть ціле число!")

if __name__ == "__main__":
    main()
