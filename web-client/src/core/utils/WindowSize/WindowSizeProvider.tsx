//! Provider for WindowSizeContext

import { PropsWithChildren, useEffect, useState } from "react";

import { WindowSize, WindowSizeContext } from "./useWindowSize";

/// Provider for window size
export const WindowSizeProvider: React.FC<PropsWithChildren> = ({children}) => {
    const [windowSize, setWindowSize] = useState<WindowSize>({
        windowWidth: window.innerWidth-1,
        windowHeight: window.innerHeight-1,
    });

    useEffect(()=>{
        const handler = (e?: UIEvent) => {
            const {innerWidth, innerHeight} = (e?.currentTarget || window) as Window;
            setWindowSize({
                windowWidth: innerWidth-1,
                windowHeight: innerHeight-1,
            });
        };
        window.addEventListener("resize", handler);
        return () => {
            window.removeEventListener("resize", handler);
        };
    }, []);

    return <WindowSizeContext.Provider value={windowSize}>{children}</WindowSizeContext.Provider>;
    
};
