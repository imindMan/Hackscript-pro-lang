# Variable declaration

Well since I have thought about it I really wanted to design something like this.

`<var_name> = <var_value>`

But wait, we can also do something like this.

`<var_name>, <var_name>, <var_name> = <var_value>, <var_value>, <var_value>`

This is quite like Python. It supports multiple assignments and also multi swapping, which is great.

Another thing that I would like to add is the quick `bookmarkings`. Quite like this.

`$?<number_id><var_name> = <var_value>`

So when we want to refer them quickly, we can use some kind of references like this for quicker syntax.

Readability? What are you talking about?

Confusion? Yeah definitely.

What about global and local variables? Now normally local and global variables in other languages are just completely different. It would be something like this:

```
global scope
{
    local scope
    cannot modify global scope
    {
        even more local scope
        cannot modify global scope and also the previous local scope
    }
}

global scope

This is mainly for safety reasons, and I think this is good practice. Although Hackscript can be broken at some points, this point is pretty not good to violate.

```

Hackscript itself also does scopes. Like literally scope, it's something like this

```
' this is global scope

g?<some_EH> (
    ' this is the local scope, it's different from global scope
)

' this is global scope


```

So basically the concept is still the same: we still preserve the local and global scopes and seperate them differently so that variables cannot mess up the whole program.

What about modification for higher scope variables? Well in older version we have the `tphd` placeholder, which is an incomplete support for handling global variables. `tphd` is accessible in every scope, meaning in every scope, you can change the variable inside the `tphd`.

But in newer versions like this one, this support is not good enough. What we need in something quicker and easier **WITHOUT PASSING ARGUMENTS**. So I decided to do something _quite unsafe_, that is to request to the global scope the variable we want to change and change it.

This method is very unsafe because it gives the function, again, not the "global implementation" for variable but the "global modification". In every cases, this can lead to a very bad situations when functions just break everything. But I'll do it anyway. If you don't want to risk yourself like that, just force it immutable and it will be safe :)

> [!CAUTION]
> The variable in Hackscript is **mutable** by default

Btw here's the syntax

```
' for global constant variable

!#<var_name> = <var_value>
\

' for local constant variable

g(
    !#<var_name> = <var_value>
)

' for global "loosing" variable

#<var_name> = <var_value>

' for local "loosing" variable

g(
    #<var_name> = <var_value>
)

' for modifying a higher-scope variable
<var_name> = <var_value>

g(
    ' normally it would search for the "higher" scope variable. If it cannot find anything like it,
    ' it doesn't throw the error yet, it still searches for more higher scope variable until it found it
    ' so be aware if for some reasons you write multi-scope code (i mean it's not likely but what if)
    ?#<var_name> = <var_value>
)

```

`What does this even mean, "loosing" variables?` Well in some cases you just want to change the variable value if some conditions is passed. By default the loosing variable can be changed on the current scope but not in other scopes. If you want to override or specify more, EH will be for you.
