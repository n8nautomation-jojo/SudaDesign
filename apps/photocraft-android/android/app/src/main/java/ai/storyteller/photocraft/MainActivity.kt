package ai.storyteller.photocraft

import android.content.Intent
import android.graphics.Color
import android.net.Uri
import android.os.Build
import android.os.Bundle
import android.provider.OpenableColumns
import android.view.WindowManager
import androidx.activity.result.ActivityResultLauncher
import androidx.activity.result.contract.ActivityResultContracts
import androidx.core.view.ViewCompat
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat
import com.google.androidgamesdk.GameActivity

/**
 * SudaDesign MainActivity.
 *
 * Hosts the native Rust/egui/wgpu surface via GameActivity with full Edge-to-Edge display,
 * notch/cutout avoidance, system bar styling, and live WindowInsets reporting to Rust.
 *
 * Also provides the Android Storage Access Framework (SAF) bridge so Rust can:
 *   - Request an open-document picker (any file type the user owns)
 *   - Write saved files to the app's external-files directory
 */
class MainActivity : GameActivity() {

    // -----------------------------------------------------------------------
    // Activity-result launchers (registered in onCreate before first resume)
    // -----------------------------------------------------------------------

    private lateinit var openDocLauncher: ActivityResultLauncher<Intent>
    private lateinit var createDocLauncher: ActivityResultLauncher<Intent>

    // -----------------------------------------------------------------------
    // Companion object — statics for JNI calls from Rust
    // -----------------------------------------------------------------------

    companion object {
        init {
            System.loadLibrary("photocraft_android")
        }

        /** Volatile reference so Rust JNI helpers can reach the running activity. */
        @Volatile
        var instance: MainActivity? = null

        /**
         * Called from Rust (via JNI) to open the Android system file picker.
         * Runs on the Rust thread; switches to the UI thread internally.
         */
        @JvmStatic
        fun requestOpenDocument(multiple: Boolean, extensionsCsv: String) {
            instance?.runOnUiThread {
                val intent = Intent(Intent.ACTION_OPEN_DOCUMENT).apply {
                    addCategory(Intent.CATEGORY_OPENABLE)
                    type = "*/*"
                    if (multiple) {
                        putExtra(Intent.EXTRA_ALLOW_MULTIPLE, true)
                    }
                    val mimeTypes = mutableListOf<String>()
                    val exts = extensionsCsv.split(",").map { it.trim().lowercase() }.filter { it.isNotEmpty() }
                    if (exts.any { it in listOf("ttf", "otf", "woff", "woff2") }) {
                        mimeTypes.addAll(listOf(
                            "font/ttf", "font/otf", "font/woff", "font/woff2",
                            "application/x-font-ttf", "application/x-font-otf",
                            "application/font-woff", "application/font-woff2",
                            "application/octet-stream"
                        ))
                    } else {
                        mimeTypes.addAll(listOf(
                            "image/*",
                            "application/photoshop",
                            "application/x-photoshop",
                            "application/psd",
                            "application/octet-stream"
                        ))
                    }
                    mimeTypes.add("*/*")
                    putExtra(Intent.EXTRA_MIME_TYPES, mimeTypes.toTypedArray())
                }
                instance?.openDocLauncher?.launch(intent)
            }
        }

        /**
         * Called from Rust (via JNI) to show the Android system document creation (Save) picker.
         */
        @JvmStatic
        fun requestCreateDocument(suggested: String, mimeType: String) {
            instance?.runOnUiThread {
                val intent = Intent(Intent.ACTION_CREATE_DOCUMENT).apply {
                    addCategory(Intent.CATEGORY_OPENABLE)
                    type = if (mimeType.isNotEmpty()) mimeType else "*/*"
                    putExtra(Intent.EXTRA_TITLE, suggested)
                }
                instance?.createDocLauncher?.launch(intent)
            }
        }

        /**
         * Synchronously write byte payload to a content:// URI via ContentResolver.
         */
        @JvmStatic
        fun writeToContentUri(uriString: String, bytes: ByteArray): Boolean {
            val uri = Uri.parse(uriString)
            return try {
                instance?.contentResolver?.openOutputStream(uri, "wt")?.use { os ->
                    os.write(bytes)
                    os.flush()
                }
                true
            } catch (e: Exception) {
                android.util.Log.e("SudaDesign", "writeToContentUri error: $e")
                false
            }
        }
    }

    // -----------------------------------------------------------------------
    // Lifecycle
    // -----------------------------------------------------------------------

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        instance = this

        try {
            nativeInit()
        } catch (e: UnsatisfiedLinkError) {
            android.util.Log.e("SudaDesign", "nativeInit link error: $e")
        }

