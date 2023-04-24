package io.brunk.tokenizers

import java.lang.foreign.MemorySession
import io.brunk.tokenizers.lib_h.*
import java.lang.foreign.MemoryAddress
import io.brunk.tokenizers.lib_h.tokenizer_encode

class Tokenizer private (tokenizerPtr: MemoryAddress, session: MemorySession) {
  def encode(input: String): Encoding =
    val nativeInput = session.allocateUtf8String(input)
    val encodingResult = tokenizer_encode(session, tokenizerPtr, nativeInput)
    val tag = ExtResult_Encoding.tag$get(encodingResult)
    if tag == OK_Encoding() then
      Encoding(ExtResult_Encoding.ok$get(encodingResult), session)
    else
      val errPtr = ExtResult_Encoding.err$get(encodingResult)
      throw new RuntimeException(errPtr.getUtf8String(0))
}

object Tokenizer {

  /** Instantiate a new Tokenizer from an existing file on the Hugging Face Hub.
    *
    * @param identifier
    *   The identifier of a Model on the Hugging Face Hub, that contains a
    *   tokenizer.json file
    * @return
    *   The new tokenizer
    *
    * TODO revision and auth token
    */
  def fromPretrained(identifier: String): Tokenizer =
    val memorySession = MemorySession.openConfined()
    val nativeIdentifier = memorySession.allocateUtf8String(identifier)
    val tokenizerResult = from_pretrained(memorySession, nativeIdentifier)
    val tag = ExtResult_Tokenizer.tag$get(tokenizerResult)
    if tag == OK_Tokenizer() then
      Tokenizer(ExtResult_Tokenizer.ok$get(tokenizerResult), memorySession)
    else
      val errPtr = ExtResult_Tokenizer.err$get(tokenizerResult)
      throw new RuntimeException(errPtr.getUtf8String(0))
}
