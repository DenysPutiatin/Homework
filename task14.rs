from itertools import permutations

def solve_puzzle():
    letters = ('m', 'u', 'x', 'a', 's', 'l', 'o', 'n')
    count = 0
    solutions = []

    for perm in permutations(range(10), len(letters)):
        mapping = dict(zip(letters, perm))

        # Перевіряємо, щоб перші букви чисел не були нулями
        if mapping['m'] == 0 or mapping['x'] == 0 or mapping['s'] == 0:
            continue

        # Формуємо числа зі знайденими цифрами
        muxa = mapping['m'] * 1000 + mapping['u'] * 100 + mapping['x'] * 10 + mapping['a']
        x_digit = mapping['x']
        a_digit = mapping['a']
        slon = mapping['s'] * 1000 + mapping['l'] * 100 + mapping['o'] * 10 + mapping['n']

        # Перевіряємо основну умову: muxa = x * a = slon
        if x_digit * a_digit == slon and muxa == slon:
            solutions.append(mapping)
            count += 1

    return solutions, count

def print_solution(mapping):
    muxa = mapping['m'] * 1000 + mapping['u'] * 100 + mapping['x'] * 10 + mapping['a']
    x_digit = mapping['x']
    a_digit = mapping['a']
    slon = mapping['s'] * 1000 + mapping['l'] * 100 + mapping['o'] * 10 + mapping['n']

    print(f"  {muxa}")
    print(f"x      {a_digit}")
    print("  ------")
    print(f"   {slon}")

if __name__ == '__main__':
    solutions, count = solve_puzzle()
    
    if count == 0:
        print("Рішень не знайдено.")
    else:
        print(f"Знайдено {count} рішення(-ь):\n")
        for solution in solutions:
            print_solution(solution)
            print()

