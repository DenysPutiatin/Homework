def calculate_area(width: float, height: float) -> float:
    """Рахує площу прямокутника за шириною і висотою."""
    return width * height

def main():
    try:
        width = float(input("Введіть ширину: "))
        height = float(input("Введіть висоту: "))
        area = calculate_area(width, height)
        print(f"Зайнята площа: {area:.2f} квадратних одиниць.")
    except ValueError:
        print("Будь ласка, введіть коректні числові значення!")

if __name__ == "__main__":
    main()
