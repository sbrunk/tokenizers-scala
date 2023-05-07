package io.brunk.tokenizers

import java.lang.ref.Cleaner

object NativeCleaner {
  val cleaner = Cleaner.create()
}
