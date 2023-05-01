package io.brunk.tokenizers

import java.lang.foreign.MemorySession
import io.brunk.tokenizers.lib_h.*
import java.lang.foreign.MemoryAddress
import io.brunk.tokenizers.lib_h.tokenizer_encode
import io.brunk.tokenizers.Tokenizer.freeTokenizerAction
import java.lang.foreign.MemorySegment
import io.brunk.tokenizers.Tokenizer.extractError

class Tokenizer private (tokenizerPtr: MemoryAddress, session: MemorySession)
    extends AutoCloseable {

  private val cleanable =
    io.brunk.tokenizers.NativeCleaner.cleaner.register(this, freeTokenizerAction(tokenizerPtr))

  def encode(input: String, addSpecialTokens: Boolean = true): Encoding =
    val nativeInput = session.allocateUtf8String(input)
    val encodingResult = tokenizer_encode(session, tokenizerPtr, nativeInput, addSpecialTokens)
    val tag = ExtResult_Encoding.tag$get(encodingResult)
    if tag == OK_Encoding() then Encoding(ExtResult_Encoding.ok$get(encodingResult), session)
    else
      val errPtr = ExtResult_Encoding.err$get(encodingResult)
      extractError(errPtr)

  override def close(): Unit = cleanable.clean()
}

object Tokenizer {

  LoadNativeTokenizers()

  def extractError(errPtr: MemoryAddress) =
    val e = errPtr.getUtf8String(0)
    string_free(errPtr)
    throw new RuntimeException(e)

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
    val memorySession = MemorySession.openConfined()
    val nativeIdentifier = memorySession.allocateUtf8String(identifier)
    val tokenizerResult = from_pretrained(memorySession, nativeIdentifier)
    val tag = ExtResult_Tokenizer.tag$get(tokenizerResult)
    if tag == OK_Tokenizer() then
      val tokenizer = Tokenizer(ExtResult_Tokenizer.ok$get(tokenizerResult), memorySession)
      tokenizer
    else
      val errPtr = ExtResult_Tokenizer.err$get(tokenizerResult)
      extractError(errPtr)

  private def freeTokenizerAction(tokenizerPtr: MemoryAddress): Runnable = () =>
    tokenizer_free(tokenizerPtr)

}
