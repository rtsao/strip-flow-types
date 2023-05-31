# strip-flow-types

Remove Flow type annotations with tree-sitter

## Features

- Replaces all types with whitespace so line and column numbers are unaffected (no need for source maps)
- Compiled to native code using Rust bindings for tree-sitter
- Tested against test suites for Babel and Flow

## Usage

```js
import { transform } from "strip-flow-types";

transform(`function foo(a?: number) {}`);
//     => "function foo(a         ) {}"
```


## Limitations

- Cannot parse uncommon Flow syntax that the [tree-sitter-typescript](https://github.com/tree-sitter/tree-sitter-typescript) grammar cannot parse (see commented out tests). This can be fixed upstream
- Does not currently handle the edge case where a paren/brace must be moved to produce valid JS syntax when multiline types are removed
