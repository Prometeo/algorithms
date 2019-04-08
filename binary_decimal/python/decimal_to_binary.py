import sys

""" Basic function to get the binary from a decimal number """

binary_repr = []


def convert_to_binary():
    decimal_number = int(sys.argv[1])
    while decimal_number > 0:
        reminder = decimal_number % 2
        binary_repr.append(reminder)
        decimal_number = decimal_number // 2

    binary_repr.reverse()
    return binary_repr


if __name__ == "__main__":
    result = convert_to_binary()
    result_str = "".join(str(i) for i in result)
    print(result_str)
