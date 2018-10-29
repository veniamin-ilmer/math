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

## Digit Storage Challenge

Although it feel like we just magically got an extra set of information using bijective base 2, there is one crutial piece of information that requires extra memory: The number's digit size.

Let me explain:

Let's say you want to store the number 1 as a byte in memory.

How does it normally get stored?

Well, it would be stored as 00000001.

In our bijective number system, 0 does not normally exist. We only have 1 and 2. Adding in a zero for space padding will add in an extra digit variety, which will increase our matrix economy, which we are trying to avoid!

I have a solution to this problem, but in order to show it, I'm going to need to show how computers currently represent data.

I'll present this from the perspective of x86 computers, since these are most popular, but you can assume this is applicable/relatable to most CPU variants.

Integers can be represented in four different ways (unsigned):

* AL - 8 bit register, containing numbers 0 to 255.
* AX - 16 bit register, containing numbers 0 to 65,535.
* EAX - 32 bit register, containing numbers 0 to 4,294,967,295.
* RAX - 64 bit register, containing numbers 0 to 18,446,744,073,709,551,615.

When saving information to the memory, the programmer has to know in advance, how big the number will be.

For example, if they know the number will be really small, like the number 3 or 10, then they can choose to store it in an 8 bit register.

If they know that the number will be really large, they can store it in a 64 bit register.

There are a set of mathematical operators for each number type.

So in computer opcodes,

    ADD AL, BL
    ADD AX, BX
    ADD EAX, EBX
    ADD RAX, RBX

Each of these are considered different commands, that use alternative math logic gates, even though they are all performing the same mathematical operation.

Essentially, there is a consequence: More varing types of integer types (8 bit, 16 bit, etc), cause an increase of code memory requirements. This drives up the radix economy.

Additionally, since we are restricted to having the digit size as 8 bits, for example, instead of 6 or 7, that drives up the radix economy too.

