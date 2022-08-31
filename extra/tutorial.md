# Introduction

This is a very quick tutorial that helps you understand all the concepts in HackScript after around 10 minutes to read

# List of Memory

HackScript program interacts with the thing called "List of Memory". All the things happen will effect to this "List of Memory". <br>
List of Memory is just an array stored empty memories (always 10, but if you need more, List of Memory will automatically add another memory).
You must store data into it.

## Memory

Memory is just a place to store some data. <br>

```
Name      Data      Status
0          []        200
```

Usually memory stores data as an array. Meaning if you can push more data into it. <br>
To show the current event in Memory, we have status

## Status

Status is just a number which have (default) 4 values.

```
200 - means "Close"
210 - means "Open"
220 - means "Save"
230 - means "Reset"
```

By default, 200 is the intermediate status, meaning we can change to another status at 200.

# Pointer

To interact to datas and memories, we have Pointers.
Pointer have two types

## Normal pointer

Normal pointer can move. HackScript only has one normal pointer: the curr_memo pointer `$2` (curr_memo pointer points to **first empty memory in the List of Memory**)

## Constant pointer

Constant pointer cannot move. Constant pointer will be the replacement of variables.

## Move a pointer

It's kinda like C. We have to reassign the pointer using this `<-` operator. <br>
E.g: `$2 <- $2 + 1` (move up to the next memory).

# General syntax

Instructions tell HackScript (or computer) to do something. This is the general syntax of Instructions - which is the general syntax of HackScript.

```
<instruction_name> #<para1>, <para2>
e.g: hi #"Hello"
```

In the case there's no parameters, we have to place the # in. E.g `hi ##`<br>

## Basic built-in instructions (methods)

```
! #<status>: change the current memory status in <status> (<status> has to be 200, 210, 220, 230)
$ #<pointer>: put the <pointer> to the launch table
. #<pointer>: end launching the <pointer> in the launch table
pu #<how_to_push>, <data>: push <how_to_push> (in or out) to the pointer on launch table
(if <how_to_push> is 'in', push the <data> in that pointer.
If <how_to_push> is 'out', pull out to the <data>
(<data> can be 'con'(console) or 'trash')).
```

## Sample program using the basic methods ('Hello, World!')

This is the sample List of Memory

```
0   []    200    <- $2
1   []    200
2   []    200
3   []    200
4   []    200
```

Native HackScript doesn't support the print() function. So, we can do that by pushing the data memory to the console.<br>
The "data" here has to push into "somewhere", we can use $2. <br>

So, the steps in:

```
1. Push the "Hello, World!" into $2
2. Pull the data of $2 out to the console.
```

The first line of code in HackScript should be `! #200` (sometimes it matters). <br>
The first step can split into

```
1. Launch the $2 to the launch table
2. Push the "Hello, World!" into $2
3. End launch it.
```

Rewrite to HackScript. <br>

```
$ #$2
  pu #in, "Hello, World!"
. #$2
```

Once running, the List of Memory will look like this. <br>

```
0   ["Hello, World!"]    220  <- $2
1   []    200
2   []    200
3   []    200
4   []    200
```

The second step can split into

```
1. Launch the $2 to the launch table
2. Pull the "Hello, World!" out to the console
3. End launch it.
```

Rewrite to HackScript. <br>

```
$ #$2
  pu #out, con
. #$2
```

But the data cannot be pushed if we don't change the memory status to 200. <br>
So, the whole program will be

```
! #200
$ #$2
  pu #in, "Hello, World!"
. #$2
! #200
$ #$2
  pu #out, con
. #$2
```
# Comment
Comment in HackScript starts with ' character. <br>
E.g: `' This is a comment`

# Variables 
HackScript doesn't have variables, however, you can use constant pointer instead. <br>
To make a constatnt pointer, we can use the set method. E.g 
```
s #$<name_of_the_variable>
```
Now you can use it as a normal pointer: you can launch it, change the status, push the data, etc. <br>
Equivalent HackScript to Python: 
```
Python:         HackScript:
a = 3           s #$a ' Set the constant pointer
                $ #$a ' Launch it
                  pu #in, 3 ' Push into the pointer
                . #$a  ' End launch it.

```

