# Logarithmic Exponentation

Addition and multiplication are quite useful because of their commutative properties. Exponentiation is harder to work with because of its lacking commutativity.

Inspired by the logarithmic product rule:

ln(a) + ln(b) = ln(a * b)

Let's work with a new operation:

ln(a) * ln(b) = ln(a @ b)

"@" just happened to be a convenient symbol available on the keyboard. It can be replaced with another symbol.

This operation has several useful properties. I will go each and compare with multiplication.

#### Communtative property

Multiplication | New
---------------|--------------
a * b = b * a | a @ b = b @ a

#### Associative property

Multiplication | New
---------------|--------------
(a * b) * c = a * (b * c) | (a @ b) @ c = a @ (b @ c)

#### Distributive property

Multiplication | New
---------------|--------------
a * (b + c) = a * b + a * c | a @ (b * c) = a @ b * a @ c
(a + b) * (c + d) = a * c + a * b + b * c + b * d | (a * b) @ (c * d) = a @ c * a @ b * b @ c * b @ d

#### Property of 0 and identities

|Multiplication | New          |
|---------------|--------------|
|x * 0 = 0         | x @ 0 = 0 |
|x * 1 = x         | x @ 1 = 1 |
|                  | x @ e = x |
|x * 2 = x + x     | x @ e ^ 2 = x * x |
|x * 3 = x + x + x | x @ e ^ 3 = x * x * x |
|x * -1 = -x | x @ e ^ -1 = 1 / x |

Notice how @ is not a replacement for exponentation. However it can be used to complement exponentation.

#### Property of Powers

These are particularly interesting because they deviate from multiplication:

Muliplication | New          
--------------|--------------
(x * y) ^ c = x ^ c * y ^ c | (x @ y) ^ c = x ^ c @ y = x @ y ^ c

x ^ a * x ^ b = x ^ (a + b)
