# rrpn

Rust RPN Calculator

## Installation

```
$ git clone https://github.com/eze-kiel/rrpn.git
$ cd rrpn/
$ cargo build --release
```

## Usage

```
./rrpn [-h]
```

## Commands

```
== Basic operations
+, add     : add the last 2 valutes of the stack
-, sub     : substract the last 2 values of the stack
*, mul     : multiply the last 2 values of the stack
/, div     : divide the last 2 values of the stack
^, pow     : do a power between the 2 values of the stack

== Trigonometry
sin        : calculate the sinus of the last value
cos        : calculate the cosinus of the last value

== Variables
pi         : push pi to the stack
e          : push e to the stack

== Misc
sum        : sum the stack
mean       : calculate the mean value of the stack
swap       : swap the last 2 values of the stack
c, clear   : clear the stack
d, drop    : drop the last value of the stack
q, quit    : quit the program
?, h, help : show this help
```

## License

MIT
