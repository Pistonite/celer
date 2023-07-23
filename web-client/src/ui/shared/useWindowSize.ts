//! useWindowSize Hook
//!
//! This hook gets the window size and listens for changes.
import { useEffect, useState } from "react";

/// Window size type
export type WindowSize = {
    windowWidth: number;
    windowHeight: number;
};

/// Window size hook
export const useWindowSize = () => {
    const [windowSize, setWindowSize] = useState<WindowSize>({
        windowWidth: window.innerWidth - 1,
        windowHeight: window.innerHeight - 1,
    });

    useEffect(() => {
        const handler = (e?: UIEvent) => {
            let { innerWidth, innerHeight } = (e?.currentTarget ||
                window) as Window;

            // sometimes window.innerWidth does not get the correct value
            // for mobile
            const docElement = document.documentElement;
            if (docElement) {
                const { clientWidth, clientHeight } = docElement;
                if (clientWidth < innerWidth || clientHeight < innerHeight) {
                    innerWidth = clientWidth - 1;
                    innerHeight = clientHeight - 1;
                }
            }
            setWindowSize({
                windowWidth: innerWidth,
                windowHeight: innerHeight,
            });
        };
        window.addEventListener("resize", handler);
        return () => {
            window.removeEventListener("resize", handler);
        };
    }, []);

    return windowSize;
};
