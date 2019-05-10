# Binary and Decimal
This section has algorithms to convert integers from decimal do binary and from binary to decimal.

# decimal to binary
1. Store the remainder when the number by 2 in an array.
2. Divide the result by 2
3. Repeat 1 and 2 while the number is greater than 0.
4. Reverse the array.
5. Convert the array into string and return it.


# binary to decimal
1. Get every digit starting from the left.
2. Set the total equal to zero.
3. Multiply the total by 2 and add the current digit.
4. Update the total with the result of the step 3 operation.
5. Repeat steps 3 and 4 until there are no more digits.
