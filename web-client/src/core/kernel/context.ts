//! Kernel context for react to use the kernel

import { createContext, useContext } from "react";
import { Kernel } from "./Kernel";

export const KernelContext = createContext<Kernel>(null as unknown as Kernel);
export const useKernel = () => useContext(KernelContext);
