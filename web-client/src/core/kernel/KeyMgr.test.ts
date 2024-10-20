import { describe, it, expect, vi } from "vitest";

import type { AppStore } from "core/store";

import { KeyMgr } from "./KeyMgr";

const createMockStore = (initialState: unknown) => {
    return {
        getState: () => initialState,
        subscribe: vi.fn(),
        dispatch: vi.fn(),
    } as unknown as AppStore;
};

const getAttr = (obj: unknown, member: string) => {
    expect(obj).toHaveProperty(member);
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    return (obj as any)[member];
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const setAttr = (obj: unknown, member: string, value: any) => {
    expect(obj).toHaveProperty(member);
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (obj as any)[member] = value;
};

describe("ui/doc/KeyMgr", () => {
    const setupTest = (editingKeyBinding?: string) => {
        const mockStore = createMockStore({
            view: {
                editingKeyBinding,
                currentSection: 1,
                currentLine: 2,
            },
            document: {
                document: {
                    loaded: true,
                    route: [
                        { lines: { length: 5 } },
                        { lines: { length: 6 } },
                        { lines: { length: 7 } },
                    ],
                },
            },
            settings: {
                prevLineKey: ["a"],
                nextLineKey: ["d", "e"],
                prevSplitKey: ["a", "b", "c"],
                nextSplitKey: ["d", "e", "f"],
            },
        });
        return {
            store: mockStore,
            keyMgr: new KeyMgr(mockStore),
        };
    };

    describe("onKeyDown", () => {
        it("should not add to current stroke if repeat", () => {
            const { keyMgr } = setupTest();
            keyMgr.onKeyDown("x");
            keyMgr.onKeyDown("x");
            expect(getAttr(keyMgr, "currentStrokes")).toEqual(["x"]);

            const { keyMgr: keyMgr2 } = setupTest("editing");
            keyMgr2.onKeyDown("x");
            keyMgr2.onKeyDown("x");
            expect(getAttr(keyMgr2, "currentStrokes")).toEqual(["x"]);
        });

        it("editing: should add to current strokes", () => {
            const { keyMgr } = setupTest("editing");
            expect(getAttr(keyMgr, "currentStrokes")).toEqual([]);
            keyMgr.onKeyDown("x");
            expect(getAttr(keyMgr, "currentStrokes")).toEqual(["x"]);
        });

        it("editing: should not detect current binding", () => {
            const { store, keyMgr } = setupTest("editing");
            keyMgr.onKeyDown("a");
            expect(store.dispatch).not.toHaveBeenCalled();
        });

        it("not editing: should add to current strokes", () => {
            const { keyMgr } = setupTest();
            keyMgr.onKeyDown("x");
            expect(getAttr(keyMgr, "currentStrokes")).toEqual(["x"]);
        });

        it("not editing: should detect binding", () => {
            const { store, keyMgr } = setupTest();
            keyMgr.onKeyDown("a");
            expect(getAttr(keyMgr, "currentStrokes")).toEqual(["a"]);
            expect(getAttr(keyMgr, "lastDetected")).toEqual(["a"]);
            expect(store.dispatch).toHaveBeenCalledWith({
                type: "view/setDocLocation",
                payload: { section: 1, line: 1 },
            });
        });

        it("not editing: should detect binding, multiple strokes", () => {
            const { store, keyMgr } = setupTest();
            keyMgr.onKeyDown("d");
            expect(store.dispatch).not.toHaveBeenCalled();
            keyMgr.onKeyDown("e");
            expect(getAttr(keyMgr, "lastDetected")).toEqual(["d", "e"]);
            expect(store.dispatch).toHaveBeenCalledWith({
                type: "view/setDocLocation",
                payload: { section: 1, line: 3 },
            });
        });
    });

    describe("onKeyUp", () => {
        it("editing: should set binding", () => {
            const { store, keyMgr } = setupTest("editing");
            setAttr(keyMgr, "currentStrokes", ["a"]);
            keyMgr.onKeyUp("a");
            expect(store.dispatch).toHaveBeenCalledWith({
                type: "view/setEditingKeyBinding",
                payload: undefined,
            });
            expect(store.dispatch).toHaveBeenCalledWith({
                type: "settings/setDocKeyBinding",
                payload: {
                    name: "editing",
                    value: ["a"],
                },
            });
        });
        it("editing: should set binding, multiple keys, release last", () => {
            const { store, keyMgr } = setupTest("editing");
            setAttr(keyMgr, "currentStrokes", ["a", "b"]);
            keyMgr.onKeyUp("b");
            expect(store.dispatch).toHaveBeenCalledWith({
                type: "view/setEditingKeyBinding",
                payload: undefined,
            });
            expect(store.dispatch).toHaveBeenCalledWith({
                type: "settings/setDocKeyBinding",
                payload: {
                    name: "editing",
                    value: ["a", "b"],
                },
            });
        });
        it("editing: should set binding, multiple keys, release non last", () => {
            const { store, keyMgr } = setupTest("editing");
            setAttr(keyMgr, "currentStrokes", ["a", "b"]);
            keyMgr.onKeyUp("a");
            expect(store.dispatch).toHaveBeenCalledWith({
                type: "view/setEditingKeyBinding",
                payload: undefined,
            });
            expect(store.dispatch).toHaveBeenCalledWith({
                type: "settings/setDocKeyBinding",
                payload: {
                    name: "editing",
                    value: ["a", "b"],
                },
            });
        });
        it("editing: should update current strokes", () => {
            const { keyMgr } = setupTest("editing");
            setAttr(keyMgr, "currentStrokes", ["a", "b"]);
            keyMgr.onKeyUp("a");
            expect(getAttr(keyMgr, "currentStrokes")).toEqual(["b"]);
        });
        it("not editing: should update current strokes", () => {
            const { keyMgr } = setupTest();
            setAttr(keyMgr, "currentStrokes", ["a", "b"]);
            keyMgr.onKeyUp("b");
            expect(getAttr(keyMgr, "currentStrokes")).toEqual(["a"]);
        });
        it("not editing: should not clear last detected when still detected", () => {
            const { keyMgr } = setupTest();
            setAttr(keyMgr, "currentStrokes", ["a", "b", "c"]);
            setAttr(keyMgr, "lastDetected", ["a", "b"]);
            keyMgr.onKeyUp("c");
            expect(getAttr(keyMgr, "lastDetected")).toEqual(["a", "b"]);
            setAttr(keyMgr, "currentStrokes", ["c", "a", "b"]);
            setAttr(keyMgr, "lastDetected", ["a", "b"]);
            keyMgr.onKeyUp("c");
            expect(getAttr(keyMgr, "lastDetected")).toEqual(["a", "b"]);
            setAttr(keyMgr, "currentStrokes", ["a", "c", "b"]);
            setAttr(keyMgr, "lastDetected", ["a", "b"]);
            keyMgr.onKeyUp("c");
            expect(getAttr(keyMgr, "lastDetected")).toEqual(["a", "b"]);
        });
        it("not editing: should clear last detected when no longer detected", () => {
            const { keyMgr } = setupTest();
            setAttr(keyMgr, "currentStrokes", ["a", "b", "c"]);
            setAttr(keyMgr, "lastDetected", ["a", "b", "c"]);
            keyMgr.onKeyUp("c");
            expect(getAttr(keyMgr, "lastDetected")).toEqual([]);
            expect(getAttr(keyMgr, "currentStrokes")).toEqual(["a", "b"]);
            setAttr(keyMgr, "currentStrokes", ["a", "b", "c"]);
            setAttr(keyMgr, "lastDetected", ["a", "b", "c"]);
            keyMgr.onKeyUp("b");
            expect(getAttr(keyMgr, "lastDetected")).toEqual([]);
            expect(getAttr(keyMgr, "currentStrokes")).toEqual(["a", "c"]);
            setAttr(keyMgr, "currentStrokes", ["a", "b", "c"]);
            setAttr(keyMgr, "lastDetected", ["a", "b", "c"]);
            keyMgr.onKeyUp("a");
            expect(getAttr(keyMgr, "lastDetected")).toEqual([]);
            expect(getAttr(keyMgr, "currentStrokes")).toEqual(["b", "c"]);
        });
    });
});
