import sys


""" Basic function for converting binary to decimal """
binary = sys.argv[1]
list_binary = [int(x) for x in str(binary)]


def convert_to_decimal():
    variable = 0
    for i in list_binary:
        variable = 2 * variable + i

    return variable


if __name__ == "__main__":
    r = convert_to_decimal()
    print(str(r))
