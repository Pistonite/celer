// //!

// import { useWindowSize } from "core/utils";
// //import { Layout, Widget, useLocalStore } from "data/store";
// import { produce } from "immer";
// import { useCallback, useMemo, useState } from "react";
// import { Layout as LayoutData } from "react-grid-layout";

// /// The grid size
// export const GRID_SIZE = 64;
// /// Margin between cells when editing
// export const EDITING_MARGIN = 10;
// /// Maximum width of the window to be considered mobile
// const MOBILE_WIDTH_THRESHOLD = 400;

// // const adjustHeight = (layout: Widget) => {
// // 	// Make sure the top of always visible
// // 	if (layout.y < 0) {
// // 		layout.y = 0;
// // 	}
// // 	// Make sure the bottom is always visible
// // 	if (layout.y >= GRID_SIZE) {
// // 		layout.y = GRID_SIZE;
// // 	}
// // 	if (layout.y + layout.h > GRID_SIZE) {
// // 		layout.h = GRID_SIZE - layout.y;
// // 	}
// // 	// Make sure the left is always visible
// // 	if (layout.x < 0) {
// // 		layout.x = 0;
// // 	}
// // 	if (layout.x >= GRID_SIZE) {
// // 		layout.x = GRID_SIZE;
// // 	}
// // 	// Make sure the right is always visible
// // 	if (layout.x + layout.w > GRID_SIZE) {
// // 		layout.w = GRID_SIZE - layout.x;
// // 	}
// // 	// If the widget has 0 width or height, try to make it visible
// // 	if (layout.w <= 0) {
// // 		layout.w = GRID_SIZE - layout.x;
// // 	}
// // 	// If the widget has 0 width or height, try to make it visible
// // 	if (layout.h <= 0) {
// // 		layout.h = GRID_SIZE - layout.h;
// // 	}
// // 	return layout;
// // };

// /// Layout API returned by useLayoutApi hook
// export type LayoutApi = Readonly<{
// 	/// The current layout
// 	currentLayout: Layout;
// 	/// Current layout index
// 	///
// 	/// If this is 0, the layout is the default, immutable layout. Otherwise it is the index of the saved layout (starting from 1).
// 	currentLayoutIndex: number;
// 	/// If the user is editing the layout
// 	isEditingLayout: boolean;
// 	/// Start editing the layout
// 	///
// 	/// If the current layout is immutable, do nothing
// 	startEditingLayout: () => void;
// 	/// Save the current layout
// 	///
// 	/// If the current layout is immutable, do nothing
// 	saveEditingLayout: (layout: Layout) => void;
// 	/// Switch layout
// 	switchLayout: (index: number) => void;
// 	/// Duplicate the current layout
// 	duplicateCurrentLayout: () => void;
// 	/// Delete the current layout
// 	///
// 	/// If the current layout is immutable, do nothing
// 	deleteCurrentLayout: () => void;
// }>;

// // export const useLayoutApi = (): LayoutApi => {
// // 	const [isEditingLayout, setEditingLayout] = useState(false);
// // 	const [store, setStore] = useLocalStore();
// // 	const {
// // 		CurrentLayout: currentLayoutIndex,
// // 		SavedLayouts: savedLayouts,
// // 	} = store;
// // 	const { windowWidth, windowHeight } = useWindowSize();
// // 	const isViewing = true; // TODO: Implement this
// // 	const currentLayout = useMemo(() => {
// // 		if (currentLayoutIndex <= 0 || currentLayoutIndex > savedLayouts.length) {
// // 			return getDefaultViewerLayout(windowWidth, windowHeight);
// // 		}
// // 		return savedLayouts[currentLayoutIndex - 1];
// // 	}, [currentLayoutIndex, windowWidth, windowHeight, savedLayouts]);

// // 	const startEditingLayout = useCallback(() => {
// // 		if (currentLayoutIndex <= 0) {
// // 			return;
// // 		}
// // 		setEditingLayout(true);
// // 	}, [currentLayoutIndex]);

// // 	const saveEditingLayout = useCallback((layout: Layout) => {
// // 		setStore(produce(draft => {
// // 			const i = draft.CurrentLayout;
// // 			if (i <= 0 || i > draft.SavedLayouts.length) {
// // 				return;
// // 			}
// // 			draft.SavedLayouts[i - 1] = layout;
// // 		}));
// // 		setEditingLayout(false);
// // 	}, []);

// // 	const switchLayout = useCallback((index: number) => {
// // 		setStore(produce(draft => {
// // 			if (index < 0 || index > draft.SavedLayouts.length) {
// // 				return;
// // 			}
// // 			draft.CurrentLayout = index;
// // 		}));
// // 	}, []);

// // 	const duplicateCurrentLayout = useCallback(() => {
// // 		setStore(produce(draft => {
// // 			const i = draft.CurrentLayout;
// // 			if (i < 0 || i > draft.SavedLayouts.length) {
// // 				return;
// // 			}
// // 			draft.SavedLayouts.push(draft.SavedLayouts[i - 1]);
// // 		}));
// // 	}, []);

// // 	const deleteCurrentLayout = useCallback(() => {
// // 		setStore(produce(draft => {
// // 			const i = draft.CurrentLayout;
// // 			if (i <= 0 || i > draft.SavedLayouts.length) {
// // 				return;
// // 			}
// // 			draft.SavedLayouts.splice(i - 1, 1);
// // 		}));
// // 	}, []);

// // 	return {
// // 		currentLayout,
// // 		currentLayoutIndex,
// // 		isEditingLayout,
// // 		startEditingLayout,
// // 		saveEditingLayout,
// // 		switchLayout,
// // 		duplicateCurrentLayout,
// // 		deleteCurrentLayout,
// // 	};

// // };



// const getDefaultViewerLayout = (windowWidth: number, windowHeight: number): Layout => {
// 	console.log(windowWidth, windowHeight)
// 	if (windowWidth < MOBILE_WIDTH_THRESHOLD) {
// 		return {
// 			widgets: [
// 				{
// 					type: "document",
// 					x: 0,
// 					y: 0,
// 					w: GRID_SIZE,
// 					h: GRID_SIZE,
// 				},
// 			],
// 			toolbar: 0,
// 			toolbarAnchor: "top",
// 		};
// 	}
// 	if (windowWidth >= windowHeight) {
// 		// Landscape
// 		return {
// 			widgets: [
// 				{
// 					type: "document",
// 					x: 0,
// 					y: 0,
// 					w: GRID_SIZE / 2,
// 					h: GRID_SIZE,
// 				},
// 				{
// 					type: "map",
// 					x: GRID_SIZE / 2,
// 					y: 0,
// 					w: GRID_SIZE / 2,
// 					h: GRID_SIZE,
// 				},
// 			],
// 			toolbar: 0,
// 			toolbarAnchor: "top",
// 		}
// 	}
// 	// Portrait
// 	return {
// 		widgets: [
// 			{
// 				type: "document",
// 				x: 0,
// 				y: 0,
// 				w: GRID_SIZE,
// 				h: GRID_SIZE / 2,
// 			},
// 			{
// 				type: "map",
// 				x: 0,
// 				y: GRID_SIZE / 2,
// 				w: GRID_SIZE,
// 				h: GRID_SIZE / 2,
// 			},
// 		],
// 		toolbar: 0,
// 		toolbarAnchor: "top",
// 	};
// }