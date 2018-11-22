# Power Operations

This is an alternative math representation for exponentation.

## Motivation

Off the top of your head:

*Multipication* is to *Division*, as *Exponention* is to *what*?

Children learn addition, subtraction, multiplication, and division in grade school, yet exponentation and logarithms are left over for precalculus students to fully understand.

Multiplication is repeated addition. Exponentation is repeated multiplication.

Yet, exponentation is considered much more complicated than multiplication.

I feel the reason for this, is a poor representation for exponentation and logarithms.

It is inconsistent with all the other math operators, and makes it look quite alien to a grade school student.

## Building a better representation

I feel that consistency with the rest of math operators is extremely important.

Hence, let's start with the basic math operators.

* Addition involves repeat incrementing (Adding 1). For example, 7 + 5 or 5 + 7 means you start with 7, then keep incrementing it 5 times.

* Subtraction is the reverse of addition, hence it involves repeat decrementing. For example, 20 - 7 means you start with 20, then keep decrementing it 7 times.

* Multiplication involves repeat adding. For example, 7 * 5 or 5 * 7 means you add 7 together 5 times.

* Division is the reverse of multiplication, and it can be done with repeat subtraction. For example, 41 / 7 means you start with 41, and keep dividing it by 7 until you run out of numbers to divide. The answer is how many divisions you did.

As a reminder, lets divide 40 by 7 with repeated subtraction:

* 41 - 7 = 34
* 34 - 7 = 27
* 27 - 7 = 20
* 20 - 7 = 13
* 13 - 7 = 6

Since 6 is smaller than 7, we stop subtracting.

In total, we needed to subtract 5 times. Hence, 41 / 7 = about 5.

Alright, we already know, repeat multiplication is exponentation.

Following the pattern, we'd want the reverse of exponentation to be repeated division.

So for example, I have the number 100, and want to keep dividing it by, say 3, until the number becomes less than my divisor.

* 100 / 3 = 33.333...
* 33.333... / 3 = 11.111...
* 11.111... / 3 = 3.703...
* 3.703... / 3 = 1.235...

Since 1.235.. is smaller than 3, we stop divising.

In total, we needed to divide 4 times, so 4 with some remainder is our answer.

In our current math system, how do we represent this calculation?

Well, that would be ln(100) / ln(3)

Why is the reverse of exponentation equal to three math operations?

That makes things confusing. Let's redefine the reverse of exponentation as:

x <sub>v</sub> y = ln(x) / ln(y)

Hence, in our example 100 <sub>v</sub> 3 = about 4
