# Advent of Code 2024
This year, I'm using a simplified version of [@ChristopherBiscardi](https://github.com/ChristopherBiscardi)'s setup for creating daily templates.

The frequently used commands for this project are documented and managed by [just](https://github.com/casey/just) (see `justfile`) to improve productivity and consistency.

# Goal 🎯
The goal this year is to complete at least part1 each day.

# Create a new day
The naming convention for each day is `day-XX`
```bash
just create <day>
```

# Handy Crates
## Parsing and Lexical Analysis
- `itertools`: Enhances iterator capabilities with extra combinators, adaptors, and functions for advanced iterator manipulations.
- `nom`: A powerful, versatile parser combinator library for quickly building custom parsers.
- `nom_locate`: Extends nom by adding location tracking to parsed data, which is useful for error reporting and debugging.
- `nom-supreme`: Adds error reporting and context to nom, making debugging and providing detailed feedback for failed parses more accessible.
## Error Handling
- `miette`: Provides user-friendly, formatted error reporting with optional enhancements like syntax highlighting (fancy feature).
- `thiserror`: A simple and ergonomic library for defining custom error types with minimal boilerplate.
## Logging and Tracing
- `tracing`: A structured, async-compatible logging library for instrumenting code and collecting runtime diagnostics.
- `tracing-subscriber`: Subscriber implementation for tracing, including support for formatted output (fmt) and runtime filtering (env-filter).
## Testing
- `rstest`: A procedural macro for parameterized testing, making test cases more concise and readable.
- `rstest_reuse`: Enables reuse of rstest-based test templates, reducing redundancy in test definitions.
- `test-log`: Facilitates logging within test cases, supporting tracing integration to enhance test diagnostics.
