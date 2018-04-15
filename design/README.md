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

Another question we should answer first is "What kind of transition function produces the line tree?".

Assuming there is already a total ordering to the state possibilities, the line transition function can just return the next state in the order. The tree editing operation would amount to changing the order. This could be implemented by an if statement that checks if the state is n2 and if it have one of the actions jump the state to n1 instead of advancing by one.

We would like a way to represent these kinds of edits that is easier to manipulate than code. For each state we need to be able to answer the question "What state does pressing each of the buttons result in?". We could have an enum like the following:

```rust
enum Transition {
    Next,
    ThisOne(State)
    Terminal,
}
```

And there would be one of these stored for every possible state. 

This would require at maximum branching slightly more than one state's worth of storage (call that "sizeof(S)") per action, per state possibility, so O(sizeof(S) * |A| * |S|) storage. Assuming no restrictions on valid states, |S| is 2<sup>8 * sizeof(S)</sup>, so in those cases the formula becomes O(sizeof(S) * |A| * 2<sup>8 * sizeof(S)</sup>) which in almost all cases is essentially O(2<sup>sizeof(S)</sup>), which is a lot!

Let's say we only want to dedicate at most one gigabyte (which for the purposes of this document is 2<sup>30</sup> bytes :|) to the transition function.

So we can solve the following equation to figure out the maximum size our state can be: 2<sup>30</sup> =  sizeof(S) * |A| * 2<sup>8 * sizeof(S)</sup>


2<sup>30</sup> =  sizeof(S) * |A| * 2<sup>8 * sizeof(S)</sup>

2<sup>30</sup> / (sizeof(S) * |A|) = 2<sup>8 * sizeof(S)</sup>

1 / (sizeof(S) * |A|) = 2<sup>8 * sizeof(S)</sup> / 2<sup>30</sup>

1 / (sizeof(S) * |A|) = 2<sup>(8 * sizeof(S)) - 30</sup>

log2(1 / (sizeof(S) * |A|)) = 8 * sizeof(S) - 30

log2(1) - log2(sizeof(S) * |A|) = 8 * sizeof(S) - 30

0 - log2(sizeof(S) * |A|) = 8 * sizeof(S) - 30

30 = 8 * sizeof(S) + log2(sizeof(S) * |A|)


If we assume |A| is 8 then this becomes

30 = 8 * sizeof(S) + log2(8 * sizeof(S))

And if I plug "30 = 8 * S + log2(8 * S) solve for S" into wolfram alpha it says it's approximately 3.1671 suggesting that if we made the state an entire 32-bit (4 bytes) integer then we would need over a gig to store it!

Let's confirm it to be sure I didn't make an algebra mistake. starting with our formula: 2<sup>30</sup> =  sizeof(S) * |A| * 2<sup>8 * sizeof(S)</sup>

if we drop the left hand side then if this is right when we replace sizeof(S) with 4 and |A| with 8 then we should get more than 2<sup>30</sup>.

4 * 8 * 2<sup>8 * 4</sup>
32 * 2<sup>32</sup>
2<sup>5</sup> * 2<sup>32</sup>
2<sup>37</sup>

Which pretty clearly exceeds a 2<sup>30</sup>.

So we need to find a more compact representation.

It might be more productive to start from at least a single known game and examine how to mutate it and generalize a generation formula from there.

Let's try single player Nim. It's simple and it already has scaling built-in. (Is the scaling a feature or a bug?)

We can also make some simple variations to see how they effect the tree. XOR Nim for example. Or <del>floating</del> fixed point Nim.

We don't need to make the rest of these, we just need to make the transition function, which is traditionally called `update`.

To be clear, the rules of single player Nim I'm imagining are as follows.

The total starts at n. On your turn you can select a number less than or equal to the current total and decrease the total by that amount. When the total reaches 0, the game is over.

I didn't say it would be a fun game!

If we we will assume that n is 255 and that input is another u8 then the code is simply the following:

```rust
fn update(state: &mut u8, input: u8) -> bool {
    if input <= state {
        state -= input;
    }
    
    state == 0
}
```

