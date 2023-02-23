# shexantlr 

This module contains the ANLTR-based ShEx Compact parser.

- jar: Contains the JAR binary obtained from [ANTLR-Rust](https://github.com/rrevenantt/antlr4rust/releases/download/antlr4-4.8-2-Rust0.3.0-beta/antlr4-4.8-2-SNAPSHOT-complete.jar)
- grammar/ShExDoc.g4 contains ShExC grammar
- src/grammar: Generated parsers and lexers

# Building

Although `build.rs` is supposed to generate the parsers...it seems it doesn't do it at this moment when we execute `cargo build`. I found that I can generate them just running the following command.

## Generating parsers manually from ANTLR

```
java -jar jar/antlr4-4.8-2-SNAPSHOT-complete.jar -Dlanguage=Rust -visitor -Xlog -o src/grammar grammar/ShExDoc.g4
```
