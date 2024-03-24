/// Implementation of the Kernel in VIEW mode
import { Void } from "pure/result";

import {
    getRawPluginOptionsForTitle,
    injectSplitTypesIntoRequest,
    setDocument,
} from "core/doc";
import {
    AppStore,
    settingsSelector,
    viewActions,
} from "core/store";
import { AlertMgr, SerialEvent, SerialEventCancelToken, consoleKernel as console, sleep } from "low/utils";
import { ExpoDoc, ExportRequest } from "low/celerc";

import { Kernel } from "./Kernel";
import { UiMgr, UiMgrInitFn } from "./UiMgr";
import { createAndBindStore } from "./store";
import { KeyMgr } from "./KeyMgr";
import { AlertMgrImpl } from "./AlertMgr";
import {
    getPreloadedDocumentTitle,
    loadDocument,
    sendExportRequest,
} from "./server";

export class KernelViewImpl implements Kernel {
    private store: AppStore;
    private uiMgr: UiMgr;
    private keyMgr: KeyMgr;
    public readonly alertMgr: AlertMgr;

    private preloadedDocumentTitle: string | undefined;
    private loadEvent: SerialEvent;

    constructor(initUiMgr: UiMgrInitFn) {
        this.store = createAndBindStore(this);
        this.uiMgr = new UiMgr(this, this.store, initUiMgr);
        this.keyMgr = new KeyMgr(this.store);
        this.alertMgr = new AlertMgrImpl(this.store);
        this.preloadedDocumentTitle = getPreloadedDocumentTitle();
        this.loadEvent = new SerialEvent((current, latest) => {
            console.warn(`cancelling previous load (current=${current}, latest=${latest})`);
        });
    }

    public asEdit(): never {
        throw new Error("Cannot switch to edit mode from view mode");
    }

    public init() {
        console.info("initializing view mode kernel...");
        this.uiMgr.init();
        this.keyMgr.init();

        if (this.preloadedDocumentTitle) {
            document.title = this.preloadedDocumentTitle;
        }
        setTimeout(() => {
            this.reloadDocument();
        }, 0);
        this.store.dispatch(viewActions.setStageMode("view"));
    }

    public delete() {
        this.uiMgr.delete();
        this.keyMgr.delete();
    }

    /// Reload the document from the server based on the current URL
    ///
    /// In view mode, a next load will cancel previous loads so the latest state
    /// is guaranteed to be loaded.
    public async reloadDocument() {
        await this.loadEvent.run(this.reloadDocumentInternal.bind(this));
    }

    private async reloadDocumentInternal(serial: number, shouldCancel: () => Void<SerialEventCancelToken>) {
        this.store.dispatch(viewActions.setCompileInProgress(true));
        // let UI update
        await sleep(0);

        let retry = true;
        while (retry) {
            let cancel = shouldCancel();
            if (cancel.err) {
                return cancel;
            }

            console.info(`reloading document from server (serial=${serial})`);
            const settings = settingsSelector(this.store.getState());
            const pluginOptions = getRawPluginOptionsForTitle(
                settings,
                this.preloadedDocumentTitle,
            );
            const result = await loadDocument(pluginOptions);

            cancel = shouldCancel();
            if (cancel.err) {
                return cancel;
            }

            if (result.type === "failure") {
                setDocument(this.store, undefined);
                console.info("failed to load document from server");
                console.error(result.data);
                retry = await this.alertMgr.show({
                    title: "Failed to load route",
                    message: result.data,
                    learnMoreLink:
                        "/docs/route/publish#viewing-the-route-on-celer",
                    okButton: "Retry",
                    cancelButton: "Cancel",
                });
                if (!retry) {
                    await this.alertMgr.show({
                        title: "Load cancelled",
                        message:
                            'You can retry at any time by refreshing the page, or by clicking "Reload Document" from the toolbar.',
                        okButton: "Got it",
                        cancelButton: "",
                    });
                    break;
                }
                console.warn("retrying in 1s...");
                await sleep(1000);
                continue;
            }
            console.info("received document from server");
            const doc = result.data;
            try {
                const { title, version } = doc.execDoc.project;
                if (!title) {
                    document.title = "Celer Viewer";
                } else if (!version) {
                    document.title = title;
                } else {
                    document.title = `${title} - ${version}`;
                }
            } catch (e) {
                console.warn("failed to set document title");
                console.error(e);
                document.title = "Celer Viewer";
            }
            setDocument(this.store, doc);
            break;
        }
        this.store.dispatch(viewActions.setCompileInProgress(false));

        return { val: undefined };
    }

    public exportDocument(request: ExportRequest): Promise<ExpoDoc> {
        injectSplitTypesIntoRequest(request, this.store.getState());
        const settings = settingsSelector(this.store.getState());
        const pluginOptions = getRawPluginOptionsForTitle(
            settings,
            this.preloadedDocumentTitle,
        );
        return sendExportRequest(pluginOptions, request);
    }
}