if we imagine that there is a 2 bit type called `u2` then we can write this nearly identical code.

```rust
fn update(state: &mut u2, input: u2) -> bool {
    if input <= state {
        state -= input;
    }
    
    state == 0
}
```


this tree is small enough that mapping the entire thing is practical.

We can make a truth table (where we write the numbers in binary):

| state | input | result state |
|-------|-------|--------------|
| 0 | 0 | 0 |
| 0 | 1 | 0 |
| 0 | 10 | 0 |
| 0 | 11 | 0 |
| 1 | 0 | 1 |
| 1 | 1 | 0 |
| 1 | 10 | 1 |
| 1 | 11 | 1 |
| 10 | 0 | 10 |
| 10 | 1 | 1 |
| 10 | 10 | 0 |
| 10 | 11 | 10 |
| 11 | 0 | 11 |
| 11 | 1 | 10 |
| 11 | 10 | 1 |
| 11 | 11 | 0 |

We can also make a `.tgf` file

```
0 0
1 1
2 10
3 11
#
0 0 0
0 0 1
0 0 10
0 0 11
1 1 0
1 0 1
1 1 10
1 1 11
2 2 0
2 1 1
2 0 10
2 2 11
3 3 0
3 2 1
3 1 10
3 0 11
```

and render it:

![single_player_nim.png](single_player_nim.PNG?raw=true)

Just by modifying the type, we can produce an infinite family of game trees.

Reminder: this long path originated from this paragraph:

> Thought experiment: what if we just came up with an arbitrary finite state machine representation and let it evolve?
> We would need to check that each game is winnable, so the surviving games would probably mostly just have the first state as an accept state. So disallow that.
> Then there would probably be a bunch of "press right to win" games. So we would need to ensure the path from the initial state is at least length 8 or something.
> Then I suppose, it would just generate random chains with a shape determined by the starting conditions.

So lets say we successfully generated rulesets that produce nicely formed game trees, then what are we expecting to happen? We're hoping that we'll be able to take those games, play them, then figure out why they're not fun, (it's unreasonable to expect the first ones to be fun,) then adjust as necessary to make them more fun.

In that case, since the amount of possible games seems unreasonably plentiful, we don't need to capture every possible well formed game tree in the initial set before we start winnowing down. Even if we run out of games, we can use the knowledge we gain winnowing the smaller set to winnow down the complete set and more quickly get to the parts outside of the original set, and find new games.

So we can just find an update function that represents some game tree which would fit our criteria and start from there.

Let's try starting with a specific tree that fits our criteria assuming a length of 8 and 4 inputs. This tree needs to very simple to describe in code which usually correlates well with being easy to describe in words.

Branches out with all 4 branches to new states each time and counting from the bottom row only, the even-numbered bottom nodes are in the accept set.

After describing that in code, it will hopefully be instructive to modify it to produce a tree of length 9 only in the places that were accept states in the previous one. 

More ideas: some nodes should point to the same nodes one level down. Have one or two accept states and truncate all dead ends to the state after the dooming choice.

This is again feeling like a lot of work. Say I had a fixed asymmetrical tree and I randomized which input did what on each node and pretended that was the full space generator, what would the next step be?

I think the next step would be making the game more comprehensible. The easiest first step would be to display the code while playing. The next step would be making the rules easier to internalize.

The two broad approaches for making the rules easier to internalize I can think of off the top of my head are to restrict the rules to those that are easy to understand, or to produce a new system of signs which are optimized for reading as opposed to code which is meant to facilitate reading and writing. Note these approaches are not mutually exclusive.

Before I take the first approach, I would want to be reasonably sure that none (or at least very few) of the possible rulesets I would be "throwing away" are interesting. If I exclusively took the second approach, given I have a fall-back to the code when something is produced that my set of signs has not accounted for, I would not be forced to "throw away" any possibilities, at the cost of a reduced amount of clarity. Therefore the decision to take the second approach can be postponed until it is deemed necessary.

It would be very easy to simply generate |A| manipulations of the state and make an update function that merely switched on the input, but that would be throwing away the possibility of context sensitivity.

