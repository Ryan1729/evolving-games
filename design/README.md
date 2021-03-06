# The Plan

The plan here is to design a data format, probably resembling a programming language, that represents a set of rules for a solitaire variant, leaning towards ones like Shenzhen Solitaire. Then the plan is to make a program that generates that data format and then some automated-ish method of evaluating different instances of that format and producing better ones based on the results of the evaluations.

## Designing the language

Since this language, (even if it turns out to have a binary/other non-text representation it's still a language in the Computing Science sense,) will need to be interpreted to produce a playable game, it seems pragmatic to start by producing tools to produce a playable game, then design the language to express the use of those tools. Further, since we know that we are trying to create solitaire variants, simple 2D graphics consisting largely of rectangles will be fine.

Then we should ensure the design the language can capture the rules of a solitaire game in a reasonably short manner, but at the same time we should allow room for representing a sufficient number of distinct games.

I've now re-read [this book chapter](http://gameprogrammingpatterns.com/bytecode.html) and I think a stack-based VM is the way to go. A good design for a stack-based VM for solitaire of any sort does not spring readily to my mind. So, once a again I think the path forward on this project is to do some more research/experimentation by making a Shenzhen Solitaire specific implementation. This time instead of porting the pico-8 implementation, I will start from that ported version and make it work like it's running on a stack-based VM, (I'll basically pretend I'm writing in Forth.) Hopefully that will make the design for a general solitaire VM clear.

__

I've now started porting the ported pico-8 version of shenzhen solitaire to a custom bytecode, and I've gotten far enough that I'm convinced I could finish it. The problem is, if I naively generate this bytecode then the resulting program is very likely to crash, much less represent a playable game. This is a solvable problem, since I can generate within particular restrictions and/or change the bytecode to remove invalid sequences, or add error handling to the implementations. However another problem looms: If I do that, how can I be sure that the possibility space of the resulting generator contains any interesting games besides the original? Before going further and completing the bytecode, it seems like a very good idea to demonstrate that there is at least one distinct, (from Shenzhen Solitaire,) playable game in either the current bytecode space or an extension/completion thereof.

In order to prove a statement we must first have definitions of the words used in it. What would count as a "distinct (from Shenzhen Solitaire), playable game"? Deletions of parts of the rules of Shenzhen Solitaire would not count. Neither would trivial extensions to them, for instance "Shenzhen Solitaire but where the cards go up to 11". In order to count as a distinct game from Shenzhen Solitaire there need to be non-trivial extensions to the rules. Of course, removals or trivial extensions can be made to a generated game, but there still needs to be non-trivial extensions.

A game does not need to be new to be distinct. What other games can be implemented with this bytecode? The obvious games to attempt to implement are some of the more popular solitaire games like Klondike or Spider Solitaire, but other solitaire variants may be good candidates. The other game does not need to be that different from Shenzhen Solitaire, it just needs to be different enough that I am convinced that something novel and interesting probably resides in the span of the two game vectors, in the bytecode space.

__

I've tried generating bytecode and I'm finding it increasingly difficult to make smaller and smaller improvements to the generation. One potential way to make improving the generation easier would be to redesign the bytecode itself towards that purpose. 

That is, I can design a way to express the space of interesting card laying predicates and then select between them.
