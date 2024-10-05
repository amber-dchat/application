package com.plugin.aupdater

import android.app.Activity
import android.content.Intent
import android.net.Uri
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@TauriPlugin
class Updater(private val activity: Activity): Plugin(activity) {
    @Command
    fun open(invoke: Invoke) {
      val url = Uri.parse(invoke.parseArgs(String::class.java))
      val intent = Intent(Intent.ACTION_VIEW, url)

      val ret = JSObject()

      try {
        activity.startActivity(intent)
        ret.put("success", true)
      } catch (e: Exception) {
        ret.put("success", false)
      }
      invoke.resolve(ret)
    }
}