One way to include context sensitivity would be to, instead of generating updates predicated only on the input, if I also generate random state predicates (I presumably need to do this for the winning condition anyway) and combine them together like so:

```rust
if state_pred1(state) && input.was_pressed(Button::A) {
    //mutate the state
}
```

This presents the potential problem that all of the state predicates could be false. We could work around this by storing all of the state predicate results and having a mutation happen if they are all false.

The question now becomes, what, if any, update functions can this method *not* produce?

If we only allow one if statement per input type, then it couldn't produce something equivalent to the following:

```rust
if input.was_pressed(Button::A) {
    if state_pred1(state) {
        //mutate the state
    } else {
        //mutate the state in a different way.
    }
}
```

The simple solution to that is to generate that form since it can express the previous scheme as well! (assuming we ether allow an empty else block or there are part of the state which are not part of any predicate and thus do not affect the outcome of the game.)

But if we generate that form then we cannot represent things like this:

```rust
if input.was_pressed(Button::A) {
    if state_pred1(state) {
        //mutate the state
    } else if state_pred2(state) {
        //mutate the state in a different way.
    } else {
        //mutate the state in another different way.
    }
}
```

Given we generate a variable number of elses, and that we cover the entire space of possible state predicates, then I think we can represent any transition graph (with a maximum out-degree of |A|,) this way. The maximum amount of elses should probably be a generator parameter. We can call it |else| in case we need to refer to it later.

The best game testing experience would be clicking a button and instantly having a new game to play. Unfortunately generating and evaluating rust at runtime is difficult, so instead the best path forward seems to be generating rust files and saving them where the watcher will see them and recompile. Optionally we might want to keep at least the last few generated files in case we decide to go back to one after pressing the generate button. It might be easier to just move whatever file was there before to a `.gitignore`d folder. Since there does not currently seem to be a nice way to specify an AST and produce a rust file from it, it looks like we'll have to roll our own. Our implementation can be specialized to our needs though, so it won't have to support anything more complicated than if statements and state predicates.


Returning to this set of notes after beginning an implementation, I'm now wondering what a state mutation should consist of. That is, what is a datatype that can represent all possible mutations, which is easier to select a random instance of, than a string of code.
Given a state which consists of a series of n sections of the same type that can be in one of k states, we can just pick a random subset of the sections and for each one, choose one of the possible section to section functions and apply it to that section.

After implementing the "pick a random subset and assign a transformation function to each element of it" scheme above. 

It works but it produces transition graphs with insufficiently long latent trees. One way to increase the length of the generated trees is to impose an order on the colours and only allow increasing colours and not decreasing them. Does imposing an order on the colours like this restrict the space of games we can represent? If it does is that acceptable? What if we only partially order the colours (that is make some of them the same rank,) does that restrict the space less?

A better idea than imposing an order on the colours is imposing an order on the states themselves, since that is what we do not want to repeat. Imagine the state being treated as a large base |C| number (where |C| is the number of colours), and the actions only increase the state number. We're fine with that even though particular sections may have the same values over and over again. 

Is there anything we would also like to be potentially generated that does not fit the base |C| number mould? We might want some cycles in our graph which still allows us to uncover a latent tree. So we could choose a subset of the state and only impose an order on that part, while still changing other parts. I think any ordering would be isomorphic to a base |C| number, given we disregard what would appear on the screen. Probably we would want what on screen to be only slightly different when there is a small difference in the state's numerical interpretation, so we should probably use something like a Gray code for that. Perusing [the Gray Code Wikipedia article](https://en.wikipedia.org/wiki/Gray_code) suggests that we want to use an n-ary Gray code, possibly a balanced n-ary Gray code at that. Actually, I'm not sure that we care about whether exactly one digit changes between adjacent states. I think we might want instead for small changes to produce small changes in the output and large changes to produce large changes in the output. So the distance between 0 and the maximum state should involve changing all the sections, and adding n to the state should change some number of sections, call it s<sub>n</sub>, which should be >= s<sub>n - 1</sub> and <= s<sub>n + 1</sub>. Choosing a numbering system like this may be important to making the state more comprehensible, but we can postpone implementing this until later.

