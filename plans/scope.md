# About the scope ideas

For incomplete support and messy organization purpose, Hackscript will serve only one advanced feature: scopes

Unlike the older version v.1.0.0 HSO-Beta, this version will reduce every conditional implementation, loop implementation, and function implementation and comprise it into one only concept called "scope".

So what is a scope? Well, a scope is a marked chunk of code which does something beyond normal procedural flow of the program.

```
This is the flow of the program
|
|
|
| It just flows like this
|
|
|
|g (
    when it meets the scope, it just also flows through the scope
    |
    |
    |
    |
    |
    but with some <EH> settings, this scope can break the normal procedural flow
    and do something useful
)
|
|
|

```

# General scope implementation

## Conditional flow

The EH for scopes is _exclusively only for scopes_, and therefore cannot be used on other purposes.

Normally the conditional flow of the scope will be defined like this:

```
' this is normal flow of conditional statement
' just typical (if)-(else if)-(else) statement
' unlike other scopes, this scope is merged to the global scope, meaning you can use the global variables without
requesting it.
' also all the variables that are initialized during this scope are also still existed in the global scope
' to clear all of them, use the "trash marker" in EH
g<c> ( ' or g<cr> to mark every single variables into trash after passing this scope
    (<condition>) => (

    )
    (<condition>) => (

    )
    (<condition>) => (

    )
    () => ( ' this is the else scope

    )

)
' this is match flow of conditional statement
g<cm> l(h: <var_name>) (
    (<condition>) => (

    )
    ...
    () => (

    )
)

```
