package tokenizers

import bleep.model
import bleep.model.CrossProjectName

package object scripts {
  val nativeProject: model.CrossProjectName =
    model.CrossProjectName(model.ProjectName("native"), None)
}
