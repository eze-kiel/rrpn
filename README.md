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
+, add     : add the last 2 valutes of the stack
-, sub     : substract the last 2 values of the stack
*, mul     : multiply the last 2 values of the stack
/, div     : divide the last 2 values of the stack
sum        : sum the stack
%, mod     : modullo on the last 2 values of the stack
mean       : calculate the mean value of the stack
swap       : swap the last 2 values of the stack
c, clear   : clear the stack
d, drop    : drop the last value of the stack
q, quit    : quit the program
?, h, help : show this help
```

## License

MIT
