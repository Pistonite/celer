import "./main.css";
import { initAppRoot } from "ui/app";
import { Kernel } from "core/kernel";

const kernel = new Kernel(initAppRoot);
kernel.init();
