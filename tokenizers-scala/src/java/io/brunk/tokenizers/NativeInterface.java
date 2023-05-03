package io.brunk.tokenizers;

import org.astonbitecode.j4rs.api.Instance;

class NativeInterface {
  public static native Instance<Long> fromPretrained(Instance<String> identifier);
  public static native Instance<Long> tokenizerEncode(Instance<Long> tokenizerPtr, Instance<String> input, Instance<Boolean> addSpecialTokens);
  public static native Instance<Integer> encodingLength(Instance<Long> encodingPtr);
  public static native Instance<Long[]> encodingIds(Instance<Long> encodingPtr);
  public static native Instance<Long[]> encodingTypeIds(Instance<Long> encodingPtr);
  public static native Instance<Long> encodingAttentionMask(Instance<Long> encodingPtr);
  public static native Instance<Long> encodingSpecialTokensMask(Instance<Long> encodingPtr);
  public static native Instance<String[]> encodingTokens(Instance<Long> encodingPtr);
  public static native Instance<Long[]> encodingWordIds(Instance<Long> encodingPtr);
  public static native Instance<Integer> encodingNSequences(Instance<Long> encodingPtr);
  public static native Instance<Offset> encodingOffsets(Instance<Long> encodingPtr);
}
