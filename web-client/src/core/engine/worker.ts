//import type { WorkerConsoleMessage } from "./WorkerMessge";

(function(plugins: unknown[]){
    
    // setup code goes here

    self.addEventListener('message', function() {
        // run time code goe here


        //var verLongnamePleaseModifyMe = "b";
        //console.log(verLongnamePleaseModifyMe);
        plugins.forEach((element: any) => {
            element.init?.();
        });

        postMessage({type: "done"});

    }, false);

    

})((window as any)._INJECTED_PLUGINS as unknown[]);


