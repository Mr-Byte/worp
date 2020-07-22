<!-- TOC -->

- [Introduction](#introduction)
- [Expressions](#expressions)
    - [Pre-defined Tables](#pre-defined-tables)
    - [Local Variables](#local-variables)
    - [Dice Operator](#dice-operator)
- [Range Operators](#range-operators)
    - [Functions](#functions)

<!-- /TOC -->
# Introduction
Direct Inline Computation Expressions aka DICE is the scripting language used by Worp and Worp macros.

# Expressions

All expressions must evaluate down to a single value.  Values have one of a set of common types
* Integers (Whole number values like `-1`, `0`, `1`, etc)
* Decimals (Fractional numbers like `-1.1`, `0.0`, `1.0`, etc)
* Booleans (true/false)
* Strings (text)
* Tables
    * String-key tables (collections of values that can be looked up using a string)
        * String-key tables can have their keys accessed like `table_name.key` which will return the resulting value stored in the key
        * Optionally keys can be resolved using other expressions using the syntax `table_name[key]` where `key` is an expression that evaluates to a string
    * Integer-key tables (collections of values that can be looked up using a number)
        * Integer-key tables can have their keys access like `table_name[key]` where the key is either a numeric literal (`-1`, `0`, `1`, etc) or an expression that evaluates to an integer
    * The values contained in a table do not all need to be the same type, only the keys do
* Lists
    * Lists are a sequential listing of values of the same type.  For example `[1, 2, 3, 4]`
    * Lists can be accessed similar to Integer-key tables, with the first element always having an index of `0`

## Pre-defined Tables
There are two special tables

* The `global` table which is a string keyed table that can be used to lookup values globally within a game.  This is most useful for referencing other tables or commonly used values within a game.
* The `self` table which is a string keyed table that references the currently selected token, that can be used to look up values attached to the token.  This is useful for looking up values like ability scores of the token

## Local Variables

Local variables can be declared with the following syntax

```
{$variable_name {expression}}
```

Where `expression` is some value that can be evaluated as an expression.  This will declare a local variable in scope to the macro and all sub-macros that can be referenced in other expressions, for example

```
{{1d20 + $charisma_mod}}
```

## Dice Operator
*Draft syntax, not final*

Dice can be rolled using the `d` operator, which is an infix operator with the highest precedence.  Either side can be an expression that evaluates to an integer, a list of integers, or a list of nested list of integers (which gets flattened into a single list).  Examples include

* `1d20` - Roll a single, 20 sided die
* `2d20` - Roll two 20 sided dice
* `(5+2)d6` - Roll seven 6 sided dice
* `(global.value)d6` - Roll `global.value` worth of 6 sided dice
* `6d(global.value)` - Roll six dice with sides equivalent to the value of `global.value`
* `1d(2=..=6)` - Roll one die that produces a value in the inclusive range of 2 to 6
* `1d[1, 3, 5, 7, 9]` - Roll one die that produces a value from the specified list
* `1d[2=..=4, 8=..=10]` - Roll one die that produces a value in the inclusive ranges 2 to 4 or 8 to 10.

All dice values return a list of all dice rolled as a part of the expression.  Special functions are provided to operate on these lists, such as showing the total sum of the list, keeping the highest N values of the list, etc. and are further documented in the functions section.

# Range Operators
Range operators are used to produce lists containing a range of integer values.
* `n..m` The range of values between n and m, excluding n and m
* `n=..m` The range of values between n and m, including n, excluding m
* `n..=m` The range of values between n and m, excluding n, including m
* `n=..=m` The range values between n and m, including n and m

In all cases `n` must be less than `m`.

## Functions
*Draft, not final*

This section is not yet completed.
