package io.brunk.tokenizers

import scala.collection.immutable.ArraySeq
import org.astonbitecode.j4rs.api.Instance
import org.astonbitecode.j4rs.api.java2rust.Java2RustUtils
import io.brunk.tokenizers.Encoding.freeAction

type Offset = (Long, Long)

/* Represents the output of a [Tokenizer]. */
class Encoding private[tokenizers] (nativePtr: Long) {

  private val cleanable = NativeCleaner.cleaner.register(this, freeAction(nativePtr))

  @native private def length(encodingPtr: Long): Int

  @native private def ids(encodingPtr: Long): Array[Long]

  @native private def typeIds(encodingPtr: Long): Array[Long]

  @native private def attentionMask(encodingPtr: Long): Array[Long]

  @native private def specialTokensMask(encodingPtr: Long): Array[Long]

  @native private def tokens(encodingPtr: Long): Array[String]

  @native private def wordIds(encodingPtr: Long): Array[Long]

  @native private def nSequences(encodingPtr: Long): Int

  @native private def offsets(encodingPtr: Long): NativeOffsets

  /** the total length of this Encoding */
  def length: Int = length(nativePtr)

  /** The number of sequences in this Encoding */
  def nSequences: Int = nSequences(nativePtr)

  // /** IDs produced by the `Tokenizer` */
  def ids: Seq[Long] = ArraySeq.unsafeWrapArray(ids(nativePtr))

  /* Type of the IDs */
  def typeIds: Seq[Long] = ArraySeq.unsafeWrapArray(typeIds(nativePtr))

  /** Tokens associated with each ID */
  def tokens: Seq[String] = ArraySeq.unsafeWrapArray(tokens(nativePtr))

  /** Indice of the word associated with each token/ID */
  def wordIds: Seq[Option[Long]] =
    ArraySeq.unsafeWrapArray(
      wordIds(nativePtr).map(wordId => if wordId == -1 then None else Some(wordId))
    )

  /** Offsets of the token/ID from the NormalizedString */
  def offsets: Seq[Offset] =
    val nativeOffsets = offsets(nativePtr)
    ArraySeq.unsafeWrapArray(nativeOffsets.starts.zip(nativeOffsets.ends))

  /** Mask identifying padding tokens for the attention mechanism */
  def attentionMask: Seq[Long] = ArraySeq.unsafeWrapArray(attentionMask(nativePtr))

  // /** Mask identifying special tokens */
  def specialTokensMask: Seq[Long] = ArraySeq.unsafeWrapArray(specialTokensMask(nativePtr))

  // /** A list of overflowing Encoding generated when we got truncated */
  // def overflowing: Seq[Encoding] = ???

  // /* Ranges of tokens covered by each sequence. If this is empty we consider
  //    there is only one sequence in this Encoding, and that it covers the entire range. */
  // // sequenceRanges: HashMap[usize, Range[usize]],
}

object Encoding {
  @native
  private def free(nativePtr: Long): Unit

  private def freeAction(nativePtr: Long): Runnable = () => free(nativePtr)
}
