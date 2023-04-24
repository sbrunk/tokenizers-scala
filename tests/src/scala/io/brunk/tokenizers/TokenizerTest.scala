package io.brunk.tokenizers

class TokenizerSuite extends munit.FunSuite {
  test("pretrained-tokenizer") {
    val tokenizer = Tokenizer.fromPretrained("bert-base-cased")
    val encoding = tokenizer.encode("Hello, y'all! How are you 😁 ?")

    assertEquals(encoding.length, 13)

    assertEquals(
      encoding.ids,
      Seq[Long](101, 8667, 117, 194, 112, 1155, 106, 1731, 1132, 1128, 100, 136, 102)
    )
    assert(encoding.typeIds.forall(_ == 0))

    val expectedTokens =
      Seq("[CLS]", "Hello", ",", "y", "'", "all", "!", "How", "are", "you", "[UNK]", "?", "[SEP]")
    assertEquals(encoding.tokens, expectedTokens)

    val expectedWordIds = None +: (0 to 10).map(id => Some(id.toLong)) :+ None
    assertEquals(encoding.wordIds, expectedWordIds)
  }
}
