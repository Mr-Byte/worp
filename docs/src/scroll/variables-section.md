# Variables Section

The second section of a Scroll macro document is an optional section for declaring variables that can be used in the body section.
If this section is included must either follow the documentation section or by the first section of a document, if no documentation section is included.

If there's no documentation section, the variable section would look like:

```
$str_mod := {% global.ability_mods[self.str] %}
```

If a documentation section is included, the document would look like:

```
> This is documentation on my super cool macro!
> This macro will perform an attack that does 8d6 + Strength Mod damage.

$str_mod := {% global.ability_mods[self.str] %}
```

There's two major components of a variable declaration.  Those are the variable's name and the expression body that's evaluated and assigned to the variable for the remainder of the macro.

Names all start with the `$` character to denote that a variable is being declared (or used) in the document.  The name itself can then either start with `_` or any upper or lower case letter from A to Z.  Then can be proceeded by any number of `_`, letters, or numbers.

To indicate that a variable is being declared and simply not being used, its name must be followed by `:= {% ... %}` to indicate that an expression is being assigned to the variable.  The exact format of the expressions that can be used in place of the `...` is covered in a later chapter on the [Dice language](../dice/index.md).

You can declare zero or more variables in this section.  Those variables declared later can use the results of the previously declared variables, for example:

```
$x = {% 8d6 %}
$y = {% x + 2 %}
$z = {% y * 2 %}
```

## Scopes

A variable scope is defined as the region where a named variable is able to be used.  In a Scroll macro document, any variables declared in the variables section are available in the body section of the macro itself and in the bodies of any [sub-macros](./sub-macros.md).  Any variables declared in a sub-macro are only available to that specific sub-macro and cannot be used outside that sub-macro's definition.