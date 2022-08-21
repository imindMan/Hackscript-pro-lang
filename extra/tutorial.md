# Introduction
This is a very quick tutorial that helps you understand all the concepts in HackScript after around 10 minutes to read


# List of Memory
HackScript program interacts with the thing called "List of Memory". All the things happen will effect to this "List of Memory". <br>
List of Memory is just an array stored empty memories (always 10, but if you need more, List of Memory will automatically add another memory). 
You must store data into it.

## Memory
Memory is just a place to store some data. <br>
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

Normal pointer can move. HackScript only has one normal pointer: the curr_memo pointer `$2`

## Constant pointer

Constant pointer cannot move. Constant pointer will be the replacement of variables.

## Move a pointer

It's kinda like C. We have to reassign the pointer using this `<-` operator. <br>
E.g: `$2 <- $2 + 1` (move up to the next memory).

