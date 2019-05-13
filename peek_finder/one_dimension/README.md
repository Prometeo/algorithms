# Peek Finder for unidimensional arrays
We will find a peek in an one dimension array.

| a | b | c | d | e | f | g | h | i |
| - | - | - | - | - | - | - | - | - |
| 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |

- Position 2 is a `peak` if and only if b ≥ a and b ≤ c.
- Position 9 is a `peak` if i ≥ h.


## Straightforward Algorithm
Start from left

| 1 | 2 | ..... | n/2 | ..... | n-1 | n |
| - | - | ----- | --- | ----- | --- | - |

Worst case complexity θ(n) -> `if you have to go from 1 to n (all elements)`.


## Divide and Conquer

| 1 | 2 | ..... | n/2 | ..... | n-1 | n |
| - | - | ----- | --- | ----- | --- | - |

- Look at position n/2.
- If a[n/2] < a[n/2 - 1] then only search on the left half.
- Else if a[n/2] < a[n/2 + 1] then only search on the right half.
- Else n/2 position is a `peak`.

T(n)  =  T(n/2)  +  θ(1)

T(n) -> Work algorithm does on input size `n`.
θ(1) -> The comparision on the middle.
