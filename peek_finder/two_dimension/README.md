# Peek Finder for two dimension arrays
We will find a peek in a two dimensions array.

|     |     |     |     |
| --- | --- | --- | --- |
|     |  C  |     |     |
|  B  |  A  |  D  |     |
|     |  E  |     |     |
|     |     |     |     |


- A is a 2D peek if and only if A ≥ B, A ≥ D, A ≥ E

 y
 |
 |
 |
-|---------x

# Steps 
- Pick middle column => j = x/2
- Find global max on column j at (y, j)
- Compare (y, j-1), (y, j), (y, j+1)
- Pick left columns if (y, j-1) > (y,j) The same for the right.
- If (y,j) ≥ (y, j-1), (y, j+1) => (y, j) is a 2D peak.
- Solve the new problem with half the number of columns.
- When you have a single column find the global max

<pre>
T(n,m) = T(n,m/) + Θ(n)  <- max

T(n,m) = Θ(n) .... + Θ(n) = Θ(nlog<sub>2</sub>m) 
|-------------- log<sub>2</sub>m --------------|
</pre>
