# Spell

An aggressive CLI spellchecker build in Rust.

## Why?

Although it worked really fast, I was unsatisfied with spellcheckers like
`codespell`, which only check your text for commonly spelled words - the secret
to their incredibly fast performance.

I also wanted to learn more about spellcheckers and get back into writing Rust
after a nearly year-long break.

**`rspell` is different**. It doesn't just check text for common spelling
errors. Instead, **it checks every word**.

## Features

- Very fast algorithm for finding matching words. Each incorrect word can cost
  anywhere from a millisecond to correct up to 50, for really poorly spelled
  words.

- Custom dictionaries are allowed with the `DICTIONARY` environment variable

## Notes

The algorithm uses a sorted dictionary to store the words, and uses binary
search to find the closest matches, using the following heuristic:

> Most typos or misspellings that you don't catch while typing don't come at the
start of a word.

This is because it's more likely to type `misatke` than `istake`, due to the
nature of how we type.

This allows the algorithm to quickly find possible matches and display them
early - for most scenarios. If you type `azebra` instead of `zebra`, the
algorithm will take noticeably more time. But I don't think that's as likely,
so it's worth it.
