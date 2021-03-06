In the first attempt at this project I tried to represent every possible game with certain properties I knew I wanted, then sample from that space. This did not work because the space I was trying to sample was too large. So In this attempt I am trying to narrow the space that the generator is exploring at any one time, but I would still like to keep the space of potential games producible by the program large. So I am planning to have the program choose some aspects of the game every time it generates something. This brings up another point: the previous attempt didn't really "evolve games" in any specific sense. So in this attempt I plan to rate games as they are produced and keep a record of those ratings and use those to influence the probability of choices being made.

One worry is that the decisions I come up with for the program to choose between, (and the order I put them in) will encode my biases. However, if I remain the only one working on this, I don't see much alternative, and at least I would likely enjoy the games? Every aspect of the program should be potentially up for revision, so hopefully anything problematic can be fixed later if need be?

Another issue is that, while I can add a new choice to a list, in order for a game to be produced with that choice, I will need to implement it. Because I expect to come up with most choices for a given decision point at the same time, I expect I will end up adding options which are not implemented and which instead show a message stating that it has not been implemented. Then If I want to see the game that given seed would result in, I would have some more work to do. This may cause me to (perhaps subconsciously) avoid adding choices to the list that are harder to implement. I will have to try to fight against this impulse as much as I can.

Related to the "I have to actually implement it" issue, is the possibility of adding more and more choices but never actually getting to the point of generating a game. To combat this I will start off with the simplest game I can think of and add only the choices for that at first, so the whole system is known to work. The simplest game I can think of is "Find the win button". I considered making a "Walk to the exit" game but that would be slightly more complicated, and it has the possibility of making all the games have an player avatar, which I don't necessarily want. The "Find the win button game" is sufficiently boring that I will be forced to make extremely different kinds of choices so I definitely not be stuck in a rut making games that are similar to it.

___

After trying to come up with a set of goals for a game to have, I found myself coming back to types of goals that involve actor removal, which, it has been suggested, is something to be avoided. These goals also suggest games that if looked at as statements about the real world and/or the way the world should be, are abhorrent. Unfortunately I find myself inside a milieu where those kinds of goals are prevalent. I see two paths before me, ignore those problems with those goals and generate games with them, or try to find specific goals which do not have those problems and use those instead. I think I will try the second path. Even if I try to take the second path though, goals or other game aspects with undesirable characteristics may "sneak in" either through my own biases and imperfect attention or through generalizing out from the existing set of game elements I choose. There is only so much that can be done about my biases and human limitations, (though acknowledging them is helpful,) so I will focus my attention on generalizing out. I feel that this is an acceptable risk, given we honestly only encounter these through exploring all options rather than targeting them specifically. I feel this way because these kind of game elements *can* exist within a wholesome game if they are not taken to extremes and there are other kinds of elements available for recombination.

___

After partially implementing the program choosing from a small set of possibilities, then generating a program which implements it, I have found that when the choices presented is in the form of a number, (grid dimensions for example) the space can still be unnecessarily large if the options are approached naively. So I will now start using only part of the possible space allowed by other constraints. If a game is good, hopefully it is at least somewhat good with a less than optimal grid size. 

___

While this idea of "automatically" creating implementations of (potentially) novel games is interesting, it seems to have a high effort to reward ratio. Also, the first game came out, (as much as I finished it,) more or less exactly as I imagined it, and I wasn't expecting much. I think it might be a better idea to find a game idea that demands implementation to figure out if it is interesting, or to one which is already known to be interesting and develop a set of decision that lead to that game, then go back and fill in the other branches. If we actually make an implementation using this generation method and we fill in the other branches starting at the bottom, then we will have an implementation of the slightly different game almost immediately. And if we maintain a struct representing the decisions taken, if we go down another probably interesting path we can, (depending on the overlap,) save work getting an implementation.

So it appears the next step is to find something that demands to be implemented. As mentioned before there are two clear paths here: 

* make some decisions randomly (or perhaps intentionally) until something looks interesting and hard to imagine.
* find an existing interesting game design and implement that.

I have begun attempting the first of these options in the accompanying [decision_tree.md](./decision_tree.md) file.

