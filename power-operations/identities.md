# Properties of *power* and *apower*

After years of use, it's difficult to mentally just redefine exponentiation, so let me go over all of the properties of *power* and *apower* with you:

## Order of operations

1 <sup>^</sup> 2 <sup>^</sup> 3 <sub>v</sub> 4 <sup>^</sup> 5 is read as (((1 <sup>^</sup> 2) <sup>^</sup> 3) <sub>v</sub> 4) <sup>^</sup> 5

(You just go left to right. Treat <sup>^</sup> and <sub>v</sub> the same.

1 + 2 * 3 <sup>^</sup> 5 * 8 <sub>v</sub> 9 / 4 - 10 is read as 1 + (2 * (3 <sup>^</sup> 5) * (8 <sub>v</sub> 9) / 4) - 10

Order of operations:

1. *power* and *apower*
2. *multiplication* and *division*
3. *addition* and *subtraction*

## *Power* Identities

* 2 <sup>^</sup> 5 = 25
* 5 <sup>^</sup> 2 = 32

---

* 3 <sup>^</sup> a = a * a * a
* 2 <sup>^</sup> a = a * a
* 1 <sup>^</sup> a = a
* 0 <sup>^</sup> a = 1
* (-1) <sup>^</sup> a = 1 / a
* (-2) <sup>^</sup> a = 1 / (a * a) = 1 / 2 <sup>^</sup> a

---

* 2 <sup>^</sup> (a + b) = (a + b) * (a + b)
* 3 <sup>^</sup> (a + b) = (a + b) * (a + b) * (a + b)
* 4 <sup>^</sup> (a + b) = (a + b) * (a + b) * (a + b) * (a + b)

---

* a <sup>^</sup> (b * c) = a <sup>^</sup> b * a <sup>^</sup> c
* a <sup>^</sup> (b / c) = a <sup>^</sup> b / a <sup>^</sup> c

---

* (a + b) <sup>^</sup> 2 = a <sup>^</sup> 2 * b <sup>^</sup> 2
* (a - b) <sup>^</sup> 2 = a <sup>^</sup> 2 / b <sup>^</sup> 2

---

* a <sup>^</sup> (b <sup>^</sup> c) = (a * b) <sup>^</sup> c

## *APower* Identities

* 10000 <sub>v</sub> 10 = 4
* 64 <sub>v</sub> 2 = 6

---

* a <sub>v</sub> c + b <sub>v</sub> c = (a * b) <sub>v</sub> c
* a <sub>v</sub> c - b <sub>v</sub> c = (a / b) <sub>v</sub> c

---

* a * b <sub>v</sub> c = a <sup>^</sup> b <sub>v</sub> c
* (1 / a) * b <sub>v</sub> c = b <sub>v</sub> a <sup>^</sup> c

---

* 1 / a <sub>v</sub> b = b <sub>v</sub> a
* a <sub>v</sub> c / b <sub>v</sub> c = a <sub>v</sub> b

## Calculus identities

* (pi * i) <sup>^</sup> e = -1
* (2 * pi * i) <sup>^</sup> e = 1

---

* Derivative (x <sup>^</sup> c) / dx = c <sub>v</sub> e * x <sup>^</sup> c
* Derivative (c <sup>^</sup> x) / dx = c * (c - 1) <sup>^</sup> x
* Derivative (x <sub>v</sub> c) / dx = e <sub>v</sub> c / x
* Derivative (c <sub>v</sub> x) / dx = -c <sub>v</sub> x / x <sup>^</sup> x <sub>v</sub> e

---

* Integral (x <sup>^</sup> c) dx = e <sub>v</sub> c * x <sup>^</sup> c + constant
* Integral (c <sup>^</sup> x) dx = (c + 1) <sup>^</sup> x / (c + 1) + constant
* Integral (x <sub>v</sub> c) dx = x * (x <sub>v</sub> c - e <sub>v</sub> c)
* Integral (c <sub>v</sub> x) dx = c <sub>v</sub> e * li(x)

## Algebra with *power* and *apower*

If x <sup>^</sup> y = z then

* x = z <sub>v</sub> y
* y = (1 / x) <sup>^</sup> z

If x <sub>v</sub> y = z then

* x = z <sup>^</sup> y
* y = (1 / z) <sup>^</sup> x
