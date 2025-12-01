# Notes

- The `fold` function is a powerful and flexible iterator method in Rust often used to reduce or accumulate values. It takes an initial accumulator value and a closure (or function) as arguments. The closure is then applied to each element of the iterator, updating the accumulator.
    ```rust
    let modified_line = word_mapping.iter().fold(line.to_string(), |acc, (word, replacement)| {
        acc.replace(word, replacement)
    });

    ```
    Here's a breakdown of how it works:
    - `word_mapping.iter()` creates an iterator over the elements of word_mapping, which are tuples `(word, replacement)`.
    - `fold(line.to_string(), |acc, (word, replacement)| { ... })` starts the folding process. The initial value of the accumulator (acc) is set to `line.to_string()`.
    - The closure `|acc, (word, replacement)| { acc.replace(word, replacement) }` is applied to each element of the iterator. It takes the current accumulator value (acc) and the tuple (word, replacement).
    - Inside the closure, `acc.replace(word, replacement)` is called. This replaces occurrences of word with replacement in the current accumulator (acc).
    - The result of `acc.replace(word, replacement)` becomes the new value of the accumulator for the next iteration.
    - The final result of the fold operation is the modified line where all the specified words have been replaced according to the word mapping.
# Tips
- In the case of overlapping words like "eightwo" that should be translated to 82, we can replace the word with the format `<eng><num><eng>` (e.g., eight8eight) so that the overlapping part will remain and we can reuse the part1 solution as well.