___

<!-- Because I started writing down design notes I felt like I should keep documenting things but now I'm finding it somewhat stifling to continue to do so. So I'm putting this paragraph in here so I can feel like I've addressed this in the text so if there is an abrupt stop, at least it was telegraphed. -->

As mentioned in [decision_tree.md](./decision_tree.md), I now want to try evolving Shenzhen Solitaire variants.

This is probably post-hoc rationalization, but it does fit rather well. Shenzhen Solitaire is surprisingly compelling, and it fits with the criteria of single-player and turn-based. It also has low levels of actor removal and (arguably) is on a grid.

In broad strokes, the plan is to use the existing generator change it generate games like Shenzhen Solitaire but with card amounts, types, and relationships varied. Then we will try to come up with some criteria to evaluate these variants on and try to create the best variant. If a variant suggests a largely different game, we might go with that instead of course!

So here's the current plan:
* Make a very simple Shenzhen Solitaire variant ruleset generator say just with a random amount of cards in each suit.
* Have the generator generate the portion of the program that would depend on the amount of card in each suit.
* Write the supporting, non-generated portion: card display, mouse picking etc.
* Once the game works, add more variations to the ruleset generator and start iterating on them.

___

After watching [this video called Scholars Play Spore](https://www.twitch.tv/videos/260386735) <!-- I don't know how long that link will be good for... --> about *Spore* which talked about how great immediate feedback in a tool for creating things is, I'm tempted to (only slightly!) change direction on this project again, to a Shenzhen solitaire-like editor. The idea would be to make some interface for changing the rules which presented information about the resulting game like "% of sampled games winnable", "average moves to complete", etc.

Having another UI would increase the scope of this project considerably however. So maybe a compromise is to calculate that info and feed it to a generator to help it make decisions on the rules itself? I could make different generators that prefer different things to make different types of games more or less probable.

___

I have just returned to this project after going off and porting a pico-8 version of Shenzhen Solitaire to WASM. [See here](https://github.com/Ryan1729/wasm_shenzhen_solitaire). I did this with the intention of feeding what I learned about implementing Shenzhen Solitaire into this project.

So the current plan (revised as a result of this research) is as follows:
* implement Shenzhen Solitaire again in this project, borrowing pieces from the other one. (still generating the code for it.)
* add parameters for the ruleset generation to select from, refactoring as necessary.
* start generating games and consider how to evaluate them, in order to improve the generator's output.

Coming back to this project after a while has made me see the uglyness in having game in one massive file, and not having a clear decoupling between the different game types. So, I want to pull all the different game-specific stuff in their own separate files.

___ 

Coming back to this project after a (different) while, and reading about the The General Video Game AI (GVGAI) competition and the Video Game Description Language (VGDL) it uses, I want to take this project in a different direction, while still holding to the general idea of producing games, and also keeping the Shenzhen Solitaire inspiration. I'm also not seeing a lot of value in the creation of separate executables for the generated games, at least right now. While the macro-heavy code-gen code is fun it takes too long to produce. Creating a language that is parsed by an executable at run time seems like it would be less work and produce results faster. 

___ 

I'm now coming back to this project after successfully creating an interpreted bytecode, but failing to get it a generator to produce playable games using that bytecode, (see [here](https://github.com/Ryan1729/stack-based-wasm-shenzhen-solitaire)). I've concluded that "playability" is a very hard property to encode into a generator. Therefore, it should not be up to the generator to maintain it! If "evolving games" is going to be successful in any form then all, (or at least the vast majority of) points in the generation space need to be playable! This should, (to the extent possible) be true of other properties like "completable" or "fair". However, we still want flexibility, and a large enough space that we can find unexpected things inside it.

Therefore, I've decided to try implementing an extremely flexible game called [Bartog](https://www.pagat.com/eights/bartog.html) at this [repo](https://github.com/Ryan1729/bartog). The rules of the game "evolve" during the course of play. Potentially, implementing that game may be interesting enough on its own that I may never get back to trying to evolve games in a different way, but if not, it should at the very least greatly inform any future directions of this oft-changing, oft-postponed, project.