        // Register the SAF open-document result handler.
        openDocLauncher = registerForActivityResult(
            ActivityResultContracts.StartActivityForResult()
        ) { result ->
            if (result.resultCode == RESULT_OK) {
                val data = result.data
                val clipData = data?.clipData
                val singleUri = data?.data
                if (clipData != null && clipData.itemCount > 0) {
                    val uris = mutableListOf<Uri>()
                    for (i in 0 until clipData.itemCount) {
                        clipData.getItemAt(i).uri?.let { uris.add(it) }
                    }
                    if (uris.isNotEmpty()) {
                        handleOpenMultipleResults(uris)
                    } else {
                        onFilePickCancelled()
                    }
                } else if (singleUri != null) {
                    handleOpenResult(singleUri)
                } else {
                    onFilePickCancelled()
                }
            } else {
                onFilePickCancelled()
            }
        }

        // Register the SAF create-document (Save As / Export) result handler.
        createDocLauncher = registerForActivityResult(
            ActivityResultContracts.StartActivityForResult()
        ) { result ->
            if (result.resultCode == RESULT_OK) {
                val uri = result.data?.data
                if (uri != null) {
                    try {
                        contentResolver.takePersistableUriPermission(
                            uri,
                            Intent.FLAG_GRANT_WRITE_URI_PERMISSION or Intent.FLAG_GRANT_READ_URI_PERMISSION
                        )
                    } catch (_: SecurityException) {}
                    onSaveUriPicked(uri.toString())
                } else {
                    onSaveCancelled()
                }
            } else {
                onSaveCancelled()
            }
        }

        // 1. True Edge-to-Edge: allow window to draw behind status & navigation bars
        WindowCompat.setDecorFitsSystemWindows(window, false)

