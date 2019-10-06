# computor_v2
Project made within the 42 cursus.
https://cdn.intra.42.fr/pdf/pdf/5667/computorv2.en.pdf  
This project is a simple computor capable of computing mathematical operation using complex number and matrix, solve simple polynomial equation.  

## Requirement
This project use the 1.37.0 version of Rust, but younger version of the language should work too.  
To install Rust: https://www.rust-lang.org/tools/install  
To install the 1.37.0 or Stable: https://doc.rust-lang.org/nightly/edition-guide/rust-2018/rustup-for-managing-rust-versions.html  

## Building
`make` should compile the program and put the binary `computor` at the root of the project.

## Simple usage
You can feed the program with mathematical instruction, it will solve them.  

### Matrix
Matrix must respect a specific format to be recognised by the program.  
A matrix content must be inside brackets, lines are separated by semi-colons.  
Each line of a matrix must have the same number of value as its sisters.  
Values in a line are separated by a coma.  
[See this example](####basic-computing).  

### Variables
A variable can only name with alphabetical character and is case insensitive (It will be stored lowercase).  
To assign a variable: `varName = value`.  
The same notation is used to overwrite a variable.  

### Functions
A function can be assigned and overwrited the same way as a variable: `functionName(arg1, arg2) = expression`  
It must have at least one argument name given between parenthesis, multiple arguments are separated by coma.  
Each argument name must be unique for this function.  

### Polynomial equations
Equation are solved if their degree is below or equal to 2 and above 0.  
Simply write an equation with an unknown (not previously set).  
[See this example](####polynomial-equation-solving).

### Memory print
To print a value from memory use `= ?` after a variable or a function name.  
Note that after an assignation and a computing, the result value is printed anyways.  
Finally you can simply send `?` to print all variable and function currently memorised.  

### Examples
#### Basic computing:
```
> 1 + 1
2
> 2 * [[0,i];[1,6]]
[ 0 , 2i ]
[ 2 , 12 ]
> (1 + 2) * 4 + 3 * (12 - 5 + 3)
42
```
#### Variable assignation:
```
> a = 42
42
> a = ?
42
```
#### Function assignation:
```
> f(x) = 3x^2
3*x^2
> f(3i)
- 27
> 12i + f(2)
12 + 12i
```
#### Polynomial equation solving:
x should not be assigned to a variable.  
```
> x^2 + 2x + 1 = 0
Equation of degree 2:
 1 * x^2 + 2 * x + 1 = 0
Delta is null, 1 real solution:
x = - 1
```

## Other informations
### Optionnal flags
The following flags were implemented for debugging purpose, and are not required to use the program.  
- -v or --verbose will print some debug information, such as the token list and the token tree generated.  
- -b or --benchmark will print the execution duration of each part of the program.  

### Unit test
`make test` should execute a few unit test.  
They mainly assert that the mathematical operations with imaginary and rational numbers are valid.
