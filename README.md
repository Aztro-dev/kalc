# calc
requires gnuplot for graphing

the following functions in the complex plane

``+,-,*,/,^,``

``sin, cos, tan, asin, acos, atan, ``

``sinh, cosh, tanh, asinh, acosh, atanh, ``

``sqrt, cbrt, ln, log(base,num), abs,`` 

``dg(to_degrees),rd(to_radians)``

``re(gets real),im(gets imag)``

constants: ``pi, e``

history file is stored in ``~/.config/.calc_history`` or ``C:\\Users\\%USERNAME%\\AppData\\Roaming\\calc.history``

clear, clears terminal and graph data

# usage
```
calc 1+1
2
```
```
echo 1+1 | calc
2
```
```
calc
>asin(0.5)
π/6
0.5235987755982988
```
```
calc x^2
graphs x^2
```
```
calc x^y
graphs x^y
```
```
calc x^y --nore
displays the amiginary part of x^y
```
```
calc x^y --noim
displays the real part of x^y
```
```
calc
> x=4
> x^2
16
```
```
calc
> x^2
graphs x^2
```
```
calc
> x^y
graphs x^y
```