def gcd(a, b):
    """Обчислення НСД (greatest common divisor) за алгоритмом Евкліда."""
    while b != 0:
        a, b = b, a % b
    return a

def main():
    # Введення двох чисел користувачем
    a = int(input("Введіть перше число: "))
    b = int(input("Введіть друге число: "))
    
    result = gcd(a, b)
    print("Найбільший спільний дільник:", result)

if __name__ == '__main__':
    main()
