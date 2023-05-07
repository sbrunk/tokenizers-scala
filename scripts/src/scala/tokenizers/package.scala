package tokenizers

import bleep.model
import bleep.model.CrossProjectName

package object scripts {
  val nativeProject: model.CrossProjectName =
    model.CrossProjectName(model.ProjectName("native"), None)
  val mainProject: model.CrossProjectName =
    model.CrossProjectName(model.ProjectName("tokenizers"), None)

  // will publish these with dependencies
  def projectsToPublish(crossName: model.CrossProjectName): Boolean =
    crossName.name.value match {
      case "native" | "tokenizers" => true
      case _ => false
    }
  
  val groupId = "io.brunk.tokenizers"
}
