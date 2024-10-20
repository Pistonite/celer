//! Kernel implementation for the EDIT mode

import type { FsFileSystem } from "@pistonite/pure/fs";

import type { AppStore } from "core/store";
import { settingsActions, settingsSelector, viewActions } from "core/store";
import { injectSplitTypesIntoRequest, setDocument } from "core/doc";
import type { EditorKernel, EditorKernelAccess } from "core/editor";
import type { CompilerKernel } from "core/compiler";
import type { AlertMgr } from "low/utils";
import { consoleKernel as console } from "low/utils";
import type { ExpoDoc, ExportRequest } from "low/celerc";

import type { Kernel, KernelEdit } from "./Kernel";
import type { UiMgrInitFn } from "./UiMgr";
import { UiMgr } from "./UiMgr";
import { KeyMgr } from "./KeyMgr";
import { createAndBindStore } from "./store";
import { AlertMgrImpl } from "./AlertMgr";

/// The kernel class
///
/// The kernel owns all global resources like the redux store.
/// It is also responsible for mounting react to the DOM and
/// handles the routing.
export class KernelEditImpl implements Kernel, KernelEdit, EditorKernelAccess {
    private store: AppStore;
    private uiMgr: UiMgr;
    private keyMgr: KeyMgr;

    public readonly alertMgr: AlertMgr;

    // Editor API
    private editor: EditorKernel | undefined = undefined;

    // Compiler API
    private compiler: CompilerKernel | undefined = undefined;

    constructor(initUiMgr: UiMgrInitFn) {
        this.store = createAndBindStore(this);
        this.uiMgr = new UiMgr(this, this.store, initUiMgr);
        this.keyMgr = new KeyMgr(this.store);
        this.alertMgr = new AlertMgrImpl(this.store);
    }

    public asEdit() {
        return this;
    }

    public init() {
        console.info("initializing edit mode kernel...");
        this.uiMgr.init();
        this.keyMgr.init();

        document.title = "Celer Editor";
        this.store.dispatch(viewActions.setStageMode("edit"));
        window.addEventListener("beforeunload", (e) => {
            if (this.editor && this.editor.hasUnsavedChangesSync()) {
                e.preventDefault();
                return (e.returnValue =
                    "There are unsaved changes in the editor which will be lost. Are you sure you want to leave?");
            }
        });
    }
    public delete() {
        this.uiMgr.delete();
        this.keyMgr.delete();
    }

    public async reloadDocument() {
        const compiler = await this.ensureCompiler();
        await compiler.compile();
    }

    public async exportDocument(request: ExportRequest): Promise<ExpoDoc> {
        injectSplitTypesIntoRequest(request, this.store.getState());
        const compiler = await this.ensureCompiler();
        return await compiler.export(request);
    }

    public getEditor(): EditorKernel | undefined {
        return this.editor;
    }

    /// Get or load the compiler
    public async ensureCompiler(): Promise<CompilerKernel> {
        if (!this.compiler) {
            const { initCompiler } = await import("core/compiler");
            const compiler = initCompiler(this.store);
            this.compiler = compiler;
        }
        return this.compiler;
    }

    /// Open a project file system
    ///
    /// This function eats the error because alerts will be shown to the user
    public async openProjectFileSystem(fs: FsFileSystem): Promise<void> {
        console.info("opening file system...");

        const { editorMode } = settingsSelector(this.store.getState());
        const { write, live } = fs.capabilities;
        if (editorMode === "web") {
            // must be able to save to use web editor
            if (!write) {
                const yes = await this.alertMgr.show({
                    title: "Save not supported",
                    message:
                        "The web editor cannot be used because your browser does not support saving changes to the file system. If you wish to edit the project, you can use the External Editor workflow and have Celer load changes directly from your file system.",
                    okButton: "Use external editor",
                    cancelButton: "Cancel",
                    learnMoreLink: "/docs/route/editor/web#browser-os-support",
                });
                if (!yes) {
                    return;
                }
                this.store.dispatch(settingsActions.setEditorMode("external"));
            }
        }

        if (!live) {
            const yes = await this.alertMgr.show({
                title: "Heads up!",
                message:
                    "Your browser has limited support for file system access when opening a project from a dialog. Celer will not be able to detect new, renamed or deleted files! Please see the learn more link below for more information.",
                okButton: "Continue anyway",
                cancelButton: "Cancel",
                learnMoreLink: "/docs/route/editor/external#open-a-project",
            });
            if (!yes) {
                return;
            }
        }

        const { initEditor } = await import("core/editor");
        const editor = await initEditor(this, fs, this.store);
        this.editor = editor;
        this.updateRootPathInStore(fs);
        const compiler = await this.ensureCompiler();
        await compiler.init(editor.getFileAccess());

        // trigger a first run when loading new project
        compiler.compile();
        console.info("project opened.");
    }

    public async closeProjectFileSystem() {
        console.info("closing file system...");
        setDocument(this.store, undefined);
        this.updateRootPathInStore(undefined);
        this.editor = undefined;
        const { deleteEditor } = await import("core/editor");
        deleteEditor();
        if (this.compiler) {
            this.compiler.uninit();
        }
    }

    private updateRootPathInStore(fs: FsFileSystem | undefined) {
        this.store.dispatch(viewActions.updateFileSys(fs?.root ?? undefined));
    }
}
