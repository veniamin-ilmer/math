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

Base b | Avg E(b, N)<br> N = 1 to 6 | Avg E(b, N)<br> N = 1 to 43 | Avg E(b, N)<br> N = 1 to 182 | Avg E(b, N)<br> N = 1 to 5329 | E(b) / E(*e*)
---: | ---: | ---: | ---: | ---: | ---:
1 | 3.5 | 22.0 | 91.5 | 2,665.0 | Infinite
2 | 4.7 | 9.3 | 13.3 | 22.9 | 1.0615
*e* | 4.5 | 9.0 | 12.9 | 22.1 | 1.0000
3 | 5.0 | 9.5 | 13.1 | 22.2 | 1.0046
4 | 6.0 | 10.3 | 14.2 | 23.9 | 1.0615
5 | 6.7 | 11.7 | 15.8 | 26.3 | 1.1429
Bijective 2 | 3.3 | 7.6 | 11.4 | 20.9 | 0.9457
Bijective 3 | 4.5 | 8.2 | ... | ... | ...

As you can see, Bijective 2 has a better radix economy than Bijective 3, so implementing this in digital electronics will be easier using the existing base 2 system.

## Technical Challenges

### Zero
Although we have eliminated zeros, computers would still need to represent that number somehow.

I'd recommend 0 to be treated as a special number.

Let zero equal to the maximum number available for the current register.

So for example, if storing numbers in a 4 bit register, here is how the number would be translated:

Electrical Up / Down Bits | Bijective 2 | Decimal Translation
D,D,D,D | 1 | 1
D,D,D,U | 2 | 2
D,D,U,D | 21 | 3
D,D,U,U | 22 | 4
D,U,D,D | 211 | 5
D,U,D,U | 212 | 6
D,U,U,D | 221 | 7
D,U,U,U | 222 | 8
U,D,D,D | 2111 | 9
U,D,D,U | 2112 | 10
U,D,U,D | 2121 | 11
U,D,U,U | 2122 | 12
U,U,D,D | 2211 | 13
U,U,D,U | 2212 | 14
U,U,U,D | 2221 | 15
U,U,U,U | 2222 | 0

### Digit Storage
Although it feel like we just magically got an extra set of information using bijective base 2, there is one crutial piece of information that requires extra memory: The number's digit size.

Let me explain:

Let's say you want to store the number 1 as a byte in memory.
