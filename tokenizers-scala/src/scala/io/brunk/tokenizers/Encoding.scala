package io.brunk.tokenizers

import java.lang.foreign.MemoryAddress
import io.brunk.tokenizers.lib_h.*
import java.lang.foreign.MemorySegment
import java.lang.foreign.MemorySession
import java.lang.foreign.ValueLayout.*
import scala.collection.immutable.ArraySeq
import java.lang.foreign.MemoryLayout
import java.lang.foreign.SequenceLayout

type Offsets = (Long, Long)

/* Represents the output of a [Tokenizer]. */
class Encoding private[tokenizers] (nativePtr: MemoryAddress, session: MemorySession) {
  private val memorySession = MemorySession.openConfined()

  private def longArrayFromPtr(ptr: MemoryAddress): ArraySeq[Long] =
    val len = length
    val segment = MemorySegment.ofAddress(ptr, len * JAVA_LONG.byteSize(), memorySession)
    val arr = ArraySeq.unsafeWrapArray(segment.toArray(JAVA_LONG))
    vec_free(ptr, len)
    arr

  /** the total length of this Encoding */
  def length: Int = encoding_len(nativePtr)

  /** The number of sequences in this Encoding */
  def nSequences: Int = encoding_n_sequences(nativePtr)

  /** IDs produced by the `Tokenizer` */
  def ids: Seq[Long] = longArrayFromPtr(encoding_ids(nativePtr))

  /* Type of the IDs */
  def typeIds: Seq[Long] = longArrayFromPtr(encoding_type_ids(nativePtr))

  /** Tokens associated with each ID */
  def tokens: Seq[String] =
    val tokensPtr = encoding_tokens(nativePtr)
    for i <- 0 until length
    yield
      val strPtr = tokensPtr.getAtIndex(ADDRESS, i)
      val str = strPtr.getUtf8String(0)
      string_free(strPtr)
      str

  /** Indice of the word associated with each token/ID */
  def wordIds: Seq[Option[Long]] =
    val wordIds = longArrayFromPtr(encoding_word_ids(nativePtr))
    wordIds.map(wordId => if wordId == -1 then None else Some(wordId))

  /** Offsets of the token/ID from the NormalizedString */
  def offsets: Seq[Offsets] =
    val offsetsPtr = encoding_offsets(nativePtr)
    val layout: SequenceLayout = MemoryLayout.sequenceLayout(length, Offset.$LAYOUT())
    val offsetsSegment = MemorySegment.ofAddress(offsetsPtr, layout.byteSize(), session)
    for i <- 0 until length
    yield
        val start = Offset.start$get(offsetsSegment, i)
        val end = Offset.end$get(offsetsSegment, i)
        (start, end)
      

  /** Mask identifying special tokens */
  def special_tokens_mask: Seq[Long] = longArrayFromPtr(encoding_special_tokens_mask(nativePtr))

  /** Mask identifying padding tokens for the attention mechanism */
  def attention_mask: Seq[Long] = longArrayFromPtr(encoding_attention_mask(nativePtr))

  /** A list of overflowing Encoding generated when we got truncated */
  def overflowing: Seq[Encoding] = ???

  /* Ranges of tokens covered by each sequence. If this is empty we consider
     there is only one sequence in this Encoding, and that it covers the entire range. */
  // sequence_ranges: HashMap[usize, Range[usize]],
}
