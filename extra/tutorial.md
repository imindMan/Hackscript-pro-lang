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
