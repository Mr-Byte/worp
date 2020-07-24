# Sub-Macros

Scroll macro documents can contain sub-macros that can only be executed by other macros inside the Scroll macro document.
These are useful for creating macros with multiple interaction steps.  Such as:

```
> This is documentation on my super cool macro!
> Roll an attack for 1d20 + strength modifier

$str_mod := {% global.ability_mods[self.str] %}

{% self.name %} attacks and rolls *{% 1d20 + str_mod %}*!

[Roll Damage](#roll_damage)

== #roll_damage ==
{% self.name %} deals {% 8d6 + str_mod %} slashing damage!
```

This creates a macro that contains a named sub-macro `#roll_damage` that can only be accessed by other macros defined in the document.
As noted all sub-macros are denoted by a header like `== #... ==` where `...` is the name being given to the sub-macro.
This header must appear on a line of its own.

Sub-macros have some special rules:

* They contain all the same sections as other macros, including the documentation section, variables section, and the body section.
* They can access all variables declared by the main macro of the document, but not variables declared by other sub-macros.
