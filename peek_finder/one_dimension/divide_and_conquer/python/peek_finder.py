import random

numbers_array = random.sample(range(100), 10)
mid_position = len(numbers_array) // 2


def get_a_peek():
    if numbers_array[mid_position] < numbers_array[mid_position - 1]:
        search_array = numbers_array[:mid_position+1]
    elif numbers_array[mid_position] < numbers_array[mid_position + 1]:
        search_array = numbers_array[mid_position:]
    else:
        return numbers_array[mid_position], 1

    last_element = len(search_array)-1
    for i in range(0, last_element):
        if i == 0:
            if search_array[i] > search_array[i+1]:
                return search_array[i], i+2
        elif i == last_element:
            if search_array[i] > search_array[i-1]:
                return search_array[i], i+2
        else:
            if (search_array[i] > search_array[i-1] and
                    search_array[i] > search_array[i+1]):
                return search_array[i], i+2


if __name__ == '__main__':
    (peek, steps) = get_a_peek()
    print('Generated array: '+str(numbers_array))
    print(f'Got the peek {peek} in {steps} steps')
