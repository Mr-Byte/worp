<!-- TOC orderedList:true -->

1. [Draft Note](#draft-note)
2. [Introduction](#introduction)
3. [Syntax](#syntax)
    1. [Text Formatting](#text-formatting)
    2. [Macro Comments](#macro-comments)
    3. [Sub-macros](#sub-macros)
4. [Nested Macro Calls](#nested-macro-calls)
    1. [Macro Links](#macro-links)
    2. [Macro Links with Options](#macro-links-with-options)
    3. [Substitution Expressions](#substitution-expressions)
5. [Expressions](#expressions)
    1. [Pre-defined Tables](#pre-defined-tables)
    2. [Local Variables](#local-variables)
    3. [Dice Operator](#dice-operator)
6. [Range Operators](#range-operators)
    1. [Functions](#functions)
7. [Examples](#examples)

<!-- /TOC -->

# Draft Note
This is draft documentation and may change.  This exists mostly to start better documenting the ideas for the macro system.

# Introduction

All macros must have a name.  There will be three types of macros.
* Sub-macros (Named macros nested within another macro)
* Token macros (Macros attached to a token that are specific to that token)
* Global macros (Macros in a global namespace)

Execution of a named macro is resolved in the order of Sub-macros, Token macros, Global macros.

# Syntax

Macros will use a Markdown-like syntax.

## Text Formatting

Macros will have the following formatting options
* `*bold*` for **bold** text
* `~italic~` for _italic_ text
* `_underline_` for underlined text
* `-strike through-` for ~~strike through~~ text

## Macro Comments
Comments can be added to macros using the following syntax
```
// This is a comment in a macro
```

## Sub-macros

Sub-macros can be specified within a parent macro using the following syntax.

```
{#name {%
    // Macro body
%}}
```

These macros are not executed when the parent macro is executed, but instead exist as a method of nesting macros within a parent macro that can be executed later.

Sub-macros and variable declarations cannot nested inside sub-macros.

# Nested Macro Calls

Macros can be called by another macro using the syntax

```
// Other macro bits
#macro_name
```

The macro called will be resolved according the named macro evaluation order.

## Macro Links

Macros can contain clickable links to execute other, named macros.

```
[Link Text](#target_macro)
```

This will produce link text that when clicked by the player who executed the original macro, will execute the specified named macro.

The macro called will be resolved according the named macro evaluation order.

## Macro Links with Options
Macros can contain clickable links to execute one of a set of other, named macros.

```
[Link Text](
    "Label": #target_macro,
    "Label 2": #target_macro2
)
```

This will produce link text that when clicked by the player who executed the original macro, will present the player with a list of options.
Selecting one of the options will execute the specified named macro.

The macro called will be resolved according the named macro evaluation order.

## Substitution Expressions
In almost all cases it is necessary to perform text substitutions, such as for dice rolls.  These expressions can be embedded into macros using the syntax

```
{{expression}}
```

Which will evaluate the provided expression at execution time for a macro each time.  Some examples of this are as follows:

```
Damage {{3d6}}
```

```
Wisdom {{self.wisdom}}
```

These are just a limited number of examples.  The expression syntax will be further documented in the future.

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


# Examples

```
{$charisma_mod {global.ability_mods[self.charisma]}}
{{self.name}} casts Eldritch Blast!
Attack *{{1d20 + $charisma_mod}}*

[Roll Damage](#roll_damage)

{#roll_damage [
    {{1d10 + $charisma_mod}} Force Damage
]}
```