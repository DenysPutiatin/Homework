def is_palindrome(number: int) -> bool:
    number_str = str(number)
    return number_str == number_str[::-1]

# Тестування
if __name__ == "__main__":
    test_numbers = [121, 12321, 123, 45654, 789]
    for num in test_numbers := [121, 123, 1221, 4554, 12345]:
        print(f"{num} - Паліндром? {is_palindrome(num)}")
