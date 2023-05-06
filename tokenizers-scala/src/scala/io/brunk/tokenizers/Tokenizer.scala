package io.brunk.tokenizers

import io.brunk.tokenizers.Tokenizer.freeAction

import scala.annotation.static
import io.brunk.tokenizers.Tokenizer.free
import scala.collection.immutable.ArraySeq

class Tokenizer private (nativePtr: Long) {

  private val cleanable = NativeCleaner.cleaner.register(this, freeAction(nativePtr))

  @native
  private def encode(
      tokenizerPtr: Long,
      input: String,
      addSpecialTokens: Boolean
  ): Long

  def encode(input: String, addSpecialTokens: Boolean = true): Encoding =
    val encodingPtr = encode(nativePtr, input, addSpecialTokens)
    Encoding(encodingPtr)

  @native
  private def encodeBatch(
      tokenizerPtr: Long,
      input: Array[String],
      addSpecialTokens: Boolean
  ): Array[Long]

  def encodeBatch(input: Seq[String], addSpecialTokens: Boolean = true): Seq[Encoding] =
    val encodingsPtr = encodeBatch(nativePtr, input.toArray, addSpecialTokens)
    ArraySeq.unsafeWrapArray(encodingsPtr.map(ptr => Encoding(ptr: Long)))
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
    val nativePtr = fromPretrainedNative(identifier)
    Tokenizer(nativePtr)

  @native
  private def fromPretrainedNative(identifier: String): Long

  @native
  private def free(nativePtr: Long): Unit

  private def freeAction(nativePtr: Long): Runnable = () => free(nativePtr)

}
