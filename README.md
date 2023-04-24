# tokenizers-scala

Scala bindings for the Hugging Face [Tokenizers](https://huggingface.co/docs/tokenizers) library, written in Rust.

## Status
WIP. Early POC protoype. Very incomplete.

The bindings are based on the [Foreign Function & Memory (FFM) API](https://openjdk.org/jeps/424) which is a Java preview API. That means you'll currently need JDK 19 and compile and run with the javac/java `--preview` flag as the FFM API is not stabilized yet.

Why not JNI? Because JNI is painful and error prone and and I'm not getting paid for this.

