// derived from https://github.com/oyvindberg/tui-scala/blob/9f6b67db089ac10a183fe9f992a6bc81df125bc9/scripts/src/scala/tui/scripts/GenJniLibrary.scala

package tokenizers.scripts

import bleep._
import bleep.plugin.jni.{Cargo, JniNative, JniPackage}

import java.nio.file.Path
import bleep.plugin.jni.JniJavah


/** Generate JNI C headers for native functions.
 * We still need to convert them to Rust, but we can use the signatures.  */
object Javah extends bleep.BleepScript("Javah") {

  override def run(started: Started, commands: Commands, args: List[String]): Unit = 
    JniJavah(
      logger = started.logger,
      projectPaths = started.projectPaths(mainProject),
      bloopProject = started.bloopProject(mainProject)
    ).javah()
    
}
