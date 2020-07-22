<!-- TOC orderedList:true -->

1. [Introduction](#introduction)
    1. [Who Scroll is For](#who-scroll-is-for)
        1. [Game Masters](#game-masters)
        2. [Players](#players)
        3. [Module and Map Designers](#module-and-map-designers)
        4. [Table Top Designers](#table-top-designers)
    2. [The Scroll Documentation](#the-scroll-documentation)
2. [Macro Document Sections](#macro-document-sections)
    1. [Documentation](#documentation)
    2. [Variables](#variables)
        1. [Variable Scopes](#variable-scopes)
    3. [Body](#body)
        1. [Text formatting](#text-formatting)
        2. [Substitution Expressions](#substitution-expressions)
        3. [Calling Other Macros](#calling-other-macros)
        4. [Macro Links](#macro-links)
    4. [Sub-macros](#sub-macros)
3. [Macro Execution](#macro-execution)
    1. [Macro Scopes](#macro-scopes)

<!-- /TOC -->

# Introduction

Worp uses a custom text processing and templating language called Scroll for evaluating and running macros within the table top environment.
It is one of the core components of the Worp Virtual Table Top and acts as a method of automating the various actions performed in a game.
It is targeted towards various groups of individuals interacting with Worp over the course of developing and play a table top game.

## Who Scroll is For

Scroll targets a wide audience of Game Masters, players, module and map designers, and people designing their own table top games and systems.

### Game Masters

Game Masters will find themselves frequently interacting with and possibly writing Scroll macros over the course of a game.
They're made available through various context menus and hot bars associated with the tokens on the field.
Game Masters can also access and run macros that aren't related to any specific token.
Scroll allows Game Masters to quickly execute actions they need to perform throughout the course of a game, such as actions performed by NPCs, monsters, environmental interactions, etc.

### Players
Players will frequently use Scroll macros from their own tokens to perform actions throughout the course of a game.
They're made available through various context menus and hot bars associated with the tokens owned by a player.

### Module and Map Designers
Module and map designers will likely find themselves writing Scroll macros to give their world life and to provide actions for Game Masters and players to interact with.

### Table Top Designers
Table Top Designers will leverage Scroll to model their table top system by using it to encode all the rules and actions of their system as macros.
In this case Scroll will be frequently used to create the basic templates needed for the other groups mentioned above to play a table top system.

## The Scroll Documentation

The Scroll documentation is broken down into various sections, reflecting the individual sections of a Scroll macro.
These sections include the [documentation section](#documentation) used to help macro authors document their macros.
The [variables section](#variables) which is used to predefine common variables used throughout the body of a macro.
The [body](#body) section which contains all the text, text formatting, and substitution expressions used to produce the final output of the macro.
Finally the [sub-macro](#sub-macros) section is a special section for defining named, nested macros within a Scroll macro that are only accessible from inside the Scroll macro document itself.

# Macro Document Sections
## Documentation
## Variables
### Variable Scopes
## Body
### Text formatting
### Substitution Expressions
### Calling Other Macros
### Macro Links
## Sub-macros
# Macro Execution
## Macro Scopes
