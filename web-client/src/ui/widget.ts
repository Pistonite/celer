// import produce from "immer";
// import { useCallback, useEffect, useMemo, useState } from "react";
// import { isConsoleSession, Session, Widget, WidgetView } from "store/type";
// import { SessionApi } from "./session";

// const splitView = (view: WidgetView): [WidgetView, WidgetView] => {
// 	const splitLeftRight = view.w > view.h;
// 	const newW = splitLeftRight ? Math.floor(view.w/2) : view.w;
// 	const newH = splitLeftRight ? view.h : Math.floor(view.h/2);
// 	return [
// 		{
// 			x: view.x,
// 			y: view.y,
// 			w: newW,
// 			h: newH
// 		},{
// 			x: splitLeftRight ? view.x + newW : view.x,
// 			y: splitLeftRight ? view.y : view.y + newH,
// 			w: splitLeftRight ? view.w - newW : view.w,
// 			h: splitLeftRight ? view.h : view.h - newH
// 		}
// 	];
// };

// export const useWidgetApi = ({
// 	log,
// 	sessions,
// }: SessionApi, defaultWidgets: Widget[]) => {
// 	const [widgets, setWidgets] = useState<Widget[]>(defaultWidgets);

// 	// Close widgets that are binded to expired sessions
// 	useEffect(()=>{
// 		if (widgets.find(widget=>!(widget.sessionId in sessions))) {
// 			setWidgets(prevWidgets=>{
// 				return prevWidgets.filter((widget, i)=>{
// 					if (widget.sessionId in sessions){
// 						return true;
// 					}
// 					log("I", "client", `Closing Widget ${i} with expired Session (id=${widget.sessionId})`);
// 					return false;
// 				});
// 			});
// 		}
// 	}, [widgets, sessions, log]);

// 	const setWidgetSession = useCallback((widgetId: number, sessionId: string) => {
// 		setWidgets(produce(draft=>{
// 			if(draft[widgetId] === undefined){
// 				log("E", "client", `Cannot set session: invalid widget id ${widgetId}`);
// 				return;
// 			}
// 			log("I", "client", `Binding Widget ${widgetId} to Session (id=${sessionId})"`);
// 			draft[widgetId].sessionId = sessionId;
// 		}));
// 	}, [log]);

// 	const setWidgetTheme = useCallback((widgetId: number, theme: string | undefined) => {
// 		setWidgets(produce(draft=>{
// 			if(draft[widgetId] === undefined){
// 				log("E", "client", `Cannot set session: invalid widget id ${widgetId}`);
// 				return;
// 			}
// 			draft[widgetId].theme = theme;
// 			log("D", "client", `Setting Widget ${widgetId} theme = "${theme}"`);
// 		}));
// 	}, [log]);

// 	const splitWidget = useCallback((widgetId: number, sessionId: string) => {
// 		setWidgets(produce(draft=>{
// 			if(draft[widgetId] === undefined){
// 				log("E", "client", `Cannot split widget: invalid widget id ${widgetId}`);
// 				return;
// 			}
// 			const [newLayout1, newLayout2] = splitView(draft[widgetId].layout);
// 			const newWidget = {
// 				theme: draft[widgetId].theme,
// 				layout: newLayout2,
// 				sessionId,
// 			};
// 			draft[widgetId].layout = newLayout1;
// 			draft.push(newWidget);
// 		}));

// 	}, [log]);

// 	const closeWidget = useCallback((widgetId: number) => {
// 		setWidgets(produce(draft=>{
// 			if(draft[widgetId] === undefined){
// 				log("E", "client", `Cannot close widget: invalid widget id ${widgetId}`);
// 				return;
// 			}
// 			log("I", "client", `Closing Widget ${widgetId}`);
// 			draft.splice(widgetId, 1);
// 		}));
// 	}, [log]);

// 	return useMemo(()=>({
// 		widgets,
// 		setWidgets,
// 		setWidgetSession,
// 		setWidgetTheme,
// 		splitWidget,
// 		closeWidget
// 	}), [
// 		widgets,
// 		setWidgets,
// 		setWidgetSession,
// 		setWidgetTheme,
// 		splitWidget,
// 		closeWidget
// 	]);
// };

// export type WidgetApi = Readonly<ReturnType<typeof useWidgetApi>>;