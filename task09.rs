def shift_string(s: str, shift: int, direction: str) -> str:
    if not s:
        return s
    shift %= len(s)  # Враховуємо випадки, коли shift > len(s)

    if direction == "left":
        return s[shift:] + s[:shift]
    elif direction == "right":
        return s[-shift:] + s[:-shift]
    else:
        raise ValueError("Напрямок має бути 'left' або 'right'.")

# Тесткейси
if __name__ == '__main__':
    print(shift_string("hello", 2, "left"))    # "llohe"
    print(shift_string("hello", 2, "right"))   # "lohel"
    print(shift_string("abcdef", 8, "left"))   # "cdefab"
    print(shift_string("abcdef", 8, "right"))  # "efabcd"
    print(shift_string("", 2, "left"))         # ""
