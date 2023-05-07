package io.brunk.tokenizers

private[tokenizers]
class NativeOffsets(val starts: Array[Long], val ends: Array[Long])

case class Offset(start: Long, end: Long)
