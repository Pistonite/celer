//! Kernel context for react to use the kernel

import React from "react";
import { Kernel } from "./Kernel";

export const KernelContext = React.createContext<Kernel>(null as unknown as Kernel);
export const useKernel = () => React.useContext(KernelContext);
