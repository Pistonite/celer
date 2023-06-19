var console = {
    log: function (){
        postMessage({level: "log", payload: Array.prototype.slice.call(arguments), type: "console", source: __plugin_name__ })
    }
};