# Body Section

The third, and only required section, of a Scroll macro document is the body.
The body defines the actual text and actions to be performed by a macro.

It must either start a document or follow the document and variable sections, both of which are optional.
A simple macro with only a body might look like:

```
{% self.name %} attacks and rolls *{% 1d20 %}*!
```

A more complex example using the other document sections would look like:

```
> This is documentation on my super cool macro!
> This macro will perform an attack that does 8d6 + Strength Mod damage.

$str_mod := {% global.ability_mods[self.str] %}

{% self.name %} attacks and rolls *{% 1d20 %}*!
```

Both of these macros would produce an output similar to:

> Player attacks and rolls **19**!

All spacing in the macro body is preserved in the output.  For example:

```
This
    Macro
        Indents
            Each
                Line
```

Would produce an output with each line indented as above.

## Text Formatting

The body section offers some basic text formatting.
Formatting can provide some flavor to your macros and allow you to draw attention to specific information in a macro's output.

The formatters are as follows:

* `*bold*` produces **bold** text
* `~italic~` produces *italic* text
* `_underline_` produces <ins>underline</ins> text
* `-strike through-` produces ~~strike through~~ text

### Format Nesting

These formats can be nested inside of each other.
Like this:

```
*this is all bold ~and this part is italic bold~ while _this is bold underlined_ and this is ~_bold strike through underlined_~*
```

Would produce text like:

> **this is all bold _and this part is italic bold_ while <ins>this is bold underlined</ins> and this is ~~<ins>bold strike through underlined</ins>~~**

## Substitution Expressions

Substitution expressions take on a form similar to the expression body of a variable declaration as seen in the variables section.
They are placeholders for text that will be computed at the time the macro is executed.
An example of this looks like:

```
Roll a d20! {% 1d20 %}
```

Which when run could produce an output similar to:

> Roll a d20! 12

The syntax of the inside of a substitution expression is defined in the chapter on the [Dice](../dice/index.md) language.

### Variable Substitutions

A second form of substitution expression is the direct reference to a variable, such as:

```
$str_mod := {% global.ability_mods[self.str] %}

My strength mod is $str_mod.
```

This would produce an output similar to:

> My strength mod is 5.

### Text Formatting Expressions

Both forms of substitution expressions can be nested inside text formatters, to give their outputs a nice flair, like:

```
You roll *{% 1d20 %}*
```

Would produce an output similar to:

> You roll **20**

## Embedding Other Macros

It is possible to automatically execute and embed the output of macro inside another.
This is done by referencing a macro by name in the body of another macro, using the macro call operator.
An example of this is:

```
This is the calling macro.

#call_other_macro
```

Which, assuming there's another macro name `call_other_macro` available to be executed, could produce the output:

> This is the calling macro.
>
> This is the called macro.

How macro names get resolved for execution is covered in the section on [macro execution](./macro-execution.md).

## Macro Links

Finally, it is possible to create interactive links to other macros that will run the other macro whenever the link is clicked.
There's two forms of this macro.
The first form is a simple link and the other form is a multi-action link.


### Simple Links
Simple links are macro links that when clicked, immediately execute the named macro.
These links take on a form like:

```
[Click Me!](#other_macro)
```

Which would produce the output, containing an interactive link:

> <ins>Click Me!</ins>

Clicking the link would then evaluate and execute the macro named `#other_macro` and produce its output in chat.
This is useful if you want to write macros that can be executed in multiple stages. 
An example of such a macro would be:

```
> This is documentation on my super cool macro!
> This macro will perform an attack that does 8d6 + Strength Mod damage.

$str_mod := {% global.ability_mods[self.str] %}

{% self.name %} attacks and rolls *{% 1d20 %}*!

[Roll Damage](#roll_damage)
```

### Multi-Action Links

The second form of macro links are a form of link, that when clicked, presents the user clicking the link with a set of options for macros to execute, allowing them to select one of the options.
They take on a form like:

```
[Click Me!]("First Option": #first_macro, "Second Option": #second_option)
```

This produces the output:

> <ins>Click Me!</ins>

Clicking on the link produces a context menu with the options `First Option` and `Second Option`.
Clicking one of the options will execute the corresponding macro.