        // 2. Allow drawing into cutouts / notches so background fills screen edges
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            window.attributes.layoutInDisplayCutoutMode =
                WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_ALWAYS
        } else if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
            window.attributes.layoutInDisplayCutoutMode =
                WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_SHORT_EDGES
        }

        // 3. Transparent system bars and disable artificial contrast scrims
        window.statusBarColor = Color.TRANSPARENT
        window.navigationBarColor = Color.TRANSPARENT
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
            window.isNavigationBarContrastEnforced = false
            window.isStatusBarContrastEnforced = false
        }

        // 4. White system icons (time, battery, Wi-Fi, nav controls) for dark theme
        val insetsController = WindowCompat.getInsetsController(window, window.decorView)
        insetsController.isAppearanceLightStatusBars = false
        insetsController.isAppearanceLightNavigationBars = false

        // 5. Insets listener — reports exact safe areas and keyboard (IME) insets to Rust
        ViewCompat.setOnApplyWindowInsetsListener(window.decorView) { _, windowInsets ->
            updateInsets(windowInsets)
            windowInsets
        }

        // 6. Thermal status monitoring (Android 10+ / API 29+) to prevent overheating & throttling
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
            val pm = getSystemService(android.content.Context.POWER_SERVICE) as? android.os.PowerManager
            pm?.addThermalStatusListener { status ->
                try {
                    onThermalStatusChanged(status)
                } catch (_: UnsatisfiedLinkError) {}
            }
        }
    }

    override fun onPause() {
        super.onPause()
        try {
            onAppPauseChanged(true)
        } catch (_: UnsatisfiedLinkError) {}
    }

    override fun onResume() {
        super.onResume()
        instance = this
        try {
            onAppPauseChanged(false)
        } catch (_: UnsatisfiedLinkError) {}
        // Ensure latest insets are pushed to Rust on resume
        ViewCompat.getRootWindowInsets(window.decorView)?.let { updateInsets(it) }
            ?: ViewCompat.requestApplyInsets(window.decorView)
    }

    override fun onTrimMemory(level: Int) {
        super.onTrimMemory(level)
        try {
            onTrimMemorySignal(level)
        } catch (_: UnsatisfiedLinkError) {}
    }

    override fun onLowMemory() {
        super.onLowMemory()
        try {
            // Level 80 = ComponentCallbacks2.TRIM_MEMORY_COMPLETE
            onTrimMemorySignal(80)
        } catch (_: UnsatisfiedLinkError) {}
    }

    override fun onDestroy() {
        if (instance === this) instance = null
        super.onDestroy()
    }

    // -----------------------------------------------------------------------
    // Safe Areas & Window Insets Calculation
    // -----------------------------------------------------------------------

    private fun updateInsets(windowInsets: WindowInsetsCompat) {
        val systemBarsAndCutout = windowInsets.getInsets(
            WindowInsetsCompat.Type.systemBars() or WindowInsetsCompat.Type.displayCutout()
        )
        val ime = windowInsets.getInsets(WindowInsetsCompat.Type.ime())
        val density = resources.displayMetrics.density.coerceAtLeast(1.0f)

        // Convert physical pixels -> logical points for egui
        val left = systemBarsAndCutout.left / density
        val top = systemBarsAndCutout.top / density
        val right = systemBarsAndCutout.right / density
        val bottom = systemBarsAndCutout.bottom / density
        val imeBottom = ime.bottom / density

        try {
            onWindowInsetsChanged(left, top, right, bottom, imeBottom)
        } catch (_: UnsatisfiedLinkError) {
            // Native library still initializing
        }
    }

    // -----------------------------------------------------------------------
    // SAF open handler
    // -----------------------------------------------------------------------

    /**
     * Read the file at [uri] and forward `(name, bytes)` to Rust via the JNI
     * symbol [onFilePicked], or call [onFilePickCancelled] on any error.
     */
    private fun handleOpenResult(uri: Uri) {
        try {
            // Persist read permission so we can re-open the URI later if needed.
            contentResolver.takePersistableUriPermission(
                uri,
                Intent.FLAG_GRANT_READ_URI_PERMISSION
            )
        } catch (_: SecurityException) {
            // Non-persistable URIs are fine — we already have one-time access.
        }

        val name = queryDisplayName(uri) ?: uri.lastPathSegment ?: "document"

        // Zero-copy FD path: pass raw file descriptor directly to Rust with 0 bytes allocated on JVM heap.
        var openedViaFd = false
        try {
            val afd = contentResolver.openAssetFileDescriptor(uri, "r")
            if (afd != null) {
                val startOffset = afd.startOffset
                val declaredLength = afd.declaredLength
                val length = if (declaredLength >= 0) declaredLength else afd.length
                val pfd = afd.parcelFileDescriptor
                if (pfd != null) {
                    val rawFd = pfd.detachFd()
                    // pfd is detached; rawFd is now exclusively owned by native code
                    afd.close()
                    android.util.Log.i("SudaDesign", "handleOpenResult: passing FD $rawFd (offset=$startOffset, len=$length) for '$name' to Rust")
                    onFileDescriptorPicked(name, rawFd, startOffset, length)
                    openedViaFd = true
                } else {
                    afd.close()
                }
            }
        } catch (e: Exception) {
            android.util.Log.w("SudaDesign", "openAssetFileDescriptor failed, falling back to stream: $e")
        }

        if (openedViaFd) {
            return
        }

        // Fallback: Read bytes only if FD extraction fails (e.g. virtual content providers)
        val bytes: ByteArray? = try {
            contentResolver.openInputStream(uri)?.use { it.readBytes() }
        } catch (e: Exception) {
            android.util.Log.e("SudaDesign", "handleOpenResult: failed to read bytes: $e")
            null
        }

        if (bytes != null) {
            android.util.Log.i("SudaDesign", "handleOpenResult fallback: '$name' ${bytes.size} bytes")
            // Signed byte array for JNI — Rust side casts back to u8.
            val signed = bytes.map { it.toByte() }.toByteArray()
            onFilePicked(name, signed)
        } else {
            onFilePickCancelled()
        }
    }

    /**
     * Read multiple files from [uris] and deliver to Rust.
     */
    private fun handleOpenMultipleResults(uris: List<Uri>) {
        if (uris.isEmpty()) {
            onFilePickCancelled()
            return
        }
        val first = uris.first()
        handleOpenResult(first)
    }

    /** Query the human-readable display name for a content URI. */
    private fun queryDisplayName(uri: Uri): String? {
        return try {
            contentResolver.query(uri, arrayOf(OpenableColumns.DISPLAY_NAME), null, null, null)
                ?.use { cursor ->
                    if (cursor.moveToFirst()) {
                        cursor.getString(cursor.getColumnIndexOrThrow(OpenableColumns.DISPLAY_NAME))
                    } else null
                }
        } catch (_: Exception) {
            null
        }
    }

    // -----------------------------------------------------------------------
    // Native callbacks (implemented in Rust as `extern "system"` symbols)
    // -----------------------------------------------------------------------

    /** Cached activity reference for JNI bridge without classloader lookup issues. */
    private external fun nativeInit()

    /** Report updated Safe Area and IME Insets to Rust (in logical points). */
    private external fun onWindowInsetsChanged(
        left: Float,
        top: Float,
        right: Float,
        bottom: Float,
        ime: Float
    )

    /** Forward picked file bytes to Rust. Implemented in `services.rs`. */
    private external fun onFilePicked(name: String, bytes: ByteArray)

    /** Forward picked file descriptor to Rust for zero-copy streaming. Implemented in `services.rs`. */
    private external fun onFileDescriptorPicked(
        name: String,
        fd: Int,
        startOffset: Long,
        length: Long
    )

    /** Notify Rust the picker was cancelled. Implemented in `services.rs`. */
    private external fun onFilePickCancelled()

    /** Forward user-chosen save URI to Rust. Implemented in `services.rs`. */
    private external fun onSaveUriPicked(uri: String)

    /** Notify Rust that the save picker was cancelled. Implemented in `services.rs`. */
    private external fun onSaveCancelled()

    /** Notify Rust to trim memory / release caches under Android memory pressure. */
    private external fun onTrimMemorySignal(level: Int)

    /** Notify Rust of thermal status change (0=None, 1=Light, 2=Moderate, 3=Severe, 4=Critical). */
    private external fun onThermalStatusChanged(status: Int)

    /** Notify Rust that the app entered background/paused or resumed. */
    private external fun onAppPauseChanged(paused: Boolean)
}
