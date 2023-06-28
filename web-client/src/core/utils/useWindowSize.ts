//! useWindowSize Hook
import { useEffect, useState } from "react";

/// Hook for getting the window size and listening for changes
export const useWindowSize = () => {
    const [windowHeight, setWindowHeight] = useState(window.innerHeight-1);
    const [windowWidth, setWindowWidth] = useState(window.innerWidth-1);

    useEffect(()=>{
        const handler = (e?: UIEvent) => {
            const {innerWidth, innerHeight} = (e?.currentTarget || window) as Window;
            setWindowHeight(innerHeight-1);
            setWindowWidth(innerWidth-1);
        };
        window.addEventListener("resize", handler);
        return () => window.removeEventListener("resize", handler);
    }, []);

    return {windowHeight, windowWidth};
};

