// for updates check:
// https://github.com/ionic-team/capacitor/tree/main/android-template
// https://github.com/tauri-apps/tauri-mobile/tree/dev/templates/platforms/android-studio

plugins {
    id("org.mozilla.rust-android-gradle.rust-android")

    id("com.android.application")
    id("org.jetbrains.kotlin.android") version "1.9.20"
    // id("org.jetbrains.kotlin.android") version "2.1.10"
    // id("rust")

    // id("org.mozilla.rust-android-gradle.rust-android")
    // id("org.mozilla.rust-android-gradle.rust-android") version "0.9.3"
}


apply(plugin = "com.android.application")

android {
    namespace = System.getenv("ANDROID_APPID")
    compileSdkVersion(rootProject.ext["compileSdkVersion"] as Int)
    buildFeatures {
        buildConfig = true
    }
    defaultConfig {
        applicationId = System.getenv("ANDROID_APPID")
        minSdkVersion(rootProject.ext["minSdkVersion"] as Int)
        targetSdkVersion(rootProject.ext["targetSdkVersion"] as Int)
        versionCode = 1
        versionName = "1.0.0"
        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        aaptOptions {
            // Files and dirs to omit from the packaged assets dir, modified to accommodate modern web apps.
            // Default: https://android.googlesource.com/platform/frameworks/base/+/282e181b58cf72b6ca770dc7ca5f91f135444502/tools/aapt/AaptAssets.cpp#61
            ignoreAssetsPattern = "!.svn:!.git:!.ds_store:!*.scc:.*:!CVS:!thumbs.db:!picasa.ini:!*~"
        }
        manifestPlaceholders["ANDROID_APP_NAME"] = "\"${System.getenv("ANDROID_APP_NAME")}\""
        // manifestPlaceholders["ANDROID_APP_BG_COLOR"] = "\"${System.getenv("ANDROID_APP_BG_COLOR")}\""
        // buildConfigField("Color", "foo", System.getenv("ANDROID_APP_BG_COLOR"))
        resValue("color", "ANDROID_APP_BG_COLOR", "${System.getenv("ANDROID_APP_BG_COLOR")}") // Red color
    }
    buildTypes {
        getByName("debug") {
            isDebuggable = true
            isJniDebuggable = true
            isMinifyEnabled = false
            packaging {
                jniLibs.keepDebugSymbols.add("*/arm64-v8a/*.so")
                jniLibs.keepDebugSymbols.add("*/armeabi-v7a/*.so")
                jniLibs.keepDebugSymbols.add("*/x86/*.so")
                jniLibs.keepDebugSymbols.add("*/x86_64/*.so")
            }
            buildConfigField("String", "PUBLIC_HOST", "\"${System.getenv("PUBLIC_HOST") ?: ""}\"")
        }
        named("release") {
            isMinifyEnabled = false
            setProguardFiles(listOf(getDefaultProguardFile("proguard-android.txt"), "proguard-rules.pro"))
            buildConfigField("String", "PUBLIC_HOST", "\"${System.getenv("PUBLIC_HOST") ?: ""}\"")
        }
    }
    assetPacks += mutableSetOf()
    ndkVersion = "25.1.8937393"
}

repositories {
    flatDir {
        dirs = setOf(file("../capacitor-cordova-android-plugins/src/main/libs"), file("libs"))
    }
}

// rust {
//     rootDirRel = "../rust"
// }


// apply plugin: 'org.mozilla.rust-android-gradle.rust-android'
//
// cargo {
//     module  = "../rust"       // Or whatever directory contains your Cargo.toml
//     libname = "rust"          // Or whatever matches Cargo.toml's [package] name.
//     targets = ["arm", "x86"]  // See bellow for a longer list of options
// }

// cargo {
//     module.set("../rust")
//     targets.set(listOf("x86_64", "arm64"))
//     libname.set("rust")
// }

// project.afterEvaluate {
//     tasks.withType(com.nishtahir.CargoBuildTask::class)
//     .forEach { buildTask ->
//         tasks.withType(com.android.build.gradle.tasks.MergeSourceSetFolders::class)
//         .configureEach {
//             this.dependsOn(buildTask)
//         }
//     }
// }


// cargo {
//     module.set("../rust")      // Or whatever directory contains your Cargo.toml
//     libname.set("rust")        // Or whatever matches Cargo.toml's [package] name
//     targets.set(listOf("arm", "x86")) // Can be expanded with other targets
// }

dependencies {
    // capacitor
    implementation(fileTree(mapOf("dir" to "libs", "include" to listOf("*.jar"))))
    implementation("androidx.appcompat:appcompat:${rootProject.ext["androidxAppCompatVersion"] as String}")
    implementation("androidx.coordinatorlayout:coordinatorlayout:${rootProject.ext["androidxCoordinatorLayoutVersion"] as String}")
    implementation("androidx.core:core-splashscreen:${rootProject.ext["coreSplashScreenVersion"] as String}")
    implementation(project(":capacitor-android"))
    implementation(project(":capacitor-cordova-android-plugins"))

    // general
    implementation("androidx.media:media:1.6.0")
}

apply(from = "capacitor.build.gradle")

try {
    val servicesJSON = file("google-services.json")
    if (servicesJSON.exists()) {
        apply(plugin = "com.google.gms.google-services")
    }
} catch (e: Exception) {
    logger.info("google-services.json not found, google-services plugin not applied. Push Notifications won't work")
}

// tasks.whenTaskAdded { task ->
//     if ((task.name == 'javaPreCompileDebug' || task.name == 'javaPreCompileRelease')) {
//         task.dependsOn 'cargoBuild'
//     }
// }

// compile rust code before compiling the app
// afterEvaluate {
//      android.applicationVariants.all {
//          tasks["mergeReleaseJniLibFolders"].dependsOn("cargoBuild")
//          tasks["mergeDebugJniLibFolders"].dependsOn("cargoBuild")
//          // productFlavors.filter { it.name != "universal" }.forEach { _ ->
//              //     val archAndBuildType = name.capitalize()
//              //     tasks["merge${archAndBuildType}JniLibFolders"].dependsOn(tasks["rustBuild${archAndBuildType}"])
//              // }
//     }
// }

cargo {
    module = "../rust"
    targets = listOf("x86_64", "arm64")
    libname = "rust"
}
afterEvaluate {
    tasks.configureEach {
        if (name.startsWith("generate") && name.endsWith("Assets")) {
            dependsOn("cargoBuild")
        }
    }
}

// project.afterEvaluate {
//     tasks.withType(com.nishtahir.CargoBuildTask::class)
//     .forEach { buildTask ->
//         tasks.withType(com.android.build.gradle.tasks.MergeSourceSetFolders::class)
//         .configureEach {
//             this.inputs.dir(
//                 layout.buildDirectory.dir("rustJniLibs" + File.separatorChar + buildTask.toolchain!!.folder)
//             )
//             this.dependsOn(buildTask)
//         }
//     }
// }
//
