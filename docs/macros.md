<!-- TOC orderedList:true -->

1. [Draft Note](#draft-note)
2. [Introduction](#introduction)
3. [Syntax](#syntax)
    1. [Text Formatting](#text-formatting)
    2. [Macro Comments](#macro-comments)
    3. [Nested Macro Calls](#nested-macro-calls)
    4. [Macro Links](#macro-links)
    5. [Macro Links with Options](#macro-links-with-options)
    6. [Expression Placeholders](#expression-placeholders)
    7. [Variable Placeholders](#variable-placeholders)
4. [Sub-macros](#sub-macros)
5. [Examples](#examples)

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

Macros are written in a special text templating language.  This section contains information on that language's syntax.

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

## Nested Macro Calls

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

## Expression Placeholders
In almost all cases it is necessary to perform text substitutions, such as for dice rolls.  These expressions can be embedded into macros using the syntax

```
{% expression %}
```

Which will evaluate the provided expression at execution time for a macro each time.  Some examples of this are as follows:

```
Damage {{% 3d6 %}}
```

```
Wisdom {{% self.wisdom %}}
```

These are just a limited number of examples.  The expression syntax will be further documented in the future.

## Variable Placeholders

Variables can be declared at the start of a macro or sub-macro, before the body of the macro begins.  Variables declared in a top-level macro are available to that macro and all sub-macros, but variables declared in a sub-macro are only available in that sub-macro.

The syntax is as follows:

```
$var_name := {% some_expression %}}
```

These variables are evaluated at the time the macro is executed, in the order they are declared.  The result of the expression is then stored in the variable for the remainder of the macro execution, including any further interactions that trigger associated sub-macros.

Variables can be referenced by other expressions such as `{% $some_variable + 1 %}` or can be used directly within a macro like `$some_variable` which will print out the contents of the variable as text.

# Sub-macros

Sub-macros can be specified within a parent macro using the following syntax.

```
== #macro_name ==
```

This syntax must be on a line of its own and any text following it, until the next sub-macro definition, is parsed as a part of the sub-macro.

These macros are not executed when the parent macro is executed, but instead are added to the "local macro" listing and can be executed later by some macro-interaction.

# Examples

```
$charisma_mod := {% global.ability_mods[self.charisma] %}

{%self.name%} casts Eldritch Blast!
Attack *{% 1d20 + $charisma_mod %}*

[Roll Damage](#roll_damage)

== #roll_damage ==
{% 1d10 + $charisma_mod %} Force Damage
```