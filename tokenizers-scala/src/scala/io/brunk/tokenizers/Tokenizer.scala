package io.brunk.tokenizers

import org.astonbitecode.j4rs.api.java2rust.Java2RustUtils
import org.astonbitecode.j4rs.api.Instance

class Tokenizer private (tokenizerPtr: Instance[java.lang.Long]) {
  def encode(input: String, addSpecialTokens: Boolean = true): Encoding =
    val nativeInput: Instance[String] = Java2RustUtils.createInstance(input)
    val nativeAddSpecialTokens: Instance[java.lang.Boolean] =
      Java2RustUtils.createInstance(addSpecialTokens)
    val encodingPtr =
      NativeInterface.tokenizerEncode(tokenizerPtr, nativeInput, nativeAddSpecialTokens)
    Encoding(encodingPtr)
}

object Tokenizer {

  LoadNativeTokenizers()

  /** Instantiate a new Tokenizer from an existing file on the Hugging Face Hub.
    *
    * @param identifier
    *   The identifier of a Model on the Hugging Face Hub, that contains a tokenizer.json file
    * @return
    *   The new tokenizer
    *
    * TODO revision and auth token
    */
  def fromPretrained(identifier: String): Tokenizer =
    val tokenizerPtr = NativeInterface.fromPretrained(Java2RustUtils.createInstance(identifier))
    Tokenizer(tokenizerPtr)

  // private def freeTokenizerAction(tokenizerPtr: MemoryAddress): Runnable = () =>
  //   tokenizer_free(tokenizerPtr)

}
