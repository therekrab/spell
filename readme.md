# Spell

A really fast spellchecker built in Rust.

## A Quick Note

This project runs of a ~5K word dictionary, so it is important to consider that
it is not meant for real world use. However, more would could be added easily
without affecting program runtime (too much).

## How it works

Under the hood, this is just a lot of optimizations on the [Levenshtein
distance algorithm](https://en.wikipedia.org/wiki/Levenshtein_distance). I have
used the much more optimized Wagner–Fischer version of this algorithm, which
uses an iterative matrix approach rather than a recursive approach, but the
main idea is the same. Two words are compared and an `edit distance` is
produced. That is the minimum number of changes needed to get from one word to
the other. Those changes are:

Insertion: `a c d` to `a b c d`

Deletion: `a b c d` to `a b d`

Substitution: `a b c` to `a b d`

### Speed improvements

Because my algorithm only wants one replacement, there is a lot of opportunity
for optimizaions.

__Optimization #1__: Check through the list for the word *first*, so no edit
distances need to be calculated unnecesarily.

__Optimization #2__: Because of the previous optimization, the minimum distance
that will be found will be 1. So, the algorithm can return as soon as it finds
a word with a distance of 1.

__Optimization #3__: Add a quick stop to the distance calculation. If the spell
checker is keeping track of the closest word and its distance, then the
distance calculation should know the smallest distance, and if there is ever a
point where the minimum distance between two words is larger than that limit,
it will terminate the calculation early. This optimization was incredilby
helpful, nearly halving the time the program took.

__Optimizaion #4__: Sort the dictionary in alphabetical order (no real
improvement, but sets the stage for optimizations 5 and 6). It is also
important to notice that when people misspell words, it is normally not in the
first character(s) of the word.=

__Optimization #5__: Use Optimization #5 to binary search through the words to
get an idea of where the checked word would be to get an index `i`. If the
dictionary at `i` is the input word, then we can stop now. This is better than
a search over every element in the list, b/c it's time complexity is `O(log n)`
rather than `O(n)`.

__Optimization #6__: Once `i` is found, create a left and right pointer that
will start at `i`, then work out from there to search the closest words first.
This means that if the word starts with the same letter(s) as the correctly
spelled word, the correcly spelled word will be reached quite quickly.

### The result of optimizaions:

The combination of all of the above optimizations is quite noticable, taking
the algorithm from a time of multiple seconds to a time of under 10
milliseconds, including loading the dictionary. Looking at the time of just the
spell checker, we see times of under a millisecond. These results are quite
astonishing, and they show the power of optimizations.
