# The Plan

The plan here is to design a data format, probably resembling a programming language, that represents a set of rules for a solitaire variant, leaning towards ones like Shenzhen Solitaire. Then the plan is to make a program that generates that data format and then some automated-ish method of evaluating different instances of that format and producing better ones based on the results of the evaluations.

## Designing the language

Since this language, (even if it turns out to have a binary/other non-text representation it's still a language in the Computing Science sense,) will need to be interpreted to produce a playable game, it seems pragmatic to start by producing tools to produce a playable game, then design the language to express the use of those tools. Further, since we know that we are trying to create solitaire variants, simple 2D graphics consisting largely of rectangles will be fine.

Then we should ensure the design the language can capture the rules of a solitaire game in a reasonably short manner, but at the same time we should allow room for representing a sufficient number of distinct games.