If we go ahead with this base |C| number idea, we would like every possible operation on numbers which results in an increasing value to be potentially generated. For a given state, the set of all possible operations which result in an increasing value can be represented as the set of all possible results of those operations. It doesn't matter if the operation form 2 to 4 was doubling or adding 2. The result is the same. We could just add a random amount to the current amount with saturating arithmetic semantics. That would allow us to have a consistent rule without a special predicate for each case, (just one predicate for every case). Would that give a skewed distribution as the game went on, (the end of the game coming at a weird time)? And is that something we should deal with afterwards or a reason to do something altogether different that saturating semantics?

I think we could skew things towards smaller increments as we get closer to the end of the state space if we want to, but no matter what, if we make the states not repeat, we will eventually run out. We could also reserve part of the state space for only when a certain predicate is true or something too. So we can adjust things late if we need to so it seems to make sense to go ahead and interpret the state as a number and have each state mutation and add a random number to it.

Now having implemented it, I notice that it does certainly only produces trees, and they are at least 8 nodes deep. The thing is, that even restricting the number of possible state sections to the width of the screen, the trees are *too* deep! Given we use 8 colours and use 240 sections, there are 8<sup>240</sup> ≈ 5.51 × 10<sup>216</sup> possible states. So even if we add billions to the state with button press we would still have trees which are too deep to reach the end of in a human's lifetime. 

We can, of course simply add large enough numbers to shorten the tree down, but this is essentially identical to using even less sections of the state in almost all cases. How much state should we use? In this case since we are displaying all and only the game state we need no extra memory for graphics so if we want to try to compare the state we use to a directly human authored game then we should only count the state related to the game state.

