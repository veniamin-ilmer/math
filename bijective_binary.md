# Radix Economy of Bijective Binary numbers
## Background
### Radix Economy
[Radix Economy](https://en.wikipedia.org/wiki/Radix_economy) is the amount of memory required to store numerical data.

This is calculated with two forms of information:

* The number of digits required to store the number.
* The number of possible values each digit could have.

More digits per number require more memory. More values per digit require more memory.

The question is, what is the most optimal base to have the best balance of number digits vs variety of digits?

Thus far, the best integer base was considered 3. (See the [wikipedia page](https://en.wikipedia.org/wiki/Radix_economy) to understand why.)

As a result, people brought up ideas of building Ternary based digital logic gates that can take advantage of this optimal base. See [Ternary computer](https://en.wikipedia.org/wiki/Ternary_computer).

However, recently I came up with an alternative representation of numbers, that provides a significantly better radix economy than base 3, and doesn't require as much effort to convert.

### Bijective Binary
[Bijective Numeration](https://en.wikipedia.org/wiki/Bijective_numeration) is an alternative representation of numbers, that does not use the number 0 as a digit, and jumps directly to 1.

I'll provide an example with base 2:

Decimal | Ordinary Base 2 | Bijective Base 2
---:| ---:| ---:
1 | 1 | 1
2 | 10 | 2
3 | 11 | 11
4 | 100 | 12
5 | 101 | 21
6 | 110 | 22
7 | 111 | 111
8 | 1000 | 112
9 | 1001 | 121
10 | 1010 | 122
11 | 1011 | 211
12 | 1100 | 212
13 | 1101 | 221
14 | 1110 | 222
15 | 1111 | 1111
16 | 10000 | 1112
