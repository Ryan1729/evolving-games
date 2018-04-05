## Evolving a game

### What kind of game do we want?

We want a single player, turn-based discrete-space game, preferably without actor-removal or output-randomness (for example to-hit dice rolls), but which has a randomized setup, aka input-randomness.

Some reasons for this:
* This is a kind of game *I* would like to play!
* Single-player means we can relatively easily automate playing it.
* Turn-based (discrete-time) and discrete-space simplify the representation of possible game states.
    It also ensures that the game isn't only fun if you have superhuman reflexes.
* Discouraging actor-removal implies that the state space needs to be a certain size, but this is mostly just meant as a heuristic to hopefully produce better games.
* Discouraging output-randomness has implications for whether an "easy" source of randomness will be available. The algorithm may generate it's own PRNG but we don't want dice-rolls to be the first choice!
* Requiring input-randomness implies that at minimum a single random seed must be injected into every game, which has further implications for fitness testing.


### Plan

1. Implement some examples of the type of games we're looking for.
    * checkers problem generator ( [examples](http://www.edcollins.com/checkers/index.html) )
    * minesweeper
    * simple, traditional roguelike with deterministic (dice-less) combat.

2. Design an AST/bytecode/rules format (hereafter "the game format") which can express all of the games implemented.
    Consider how easy mutation will be to implement for this.

3. Create a simple random generator for the game format.

4. Create a fitness function for the game format.

5. Create a mutation function for the game format.

6. Run evolution with the above two functions and iterate on them to produce better games.

### Game structure

In addition to the restrictions in the "*What kind of game do we want*" section, it is useful to note  what kind of structure these generated games will take. A game can be thought of as a finite state machine. That is a game (for our purposes) will consist of a set of possible states, a list of  transitions between them, a special member of the states set called the "initial" state, and a non-empty subset of the state set called the "accepting states". Further, there must be a path along the transitions from the initial state to at least one of the accepting states, that is, the game must be winnable. Because of the imposed input randomness requirement these games must also be game generators, that is, they must produce a new one of these games for every (or just most?) seed that can be handed to them.

### Shortcut attempt

Implementing several things and looking for similarities between them seems like a lot of work. 

Thought experiment: what if we just came up with an arbitrary finite state machine representation and let it evolve?
We would need to check that each game is winnable, so the surviving games would probably mostly just have the first state as an accept state. So disallow that.
Then there would probably be a bunch of "press right to win" games. So we would need to ensure the path from the initial state is at least length 8 or something.
Then I suppose, it would just generate random chains with a shape determined by the starting conditions.

What makes a good shape for a game graph? Usually we use game trees and that's because games that return to the same state are boring. But, game software should be able to return to the same state in the menus. That is, you *should* be able to select different menu items and go back to one you selected before, and doing so should not be part of winning the game.
Okay, we can "uncover" a tree structure in any graph by conceptually merging all cycles into a single node. We could use that tree for path-to-accepting-state counting. What happens then?
Another thing we'd probably want to avoid (or just not count?) is path sections without choices.
I suppose we would just get random trees. 

But we don't want just random trees. We want trees that correspond to rules, which the player is shown in advance, so we can predict what a choice will do. If we are incorrect we should be able to look at the rules and see that they were followed.
How do we make those? 

Is it at all practical to start with trees and derive the rules? A rule doesn't really apply to a tree. it applies to a set of trees, and the more trees there are the more we can say about the rule. So probably we should start with the rule instead.

If we can define a space containing many (all?) rules that generate trees then we can search that space with genetic algorithms etc. We can add randomized cycles to the trees for flavour.

Searching "tree generation rules" results in papers with titles like "Rules Generation From the Decision Tree" which upon inspection seem to describe lossy/approximate methods of generating rules, which suggests that we should in fact examine rule generation rather than tree generation. 

So what is a rule for our purposes? For a set of rules, (a "ruleset") to be a valid generation candidate it needs to produce a game tree of a given length, or at least a graph which a tree can be "uncovered" within.

So we need some examples, both of valid and invalid rulesets. We will assume for brevity's sake that we only need a tree length of 3 or more.

<!-- ✔ ✘ -->
#### Example 0
 The empty set of rules. This does not produce a game tree. ✘
 
#### Example 1 
 The ruleset that produces the a tree containing the initial state, a single distinct accept state and a single transition between them. Valid, but too short. ✘

The above example prompts some thought about what form a ruleset takes. A ruleset needs to define the possible states, the transitions between them, an initial state and a non-empty set of accepting states. But beyond that there are no inherent restrictions, beyond those relating to suitability for a particular purpose.
There can also be multiple isomorphic rulesets which produce the same game tree. As well as isomorphic ways of describing rulesets.

For the moment, let's use the following formulation of a ruleset:

* A type defining the set of possible states
* A function which takes a state and user input, and returns the resulting state, (equivalently, a procedure which takes the current state and user input and mutates the state into the next one.)
* An initial state from the set of possible states, (a default for the state type.)
* A predicate that indicates whether a state is an accepting state or not.

This formulation suggests the approach of fixing one or more of these elements and randomizing the others.

The most complicated of these elements in the transition function. So it would be convenient if we could fix that one and produce a bunch of different, interesting games by varying the others.
Unfortunately, the update function is tightly coupled to the state type, so fixing that essentially fixes the state type as well, drastically limiting the game space. I suppose you could have some kind of abstract interface that different state types could implement to add more flexibility, but that sounds like just fixing part of the transition function, which does not (currently) sound very interesting. In practice, some parts of the transition function will be similar across generated things anyway and restricting generator output sounds a lot easier than designing abstract interfaces, if we really want to make things similar.

Given a fixed state type however, we have a lot of freedom with our transition function. We can even ignore parts of the state allowing us to make the same game with different (sufficiently large/accommodating) state, or more interestingly, we can make very different games with the same state!

The initial state and predicate are also tightly bound with the state type, but note they are also bound with the transition function in a curious manner: The initial state and the transition function define which states are possible. An initial state may allow every single possible state to be reachable given an appropriate transition function. But a different initial state with the same transition function could have no way to transition away from the initial state, (much less actually getting to an accept state). 
The initial state and accepting predicate are relatively easy to generate but in return there are fewer possibilities for them. The accepting predicate (aka goal) is very important though so care will need to be taken in generating it, and it will also likely be a focus of iteration. This suggests trying several initial states and measuring their game tree length and either maximizing it or backtracking and trying a different one several times in the generation process.
The predicate function depends heavily on the initial state. Since the game must be winnable, at least one accepting state must be reachable from the initial state. This suggests the predicate should be generated after the initial state and similarly to the initial state backtracking may be useful.

It appears that we might be able to fix the state and then generate a large amount of games with one "uber state" The question then becomes, which state type should that be?

For the purpose of actually playing a game generated this way we will want to augment our rulesets with a function that displays the game state, or equivalently have the transition procedure mutate a framebuffer to do so.
We can make the display purely a function of the state, which given we are developing an "uber state" promises to save us work since we can write the display function for the uber state once and use it on a large amount of games. We can further make it easier on ourselves by choosing a state that is easy to display.

One option would be to use the framebuffer as the game state. This is definitely the easiest option but the resulting game displays would almost certainly be unreadable. However, making the entire game state visible at all times does suggest a certain ease of insight into the state which may be useful in the first version. A simple to implement method to limit the unreadability of the state (though not completely remove it) would be to use only a limited amount of colours, say 8. We can use enums to represent the colours and use a trivial display function which just maps the enums to colours in the framebuffer.

Before devoting time to developing a framebuffer based "uber state" it seems prudent to take a moment to roughly calculate how many games we can expect to generate from such a state.

As previously mentioned, the possibilities for the initial state and predicate are a function of the state type and the transition function. To get relative sizes of different state types' possibility spaces, we can therefore factor out the initial state and predicate additions to the possibility space, and simply compare the amount of possibilities generated by particular state types and the possible transition functions over that state type.

To begin with, let's calculate an upper bound by ignoring any length restrictions.

We will make some further simplifying assumptions as well.
* There are a fixed number of inputs, all of which can be undertaken on any turn.
* Each of these inputs will correspond to a function from the state to a new state.
* We can enumerate the possible states.

We will define some shorthand.
* |A| ≡ the number of actions
* |S| ≡ the number of possible states
* |S -> S| ≡ the number of possible functions from the state to a new state


Given these assumptions since every function corresponding to an action can be chosen independently, we can say that the total will be |S -> S|<sup>|A|</sup>.
|A| is known, so we just need to calculate |S -> S| from known quantities. |S| is known and it is all we need to calculate |S -> S|. 
Without loss of generality, assume |S| = 2, we will call the two possible states 0 and 1. Note that a function from state to state is simply a mapping from a given input stat to an output state. From here we can construct a truth table.

| 0 | 1 |
| - | - |
| 0 | 0 |
| 0 | 1 |
| 1 | 0 |
| 1 | 1 |

(The two columns represent the possible inputs and each of the data rows represents a distinct function.)

Notice there are 4 = 2<sup>2</sup> functions and notice that if there was a additional possible state there would be 3<sup>3</sup> = 27 possible functions.

Therefore |S -> S| ≡ |S|<sup>|S|</sup> and therefore the upper bound on games we can generate using this method (subject to the above assumptions) is (|S|<sup>|S|</sup>)<sup>|A|</sup>.

Using the value of 8 colours from before and assuming we only allow 8 states (we make all of the pixels always the same colour) and we use the 8 NES buttons as input then we can make at most (8<sup>8</sup>)<sup>8</sup> = 16777216<sup>8</sup> = 6277101735386680763835789423207666416102355444464034512896 ≈ 6.28 × 10<sup>57</sup>

This is a vast overestimate. 

As mentioned before, we want to filter out games with uncovered or latent trees  below a given length. The above formula includes games that can never leave their initial state! Whether or not it gives us an easy way to count how many there are, we would like a way to construct rulesets without constructing rulesets which produce insufficiently long trees.

One way to do that would be to start with a ruleset that is above the required length and refuse to add any edges that decrease the length below the threshold. 

For a given state, what is the longest game tree that can be produced? Answer: The game tree with no branches, that is, a line.

If we were to start with that tree and count the length and compare it to our target length, we could perform operations which decreased the length by known amounts and stop doing so before we decrease it too far.

The first tree editing operation that comes to my mind, given we are starting with a line, is to choose a node, call it n1, and then another node above it (closer to the root), call that one n2. Then detach n1 from its parent and reattach it to n2.

While we want the generation algorithm not to produce certain types of trees, we would like it to be able to generate every possible tree that fits our inclusion criteria, if possible. Can we produce every possible tree with repeated applications of this operation? Since, for reasons outlined above, we want to start with the rules that generate the tree, rather than the tree itself, the question we should be asking first is can we generate a transition function or some other change to a transition function which produces a tree with that operation applied?
