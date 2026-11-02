# ProGuard rules for SudaDesign (PhotoCraft Android)

# Keep GameActivity and Game SDK classes
-keep class androidx.games.activity.** { *; }
-keep class com.google.androidgamesdk.** { *; }

# Keep MainActivity and all its JNI exported and callback methods
-keep class ai.storyteller.photocraft.MainActivity {
    public *;
    native <methods>;
}

# Keep any class with native methods called by Rust JNI
-keepclasseswithmembernames class * {
    native <methods>;
}

# Preserve LineNumberTable and SourceFile for actionable production crash reports
-keepattributes SourceFile,LineNumberTable

