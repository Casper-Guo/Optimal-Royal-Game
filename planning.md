# Basic Characteristics
- The number of board states is between $2 \times 10^8$ and $3 \times 10^8$.
- The game is fully Markov.
- The transition between board states is deterministic using the action definition below.
- At each state, the average number of available actions (width) is around 10.
- The average length/depth of a game is in the 120-150 range.

# Goals
- Find the optimal state-value function from the perspective of the white player.
- Then at each state, the available actions can be ranked by the value of the next state.
- If playing as black, simply reverse the ranking.

## State Representation
The key objective is to simplify the calculation of available moves and next states. The grids are referred to as follows:

```
white:  W4 W3 W2 W1 WS WE  W14  W13
public:  5  6  7  8  9 10   11   12
black:  B4 B3 B2 B1 BS BE  B14  B13
```

### Proposed Schema (WIP)
- Stored in 64-bit unsigned integers.
- Two bits each for board grids to achieve consistency in the order of W1...W4, 5...12, W13, W14, B1...B4, 5...12, B13, B14 (from least to most significant).
    - 00: unoccupied
    - 01: white
    - 10: black
- Three bits each for WS and BS from the least to the most significant (6 total)
- Bits 62-63 indicate the endgame state. This is useful for stopping search and assigning rewards.
    - 00: not set/calculated
    - 01: white win
    - 10: black win
    - 11: in progress

## Action Definition
An action is defined as the combination of the dice roll and the subsequent move on the board.

This makes it so that all possible actions, regardless of the dice roll needed, is available at every state, thus removing the need to encode the dice roll as a part of the state. This also ensures the transition following each action is fully deterministic.

## Initialization
The terminal states must be initialized to zero. Other states initialized randomly. Transition to a white win end state is rewarded positively, and vice versa. No rewards elsewhere.

## Learning
Since the policy is deterministic both during and after training. Learning $V^\pi(s)$ and $Q^\pi(s,a)$ should achieve equivalent results. The former is easier to implement.

The Bellman equation provides:

$$V^\pi(s) = \sum_a \pi(s,a) \sum_{s'}P^a_{ss'}\gamma V^\pi(s')$$

Note $P^a_{ss'} = 1 \, \forall \, a, s, s'$ since the transition is fully deterministic given a start state and a dice roll + move combination.

Note that there is no immediate reward for any action taken while the game is in progress.

## Learning Policy
Assuming a greedy policy, we can group the available moves by the dice roll needed. In each group, the move that transitions to the most valuable next state is always chosen. Then $\pi(s, a) can be based on the following dice roll distribution:

0 - $\frac{1}{16}$ (transitions to the current state)

1 - $\frac{1}{4}$

2 - $\frac{3}{8}$

3 - $\frac{1}{4}$

4 - $\frac{1}{16}$

Note that if there is no move available for a dice roll, then it transitions to the current state.

## Optimal Policy
The optimal policy is one that leads to equal or better expected return compared to all other policies at all states. This would be the greedy strategy once the optimal state value function is obtained.

## Convergence Criteria
We use value iteration with a max number of allowed iterations while also monitoring the $\Delta_{\text{max}}$ of state values. The exact threshold for stopping iteration depends on the reward value for transitioning to end states.

Convergence is accelerated by a good initial value function. We can use a very basic heuristic for this, such as (#white pieces ascended - #black pieces ascended).

## Feasibility
It is much more efficient to calculate each state's next states before running DP, so the same does not need to be done every iteration. To do this we first need a list of all valid states. This can be calculated by starting from the initial state and running BFS/DFS, alternating moves between white and black until an end state is reached. (Do this with the Python interface as we only implement transition for white in Rust).

For each state, we need to store its next states and the dice roll result needed to reach it. This map can be quite large:

The key is a 64-bit int. The values are, at minimum, a 64 bit int for the state plus a 8 bit int for the dice roll result. If we assume a state has 10 next states on average, then the memory needed is:

$3 \times 10^8 \times (64+10 \times (64+8)) = 2.352 \times 10^{11}$ bits = 29.4GB

Note that this is also an optimistic estimate since padding will be added based on Rust memory alignment rules.

Unwiedly, but maybe still within the realm of feasibility.

## Questions
What's the win/loss reward? Does it matter?

What's an appropriate discount rate?