# Data types
There're a few basic data types in HackScript. <br>
1. Number <br>
  Number holds integers or floats.
2. String <br>
  String is an array of characters. E.g `"Hello, World!"`
3. List <br>
  List is an array. You can get the multi-dimensional list, too.
  E.g `{1, 2, 3}`
4. Booleans <br>
  Booleans can be `true` or `false`.
5. Null <br>
  Null is nothing `null`.
6. Placeholder <br>
  Placeholder is a empty box let you place some data into (this is useful, you can read more to get the details).<br>
  Keyword: `phd`.
## Casting
  This is the way you can get casting: `Number <-> String <-> List` <br>
  You can cast by using the `%` method. <br>
  e.g `% #"string", 1 ' change the 1 to "1"` <br>
  We can use `%` to get the type of some value. E.g `%#null, "string" ' will return a string "<string>"` <br>
  
# Get the variable value
By tracing the `value` attribute. E.g `$<name_of_var> -> value`. You have to follow strictly the syntax (even the spaces) <br>
# Change the variable value
We can use the `<-` keyword. E.g. `($a -> value) <- 1 ' assume that $a -> value is a number`<br>
Note: `<-` can only change the current value to other value if **they are in the same data type**. <br>
So, how can we change the current value to other value if **they are not in the same data type**. This is the point where `placeholder` useful. <br>
Placeholder let you place data and change the data even the new data is not in the same of the data. <br> 
This is the sample code of using Placeholder. <br>

```
s #$a
$ #$a
  pu #in, phd
. #$a

' Now, $a -> value is phd
' For instance, we want to push the data 1 into $a, we simply
($a -> value) <- 1
' It can be understand like it. `phd <- 1` (phd let you assign directly without tracing the value attribute)
' We want to change the $a -> value to "Hello, world!"
' Simply just
($a -> value) <- "Hello, world!" ' can be understand as `phd <- "Hello, world!"`
' Get the value of $a will be $a -> value -> value (return "Hello, world!").
```
HackScript supports another Placeholder: total placeholder, `tphd`. `tphd` can be accessed everywhere. <br>

# Operators
Few basic operators. <br>
1. "+": Plus
2. "-": Minus
3. "*": Multiply
4. "/": Divide
5. "=": same thing as "=="
6. "!=", "<", "<=", ">=", ">": same thing to another programming language  <br>

# Check
HackScript doesn't have if-else statements. Instead, check. <br>
Check simply just checks the condition, if that condition is true, do the true block, if that condition is false, do the false block. <br>
Syntax: `check (..condition..): (true: (...do something...) false: (...do something...))`

# Loop
HackScript doesn't have for loop. You can use while loop and do-while loop <br>
Syntax: `while (..condition..): (do: (..do somthing..))` (this is while loop) <br>
Syntax: `do: (..do something..) while (..condition..)`

# Instruction
HackScript doesn't have function, but instead, instruction. <br>
Instruction cannot be returned, it just runs once and that's all. <br>
Syntax: `inst <name_of_instruction>(<para1>, <para2>): (..do something..) ' in case there's no parameters pass it, just left it`
Call an instruction: see [General Syntax](#general-syntax)

# Class-incomplete
Class in HackScript is like a combination of Class and Struct. <br>
Declare a class: 
```
class NameOfClass: (
  cons: (
    ' attribute here, if nothing to write, just write null
  )
  method: (
    ' method here, if nothing to write, just write null
  )
)
' Note that cons: () block must in the class when declare a class
```

Methods is something that the class can do. <br>
Methods don't connect to class, this is the example. <br>

```
class SampleClass: (
  cons: (
    this -> value: 1
    this -> value_two: 2
  )
  method: (
    inst value_say(sampleclass): (
      ' this method doesn't connect to class, so to print the value attribute, you have to put the sampleclass in
      ! #200
      $ #$2
        pu #in, (sampleclass -> value)
      . #$2
      ! #200
      $ #$2
        pu #out, con
      . #$2
    )
  )
)
```
## Call a class
E.g: 
```
SampleClass ## -> value ' get the value attribute in SampleClass
SampleClass ## -> value_say ## ' call the method in SampleClass


' Equivalent to Python:
' SampleClass().value
' SampleClass().value_say() 
```
