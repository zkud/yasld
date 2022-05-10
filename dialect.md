# YASLD Description

## Data Types (name, examples):
* number {1, 2, 3, 1.23, ...}
* bool {true, false}
* range {(range 0 10), (range (20, 30))}

## Flow Operations:
```
(if (<expression with bool result>) (<then expression>) (<else expression>))
```

## Cycles
```
(for <index variable name> (<iterable to use>) (an expression to evaluate))
```

## Variables
### Definition
```
(var <name> <value>)
```
### Modifying
```
(set <name> <new value>)
```

## IO
```
(report <smth to print to console>)
```

## Example program
Program:
```
(var sum_of_squares 0)
(for index (range 0 100) (set sum_of_squares (+ sum_of_squares index)))
(report sum_of_squares)
```
JS compilation result:
```
function* range(start, end) {
  while (start < end) {
    yield start;
    start += 1;
  }
}

let sum_of_squares = 0;
for (index of range(0, 100)) {
  sum_of_squares = sum_of_squares + index;
}
console.log(sum_of_squares);
```

This program should print the sum of all numbers from 0 to 99