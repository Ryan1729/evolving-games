Assuming a single player, turn based, square grid-based game, here's one branch down one of the possible design decision trees:

grid dimensions: let's say 8 by 8.

> Not enough stuff is defined to specify a goal yet.

Can more than one thing occupy a grid square: let's say yes.

Rough player controlled thing to not player controlled thing ratio. Choose one of "1 / n", "n / m; n, m > 1", "n / 1". : let's say "n / 1".


> Let's try picking a goal now. Presumably it must revolve around the 1 non-player controlled thing. It's still a little hard to decide on a goal since we don't know what the thing is/represents. Since all we've defined is being on a grid, not excluding with other things from a square, and there being only one other thing, the goal seems to need to involve making the player controlled thing(s) intersect with either the one thing itself, or otherwise exerting influence upon it through their position. 
>
> Making this decision has such far reaching effects on the very physics of the game world that essentially the game is defined by the goal, (or at least it should be.) The theme so far with the last two choices has been "what choice do most games not make?". So maybe we should go with the choice that fits that pattern? On the other hand, if there is literally no thing that is not player controlled, except for one, then where does the restrictions on what the player can do come from? What makes the game non-trivial? If things prevented one another from being on the same grid square then then many of the player controllable things could respond to the same input but they could only move if they all could move. But we don't have that rule. So our past choices have already restricted us, perhaps far enough that we should try the obvious, or at least, the least restrictive ideas for a while. 
>
> So, with our current restrictions, we need to have the resistance come from somewhere else. Would it even be possible to come up with a source of resistance from the either of the obvious goals of "have any one player controlled thing intersect with the one thing" or "have every single player controlled thing intersect with the one thing"? Since we only have the one other thing, unless we ant to somehow make the player controls or physics the source of the restriction then I suppose we need to have the resistance come from the one thing somehow. Assuming we try to go with the obvious intersection-based type of goal, then we are forced to either have the one thing move in an evasive manner, have it have complicated physics such that it is not intersect-able at all times, or both. Well i suppose it doesn't have to be in an evasive manner, in the sense of "moving as far away from you as possible." it could have its own agenda that you just need to understand and predict in order to move your pieces to the right position. This actually thematically fits rather nicely with the multiple things on a square decision: Maybe you're a bunch of ghosts and there's a being of some sort wandering around and you're trying to influence them/possess them? The influencing from afar also suggests a different theme, where you are abstract concepts or rules or systems or something and, while trying to do something, the one being is being buffeted around by your various manifestations. I think we have enough stuff to describe a goal in abstract terms at least. We will probably want to refine it somewhat later so let's label it "Abstract Goal". 
>
> Games being on grids more or less imply space being important so representing the goal as a location on the grid makes a lot of sense.


Abstract Goal: Influence the one non-player thing, such that it moves into a given location on the grid.


> The next step should probably determine, (or at least constrict,) the behaviour of either the one thing or a non-empty subset of the player's things. But how do we decide which of these should come first? Following this branch down a particular decision tree is being done with the intention of creating a tree which we can have a computer program automatically explore and either produce specifications for games and/or implementations. So from that perspective it seems like we should just pick an aspect  at random and constrict it. To do that though we would want to have a, (hopefully exhaustive,) list of potential game elements that we can simply uniformly choose from. However, what is easiest for a computer program (--mer to write,) is not necessarily what produces the best games.
>
> I think the next step here should be, (if possible,) related to the previous decisions. Although I can imagine game design decision trees which produce good games not following the following rule, it seems like the clearest path forward from here: Given we have chosen "n / 1" as our rough player controlled thing to not player controlled thing ratio, let's focus on the one thing.
>
> I suppose the, (potentially contentious,) heuristic behind this rule would be, when the path forward is not clear, "Further constrain the most constrained of the not entirely constrained things, further, in the hopes that it will constrain the rest of the things". If we figure that "restrictions breed creativity", as has been suggested elsewhere, then this heuristic is basically suggesting "do the easy stuff first" which doesn't sound like a bad idea.
>
> So if we decide we like that heuristic, then we should constrain the one thing next somehow. But what aspect of it should we constrain? Lets try applying the heuristic again! What is the most-but-not-completely constrained, which for brevity's sake we will call the "least unconstrained", aspect of the one thing?

> I guess since nothing is leaping t mind I should try listing aspects of the one thing and go from there.
> * oneness: there is only one of it. Fully constrained.
> * unless we specify otherwise, it *can* be in the same square as a player thing. Unless we change this, fully constrained.
> * it is not player controlled. Fully constrained.
> * it is influenced by the player controlled pieces somehow. Mostly unconstrained.
> * it exists on an 8 by 8 grid (or a subset thereof?). This is fairly unconstrained but adjusting it would adjust things for everyone. Which I suppose is fine? We could also adjust the grid size/shape only for the one thing. The clearest, simplest thing to do here, which also constrains the player pieces, seems to be to have the player pieces determine the grid squares the one thing can use.
>
> Assuming we go with that last one, then can we do so in a way that is more interesting/novel than "chess but you already got rid of all the pieces except their king. Oh and you have pieces that move differently too"? What would need to change to not have it fit that mould?

> Coming back to this after a while, chess against a single Queen, or maybe an other powerful fairy chess piece like the Amazon, actually sounds interesting enough to try.
> I didn't really achieve my stated goal of making a game design decision tree, but I might have achieved my meta-goal of finding an interesting game design seed. I can come back to this decision tree idea depending on how trying to make this n / 1 chess thing works out. In addition I can still use evolutionary algorithms and/or code generation to design this game. I think I will just treat this game idea as a particular game design path to implement in the generator, so we can potentially combine it with other things in future.

> I tried simulating this by arranging chess pieces in a chess program that let's you play against Stockfish (a highly rated chess engine), such that I had a bunch of pieces and the engine had a king and a queen. After that playing around with the idea, I'm less enthused about it. I am currently enthused by the idea of evolving Shenzhen Solitaire variants.