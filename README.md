# tokenizers-scala

[![Maven Central](https://img.shields.io/maven-central/v/io.brunk.tokenizers/tokenizers_3)](https://central.sonatype.com/artifact/io.brunk.tokenizers/tokenizers_3/)

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

## Installation

### sbt
```scala
libraryDependencies += "io.brunk.tokenizers" %% "tokenizers" % "<version>"
```

### Scala CLI

```scala
//> using lib "io.brunk.tokenizers::tokenizers:<version>"
```

### Others

Copy coordinates from Maven Central for [Scala 2.13](https://central.sonatype.com/artifact/io.brunk.tokenizers/tokenizers_2.13/) or [Scala 3](https://central.sonatype.com/artifact/io.brunk.tokenizers/tokenizers_3/).

## Status

Currently, we can only load and run pre-trained tokenizers. Training is not yet possible.


## How to build the project

1. Install [bleep](https://bleep.build/docs/installing/)
2. Install [Rust and Cargo](https://www.rust-lang.org/learn/get-started)
3. ```bash
   bleep compile
   bleep test
   ```
