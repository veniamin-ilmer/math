I believe mathematicians focus too much on Sums of functions, and not on recursive functions, so below I try and detail all of the recursive functions I've been able to find.

Read: a ^ b = b<sup>a</a>

## Multiplication Power

Recurse(n = 1 to x, d ^ (c * n)) = (d * (x ^ d - 1) / (d - 1)) ^ c

Example: sqrt(3 * sqrt(3 * sqrt(3 * sqrt(3)))) = ((4 ^ 2 - 1) / (2 * (1 / 2 - 1))) ^ 3 = **(15 / 16) ^ 3**
