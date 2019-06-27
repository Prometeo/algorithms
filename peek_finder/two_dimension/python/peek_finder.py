# import random

r1 = [10, 20, 50, 30]
r2 = [56, 83, 90, 60]
r3 = [70, 15, 52, 38]
r4 = [75, 11, 25, 99]
two_dimmensions_array = [r1, r2, r3, r4]


def peak2d(array):
    m = len(array[0])

    middle_column = m // 2

    middle_column_array = [i[middle_column] for i in array]

    index_max = middle_column_array.index(max(middle_column_array))

    if (
        middle_column > 0
        and array[index_max][middle_column] < array[index_max][middle_column - 1]
    ):
        return peak2d(
            [middle_column_array[:middle_column]
                for middle_column_array in array]
        )

    elif (
        middle_column < m - 1
        and array[index_max][middle_column] < array[index_max][middle_column + 1]
    ):
        return peak2d(
            [middle_column_array[middle_column:]
                for middle_column_array in array]
        )

    else:
        return array[index_max][middle_column]


if __name__ == "__main__":
    peak = peak2d(two_dimmensions_array)
    print(peak)
