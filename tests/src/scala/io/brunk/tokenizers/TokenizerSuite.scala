package io.brunk.tokenizers

class TokenizerSuite extends munit.FunSuite {

  // assert encoding for
  // Hello, y'all! How are you 😁 ?
  def assertEncoding(encoding: Encoding) = {
    assertEquals(encoding.length, 13)

    assertEquals(
      encoding.ids,
      Seq[Long](101, 8667, 117, 194, 112, 1155, 106, 1731, 1132, 1128, 100, 136, 102)
    )

    assert(encoding.typeIds.forall(_ == 0))

    assert(encoding.attentionMask.forall(_ == 1))

    assertEquals(encoding.specialTokensMask, 1L +: Seq.fill(11)(0L) :+ 1L)

    val expectedTokens =
      Seq("[CLS]", "Hello", ",", "y", "'", "all", "!", "How", "are", "you", "[UNK]", "?", "[SEP]")
    assertEquals(encoding.tokens, expectedTokens)

    val expectedWordIds = None +: (0 to 10).map(id => Some(id.toLong)) :+ None
    assertEquals(encoding.wordIds, expectedWordIds)

    assertEquals(encoding.nSequences, 1)

    assertEquals(
      encoding.offsets,
      Seq(
        (0L, 0L),
        (0L, 5L),
        (5L, 6L),
        (7L, 8L),
        (8L, 9L),
        (9L, 12L),
        (12L, 13L),
        (14L, 17L),
        (18L, 21L),
        (22L, 25L),
        (26L, 27L),
        (28L, 29L),
        (0L, 0L)
      )
    )
  }

  test("pretrained-tokenizer-encode") {
    val tokenizer = Tokenizer.fromPretrained("bert-base-cased")
    val encoding = tokenizer.encode("Hello, y'all! How are you 😁 ?")

    assertEncoding(encoding)
  }

  test("pretrained-tokenizer-encode-decode") {
    val tokenizer = Tokenizer.fromPretrained("bert-base-cased")
    val encoding = tokenizer.encode("Hello, y'all! How are you 😁 ?")
    val decoded = tokenizer.decode(encoding.ids)

    assertEquals(decoded, "Hello, y ' all! How are you?")
  }

  test("pretrained-tokenizer-encode-batch") {
    val tokenizer = Tokenizer.fromPretrained("bert-base-cased")
    val encodings = tokenizer.encodeBatch(Seq("Hi all", "Hello, y'all! How are you 😁 ?"))

    assertEquals(encodings.length, 2)

    assertEncoding(encodings(1))
  }

  test("pretrained-tokenizer-fail-on-invalid") {
    interceptMessage[java.lang.RuntimeException](
      """Model "invalid-tokenizer-123" on the Hub doesn't have a tokenizer"""
    ) {
      Tokenizer.fromPretrained("invalid-tokenizer-123")
    }
  }

}
