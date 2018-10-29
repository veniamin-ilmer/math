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

So, for base 2, instead of 0 and 1, the numbers would be 1 and 2.

For base 3, instead of 0, 1, 2, the numbers would be 1, 2, 3.

For base 4, instead of 0, 1, 2, 3, the numbers would be 1, 2, 3, 4.

I'll provide a full example with base 2:

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

Notice how the bijective binary number of digits grow slower than the ordinary number of digits, even though they technically use the same base: 2.

I realized that system has a signficantly better radix economy than any ordinary base!

## Bijective Binary Economy

The way that you'd calculate a radix economy of any base is by multiplying the number of digits, by the num of varieties a digits can be.

For binary, there are only two varieties of digits used: 0 and 1.

For bijective binary, there are also only two varieties of digits used: 1 and 2.

If we count up the number of digits used from the table above, and multiply it by the variety of digits, then divide it by the count of numbers we can calculate the average radix economy.

Average Radix Economy = Sum of Count of Digits / Count of Numbers * Variety of Digits

For ordinary binary, the radix economy = 54 / 16 * 2 = 6.75

For bijective binary, the radix economy = 42 / 16 * 2 = 5.25

That is significantly smaller!

To make a fair comparison, let me wikipedia's [radix economy table](https://en.wikipedia.org/wiki/Radix_economy#Comparing_different_bases):

Base b | Avg E(b, N) from 1 to 6 | Avg E(b, N) from 1 to 43 | Avg E(b, N) from 1 to 182 | Avg E(b, N) from 1 to 5329 | E(b) / E(*e*)
---: | ---: | ---: | ---: | ---: | ---:
1 | 3.5 | 22.0 | 91.5 | 2,665.0 | Infinite
