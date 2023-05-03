package io.brunk.tokenizers

import scala.collection.immutable.ArraySeq
import org.astonbitecode.j4rs.api.Instance
import org.astonbitecode.j4rs.api.java2rust.Java2RustUtils

type Offsets = (Long, Long)

/* Represents the output of a [Tokenizer]. */
class Encoding private[tokenizers] (nativePtr: Instance[java.lang.Long]) {

  /** the total length of this Encoding */
  def length: Int = Java2RustUtils.getObjectCasted[Int](NativeInterface.encodingLength(nativePtr))

  /** The number of sequences in this Encoding */
  def nSequences: Int =
    Java2RustUtils.getObjectCasted[Int](NativeInterface.encodingNSequences(nativePtr))

  // /** IDs produced by the `Tokenizer` */
  def ids: Seq[Long] =
    ArraySeq.unsafeWrapArray(
      Java2RustUtils.getObjectCasted[Array[Long]](NativeInterface.encodingIds(nativePtr))
    )

  /* Type of the IDs */
  def typeIds: Seq[Long] =
    ArraySeq.unsafeWrapArray(
      Java2RustUtils.getObjectCasted[Array[Long]](NativeInterface.encodingTypeIds(nativePtr))
    )

  /** Tokens associated with each ID */
  def tokens: Seq[String] =
    ArraySeq.unsafeWrapArray(
      Java2RustUtils.getObjectCasted[Array[String]](NativeInterface.encodingTokens(nativePtr))
    )

  /** Indice of the word associated with each token/ID */
  def wordIds: Seq[Option[Long]] =
    val wordIds =
      Java2RustUtils.getObjectCasted[Array[Long]](NativeInterface.encodingWordIds(nativePtr))
    ArraySeq.unsafeWrapArray(wordIds.map(wordId => if wordId == -1 then None else Some(wordId)))

  /** Offsets of the token/ID from the NormalizedString */
  def offsets: Seq[Offsets] =
    val offsets =
      Java2RustUtils.getObjectCasted[Array[Offset]](NativeInterface.encodingOffsets(nativePtr))
    ArraySeq.unsafeWrapArray(offsets.map(offset => (offset.start, offset.end)))

  /** Mask identifying padding tokens for the attention mechanism */
  def attentionMask: Seq[Long] = ArraySeq.unsafeWrapArray(
    Java2RustUtils.getObjectCasted[Array[Long]](NativeInterface.encodingAttentionMask(nativePtr))
  )

  // /** Mask identifying special tokens */
  def specialTokensMask: Seq[Long] = ArraySeq.unsafeWrapArray(
    Java2RustUtils.getObjectCasted[Array[Long]](
      NativeInterface.encodingSpecialTokensMask(nativePtr)
    )
  )

  // /** A list of overflowing Encoding generated when we got truncated */
  // def overflowing: Seq[Encoding] = ???

  // /* Ranges of tokens covered by each sequence. If this is empty we consider
  //    there is only one sequence in this Encoding, and that it covers the entire range. */
  // // sequenceRanges: HashMap[usize, Range[usize]],
}
