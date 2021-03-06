import random

numbers_array = random.sample(range(100), 10)


def get_a_peek():
    steps = 0
    for i in numbers_array:
        element_index = numbers_array.index(i)
        final_element = len(numbers_array) - 1
        steps += 1
        if element_index == 0:
            if i > numbers_array[element_index + 1]:
                return (i, steps)
        elif element_index == final_element:
            if i > numbers_array[element_index - 1]:
                return (i, steps)
        else:
            if (i > numbers_array[element_index - 1] and i
                    > numbers_array[element_index + 1]):
                return (i, steps)


if __name__ == '__main__':
    (peek, steps) = get_a_peek()
    print('Generated Array: ' + str(numbers_array))
    print(f'Got the peek {peek} in {steps} steps')
