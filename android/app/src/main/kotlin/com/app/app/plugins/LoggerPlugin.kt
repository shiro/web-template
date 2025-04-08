package com.app.app.plugins;

import com.getcapacitor.JSObject
import com.getcapacitor.Plugin
import com.getcapacitor.PluginCall
import com.getcapacitor.PluginMethod
import com.getcapacitor.annotation.CapacitorPlugin

@CapacitorPlugin(name = "Logger")
class LoggerPlugin: Plugin() {
    @PluginMethod
    fun log(call: PluginCall) {
        val value = call.getString("value")

        val ret = JSObject()
        ret.put("value", value)
        call.resolve(ret)
    }
}