A RAM map of Super Mario Bros is available [here](https://datacrystal.romhacking.net/wiki/Super_Mario_Bros.:RAM_map) under the terms of the [GNU FDL](https://www.gnu.org/copyleft/fdl.html). I would reproduce it here and highlight which portions I consider to be game state but the aforementioned license is too burdensome for that. Instead, I will merely list the addresses I consider game state and the total number of bytes in the following table:

|Address section|
|0x0000|
|0x0002|
|0x0003|
|0x0005|
|0x000E|
|0x000F-0x0013|
|0x0014|
|0x0016/A|
|0x001B|
|0x001D|
|0x001E|
|0x0023|
|0x0024/5|
|0x002A-0x0032|
|0x0030/2|
|0x0033|
|0x0039|
|0x0045|
|0x0046/A|
|0x004B|
|0x0057|
|0x0058/C|
|0x006D|
|0x006E-0x0072|
|0x0086|
|0x0087/B|
|0x008C|
|0x008D|
|0x00D5|
|0x009F|
|0x00A0/4|
|0x00A2/3|
|0x00B5|
|0x00B6/A|
|0x00BB|
|0x00CE |
|0x00CF-0x00D3 |
|0x00D4 |
|0x00A6 |
|0x00A7 |
|0x00D6 |
|0x00D7 |
|0x03AD |
|0x03AE-0x03B2 |
|0x03B3 |
|0x03B8 |
|0x03B9/D |
|0x03BE |
|0x03AF |
|0x03BA |
|0x0400 |
|0x0433 |
|0x043A |
|0x043B |
|0x0450 |
|0x0456 |
|0x0490 |
|0x0491 |
|0x04AC |
|0x04B0 |
|0x04C4 |
|0x04C8 |
|0x04D0 |
|0x04E0 |
|0x0500-0x069F |
|0x06CE |
|0x06D6 |
|0x06D9 |
|0x06DE |
|0x0700 |
|0x0701 |
|0x0704 |
|0x0705 |
|0x0709 |
|0x070A |
|0x0714 |
|0x071A |
|0x071B |
|0x071C |
|0x071D |
|0x071E |
|0x0722 |
|0x0723 |
|0x0743 |
|0x0747 |
|0x0750 |
|0x0754 |
|0x0755 |
|0x0756 |
|0x075A |
|0x075E |
|0x075F |
|0x0760 |
|0x0775 |
|0x077F |
|0x0782 |
|0x0783 |
|0x0784 |
|0x0785 |
|0x0786 |
|0x0787 |
|0x0789 |
|0x078A |
|0x078F |
|0x0790 |
|0x0791 |
|0x0792 |
|0x0795 |
|0x079E |
|0x079F |
|0x07A0 |
|0x07A2 |
|-------|
|Total  |
|-------|
| 580 B |


Note that the map was only mostly complete at the time I looked at it (April 2018).

Here are some reasons why I counted what I did. 
* Things like the level data pointer, (to the cartridge memory,) were not counted because they could theoretically be represented in the code directly. Besides, the generated games will probably not need the same concept of a level anyway.
* This might not need to be said but score was not counted because score doesn't affect anything gameplay wise.
* I don't know what YMF_Dummy means. I left it out.
* I interpreted the description of 0x0500-0x069F to mean that it was the currently visible tiles used, for collision purposes.
* Game mode is left out because it is more "application state". A game without a title screen etc. would still essentially be the same game.
* Luigi stuff is also left out. Poor Luigi.
* There was some borderline stuff that I wasn't sure whether it was graphics only or not as well as some stuff which was apparently duplicated. I kept most of that stuff in.

So, the total is 580 bytes. In order to compare 240 sections with 8 colours in each to that, we need to convert both the sections and the bytes to bits. Each section has 8 = 2<sup>3</sup> possibilities, which corresponds to 3 bits. So the total number of bits in the sections is 240 * 3 bits = 720 bits. The Mario ROM on the other hand has 580 bytes = 580 * 8 bits = 4640 bits. A difference of 644%. Something to note about this comparison, is the Mario ROM does not use every single bit of the bytes it uses.,  however in practice I don't expect the generated games to do so either. However, having 3 bit sections means that if, say a section is used as a boolean, then only 2 bits are unused rather than 7, meaning that the sections are likely to have fewer unused states, within the restriction of the 8 colours. Because we represent the colours with `u32`s, the actual memory usage is quite a bit higher in the generated case. But then again Mario has additional storage for graphics which is not true in the generated case. I'm assuming that this all mostly balances out, but that may be incorrect. 

It appears that the amount of state we currently have allocated for the game is not an excessive amount so we need to do something other than merely reduce the state.


To restate the problem, we used to have the problem of the game graphs having too many cycles, and the latent trees being too short, so we started interpreting the state as a number and  only adding numbers to them. This removed all the cycles and made the latent trees much longer, but now they are too long!. We have been trying to make the trees longer for a while so being able to go the other way may lead to some interesting and/or effective ways forward.

So we could very easily just add random cycles to make the latent trees shorter, but can we spend the tree length on something more interesting? Besides the excessive length, another problem with the current version is that the games are not comprehensible. Can we use the fact that we have excess depth to increase comprehensibility? I don't see a way to at the moment. If we were to go ahead and make state predicates, then what do we expect would happen? Well one thing that would happen is the possible states would be cut down. Possibly dramatically.

So then it seems like we should go ahead and make state predicates then. We can quickly define more shorthand 
* |S -> Bool| ≡ the number of possible state predicates

Again, being able to cover the whole possible space of a given type of generated artifact seems desirable. A predicate on a set, partitions the set into two subsets, the "true" set and the "false" set. Alternatively, one can view it as a single subset of the original set. The elements that are not included are what would be in the "false" set. So a state predicate can be represented as a subset of the set of possible states. This means that there are 2<sup>|S|</sup> possible predicates. As discussed before, we have restricted the number of possible states from the amount to cover the entire screen, to just enough to give each pixel on the top row a different colour. But that still leaves us with 240 sections which can be any one of 8 colours so |S| = 8<sup>240</sup> ≈ 5.51 × 10<sup>216</sup>. Therefore |S -> Bool| = 2<sup>8<sup>240</sup></sup> ≈ 10<sup>10<sup>216</sup></sup>.




