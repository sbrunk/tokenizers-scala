package io.brunk.tokenizers

import io.brunk.tokenizers.Tokenizer.freeAction

import scala.collection.immutable.ArraySeq
import java.nio.file.Path

class Tokenizer private (nativePtr: Long) {

  NativeCleaner.cleaner.register(this, freeAction(nativePtr))

  @native
  private def encode(
      tokenizerPtr: Long,
      input: String,
      addSpecialTokens: Boolean
  ): Long

  def encode(input: String, addSpecialTokens: Boolean = true): Encoding = {
    val encodingPtr = encode(nativePtr, input, addSpecialTokens)
    new Encoding(encodingPtr)
  }

  @native
  private def encodeBatch(
      tokenizerPtr: Long,
      input: Array[String],
      addSpecialTokens: Boolean
  ): Array[Long]

  def encodeBatch(input: Seq[String], addSpecialTokens: Boolean = true): Seq[Encoding] = {
    val encodingsPtr = encodeBatch(nativePtr, input.toArray, addSpecialTokens)
    ArraySeq.unsafeWrapArray(encodingsPtr.map(ptr => new Encoding(ptr: Long)))
  }

  @native
  private def decode(
      tokenizerPtr: Long,
      ids: Array[Long],
      skipSpecialTokens: Boolean
  ): String

  def decode(ids: Seq[Long], skipSpecialTokens: Boolean = true): String =
    decode(nativePtr, ids.toArray, skipSpecialTokens)
}

object Tokenizer {

  new LoadNativeTokenizers()

  /** Instantiate a new Tokenizer from an existing file on the Hugging Face Hub.
    *
    * @param identifier
    *   The identifier of a Model on the Hugging Face Hub, that contains a tokenizer.json file
    * @return
    *   The new tokenizer
    *
    * TODO revision and auth token
    */
  def fromPretrained(identifier: String): Tokenizer = {
    val nativePtr = fromPretrainedNative(identifier)
    new Tokenizer(nativePtr)
  }

  /** Instantiate a new Tokenizer from the file at the given path.
    *
    * @param path
    *   A path to a local JSON file representing a previously serialized [[Tokenizer]]
    * @return
    *   The new tokenizer
    */
  def fromFile(path: Path): Tokenizer = new Tokenizer(fromFile(path.toString()))

  @native
  private def fromPretrainedNative(identifier: String): Long

  @native
  private def fromFile(path: String): Long

  @native
  private def free(nativePtr: Long): Unit

  private def freeAction(nativePtr: Long): Runnable = () => free(nativePtr)

}
