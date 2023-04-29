import bleep.*

object Jextract extends BleepCodegenScript("Jextract") {

  def run(
      started: Started,
      commands: Commands,
      targets: List[Target],
      args: List[String]
  ): Unit =
    // TODO support other platforms
    val includePath = os
      .proc("xcrun", "--sdk", "macosx", "--show-sdk-path")
      .call()
      .out
      .text()
      .stripLineEnd + "/usr/include"
    targets.foreach { target =>
        val cmd = os.proc(
          "jextract",
          "-t",
          "io.brunk.tokenizers",
          "-I",
          "native/src",
          "-I",
          includePath,
          "-l",
          "tokenizers_scala",
          "--source",
          "@scripts/src/resources/includes.txt",
          "native/target/headers-gen/lib.h",
          "--output",
          target.sources.toString()
        )
        println(cmd.commandChunks.mkString(" "))
        cmd.call()
    }
}
