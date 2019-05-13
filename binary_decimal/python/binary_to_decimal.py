import sys


""" Basic function for converting binary to decimal """
binary = sys.argv[1]
list_binary = [int(x) for x in str(binary)]


def convert_to_decimal():
    decimal_variable = 0
    for i in list_binary:
        decimal_variable = 2 * decimal_variable + i

    return decimal_variable


if __name__ == "__main__":
    result = convert_to_decimal()
    print(str(result))
