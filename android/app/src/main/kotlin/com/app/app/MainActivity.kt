package com.app.app;

import android.os.Bundle
import com.getcapacitor.BridgeActivity

import com.app.app.plugins.EchoPlugin

class MainActivity: BridgeActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        registerPlugin(EchoPlugin::class.java)

        super.onCreate(savedInstanceState)

        // config
        // super.getBridge().webView.isVerticalScrollBarEnabled = false
        // super.getBridge().webView.overScrollMode = View.OVER_SCROLL_NEVER

        // call rust plugin
        create(this)
    }

    private external fun create(activity: MainActivity)

    // companion object {
    //     init {
    //         System.loadLibrary("fujipod")
    //     }
    // }

    // private external fun create(activity: MainActivity)
    companion object { init { System.loadLibrary("rust") } }
    // }
}
