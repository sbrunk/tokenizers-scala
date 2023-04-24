package io.brunk.tokenizers

import java.lang.foreign.MemoryAddress
import io.brunk.tokenizers.lib_h.*
import java.lang.foreign.MemorySegment
import java.lang.foreign.MemorySession
import java.lang.foreign.ValueLayout.*
import scala.collection.immutable.ArraySeq

type Offsets = (Int, Int)

/* Represents the output of a [Tokenizer]. */
class Encoding private[tokenizers] (nativePtr: MemoryAddress, session: MemorySession) {
  private val memorySession = MemorySession.openConfined()

  private def longArrayFromPtr(ptr: MemoryAddress): ArraySeq[Long] =
    val idsSegment = MemorySegment.ofAddress(ptr, length * JAVA_LONG.byteSize(), memorySession)
    ArraySeq.unsafeWrapArray(idsSegment.toArray(JAVA_LONG))

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
    yield tokensPtr.getAtIndex(ADDRESS, i).getUtf8String(0)

  /** Indice of the word associated with each token/ID */
  def wordIds: Seq[Option[Long]] =
    val wordIds = longArrayFromPtr(encoding_word_ids(nativePtr))
    wordIds.map(wordId => if wordId == -1 then None else Some(wordId))

  /** Offsets of the token/ID from the NormalizedString */
  def offsets: Seq[Offsets] = ???

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
