// plugins {
    // id("org.jetbrains.kotlin.android") version "1.9.20"
// }
plugins {
    `kotlin-dsl`
    // id("org.gradle.kotlin.kotlin-dsl") version "4.5.0"
    // id("org.jetbrains.kotlin.android") version "1.9.20"
    // kotlin("jvm") version "1.9.20" 
    // kotlin("jvm") version "2.0.20" 
    // kotlin("jvm") version "2.1.20"
}

// gradlePlugin {
//     plugins {
//         create("pluginsForCoolKids") {
//             id = "rust"
//             implementationClass = "RustPlugin"
//         }
//     }
// }

repositories {
    google()
    mavenCentral()
}

dependencies {
    // compileOnly(gradleApi())
    // implementation("com.android.tools.build:gradle:8.0.0")
    // implementation("com.android.tools.build:gradle:8.5.0")
}
