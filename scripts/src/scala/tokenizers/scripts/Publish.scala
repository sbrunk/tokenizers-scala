package tokenizers
package scripts

import bleep._
import bleep.packaging.{packageLibraries, CoordinatesFor, PackagedLibrary, PublishLayout}
import bleep.plugin.cirelease.CiReleasePlugin
import bleep.plugin.dynver.DynVerPlugin
import bleep.plugin.nosbt.InteractionService
import bleep.plugin.pgp.PgpPlugin
import bleep.plugin.sonatype.Sonatype
import coursier.Info

import scala.collection.immutable.SortedMap

object Publish extends BleepScript("Publish") {

  def run(started: Started, commands: Commands, args: List[String]): Unit = {
    commands.compile(started.build.explodedProjects.keys.filter(projectsToPublish).toList)

    val dynVer = new DynVerPlugin(
      baseDirectory = started.buildPaths.buildDir.toFile,
      dynverSonatypeSnapshots = true
    )
    val pgp = new PgpPlugin(
      logger = started.logger,
      maybeCredentials = None,
      interactionService = InteractionService.DoesNotMaskYourPasswordExclamationOneOne
    )
    val sonatype = new Sonatype(
      logger = started.logger,
      sonatypeBundleDirectory = started.buildPaths.dotBleepDir / "sonatype-bundle",
      sonatypeProfileName = "io.brunk",
      bundleName = "tokenizers",
      version = dynVer.version,
      sonatypeCredentialHost = Sonatype.sonatype01
    )
    val ciRelease = new CiReleasePlugin(started.logger, sonatype, dynVer, pgp)

    started.logger.info(dynVer.version)

    val info = Info(
      description = "Scala bindings for Huggingface Tokenizers",
      homePage = "https://github.com/sbrunk/tokenizers-scala/",
      developers = List(
        Info.Developer(
          "sbrunk",
          "Sören Brunk",
          "https://github.com/sbrunk"
        )
      ),
      publication = None,
      scm = CiReleasePlugin.inferScmInfo,
      licenseInfo = List(
        Info.License(
          "Apache",
          Some("https://opensource.org/license/apache-2-0/"),
          distribution = Some("repo"),
          comments = None
        )
      )
    )

    val packagedLibraries: SortedMap[model.CrossProjectName, PackagedLibrary] =
      packageLibraries(
        started,
        coordinatesFor = CoordinatesFor.Default(groupId = groupId, version = dynVer.version),
        shouldInclude = projectsToPublish,
        publishLayout = PublishLayout.Maven(info)
      )

    val files: Map[RelPath, Array[Byte]] =
      packagedLibraries.flatMap { case (_, PackagedLibrary(_, files)) => files.all }

    files.foreach { case (path, bytes) =>
      started.logger.withContext(path)(_.asString).withContext(bytes.length).debug("will publish")
    }
    ciRelease.ciRelease(files)
  }
}
