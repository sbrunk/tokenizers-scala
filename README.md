# tokenizers-scala

Scala bindings for the Hugging Face [Tokenizers](https://huggingface.co/docs/tokenizers) library, written in Rust.

## Usage
```scala

import io.brunk.tokenizers.Tokenizer

val tokenizer = Tokenizer.fromPretrained("bert-base-cased")
val encoding = tokenizer.encode("Hello, y'all! How are you 😁 ?", addSpecialTokens=true)
println(encoding.length)
// 13
println(encoding.ids)
// ArraySeq(101, 8667, 117, 194, 112, 1155, 106, 1731, 1132, 1128, 100, 136, 102)
println(encoding.tokens)
// ArraySeq([CLS], Hello, ,, y, ', all, !, How, are, you, [UNK], ?, [SEP])
```

## Status
Work in progress.

Currently, we can only load and run pre-trained tokenizers. Training is not yet possible.


## How to build the project

1. Install [bleep](https://bleep.build/docs/installing/)
2. Install [Rust and Cargo](https://www.rust-lang.org/learn/get-started)
3. ```bash
   bleep compile
   bleep test
   ```
