// import { getUniqueSessionId } from "data/id";
// import { appendLog, canLog, LogFunction, LoggerLevel, LoggerSource } from "data/log";
// import produce from "immer";
// import { useCallback, useMemo, useState } from "react";
// import { ConnectionSessionId, DefaultConsoleSessionName, HelpSessionId, isConsoleSession, isDataSession, isOutputSession, newConsoleSession, newDataSession, newOutputSession, OutputUidx, Session } from "store/type";

// // const logHelper = (sessions: Record<string, Session>, level: LoggerLevel, source: LoggerSource, text: string) => {
// // 	for (const key in sessions) {
// // 		const session = sessions[key];
// // 		if (isConsoleSession(session)){
// // 			if(canLog(session, level, source)) {
// // 				session.data = appendLog(session.data, level, source, text);
// // 			}
// // 		}
// // 	}
// // }

// const checkSessionHelper = <T extends Session>(sessions: Record<string, Session>, sessionId: string, predicate: (session: Session) => session is T): T | undefined => {
// 	if (!(sessionId in sessions)){ 
// 		logHelper(sessions, "E", "client", `Error: Accessing "${sessionId}" which is not a Session`);
// 		return undefined; 
// 	}
// 	const session = sessions[sessionId];
// 	if (!predicate(session)) {
// 		logHelper(sessions, "E", "client", `Error: Accessing "${sessionId}" which is the wrong Session type`);
// 		return undefined;
// 	}
// 	return session;
// }

// const checkSessionHelperNonUpdate = <T extends Session>(log: LogFunction, sessions: Record<string, Session>, sessionId: string, predicate: (session: Session) => session is T): T | undefined => {
// 	if (!(sessionId in sessions)){ 
// 		log("E", "client", `Error: Accessing "${sessionId}" which is not a Session`);
// 		return undefined; 
// 	}
// 	const session = sessions[sessionId];
// 	if (!predicate(session)) {
// 		log("E", "client", `Error: Accessing "${sessionId}" which is the wrong Session type`);
// 		return undefined;
// 	}
// 	return session;
// }



// export const useSessionApi = (defaultSessions: Record<string, Session>) => {
// 	const [sessionIdMap, setSessionIdMap] = useState<Record<string, Session>>(defaultSessions);
// 	const [activeOutputSessionIds, setActiveOutputSessionIds] = useState<string[]>([]);
// 	const [serialMap, setSerialMap] = useState<string[]>([]);
	
// 	const {
// 		sessionIds,
// 	} = useMemo(()=>{
// 		const sessionIds = Object.keys(sessionIdMap);
// 		return {
// 			sessionIds
// 		};
// 	}, [sessionIdMap]);

// 	const log = useCallback((level: LoggerLevel, source: LoggerSource, text: string) => {
// 		// See if update is needed to prevent useless rerender
// 		if (!text){
// 			return;
// 		}
// 		setSessionIdMap(produce(draft=>{
// 			logHelper(draft, level, source, text);
// 		}));
// 	}, []);

// 	const setSessionName = useCallback((sessionId: string, newName: string)=>{
// 		setSessionIdMap(produce(draft=>{
// 			if (sessionId in draft) {
// 				draft[sessionId].name = newName;
// 			}
// 		}));
// 	}, []);
    
// 	const createConsoleSession = useCallback((name: string)=>{
// 		const id = getUniqueSessionId();
// 		setSessionIdMap(produce((draft)=>{
// 			logHelper(draft, "I", "client", `Creating new Console Session "${name}"`);
// 			draft[id] = newConsoleSession(name);
// 		}));
// 		return id;
// 	}, []);

// 	const createDataSession = useCallback((name: string)=>{
// 		const id = getUniqueSessionId();
// 		setSessionIdMap(produce((draft)=>{
// 			logHelper(draft, "I", "client", `Creating new Data Session "${name}"`);
// 			draft[id] = newDataSession(name);
// 		}));
// 		return id;
// 	}, []);

// 	const createOutputSession = useCallback((name: string)=>{
// 		const id = getUniqueSessionId();
// 		setSessionIdMap(produce((draft)=>{
// 			logHelper(draft, "I", "client", `Creating new Output Session "${name}"`);
// 			draft[id] = newOutputSession(name);
// 		}));
// 		return id;
// 	}, []);

// 	const canCloseSession = useCallback((sessionId: string) => {
// 		if (!(sessionId in sessionIdMap)) {
// 			return false;
// 		}
// 		if (sessionId === ConnectionSessionId) {
// 			return false;
// 		}
// 		if (sessionId === HelpSessionId) {
// 			return false;
// 		}
// 		// Cannot close activate sessions
// 		return sessionIdMap[sessionId].uidx < 0 && !activeOutputSessionIds.includes(sessionId);
// 	}, [sessionIdMap, activeOutputSessionIds]);

// 	const closeSession = useCallback((sessionId: string)=>{
// 		setSessionIdMap(produce(draft=>{
// 			if (!(sessionId in draft)){
// 				return;
// 			}
// 			if (!canCloseSession(draft[sessionId].name)){
// 				logHelper(draft, "E", "client", `Error: Session "${draft[sessionId].name}" is not allowed to be closed.`)
// 				return;
// 			}

// 			logHelper(draft, "I", "client", `Closing Session "${draft[sessionId].name}"`);
// 			delete draft[sessionId];
// 		}));
		
// 	}, [canCloseSession]);

// 	const closeAllSessions = useCallback(() => {
// 		setSessionIdMap(produce((draft)=>{
// 			for (const key in draft){
// 				if (canCloseSession(key)) {
// 					logHelper(draft, "I", "client", `Closing Session "${key}"`);
// 					delete draft[key];
// 				}
				
// 			}
			
// 		}));
// 	}, [canCloseSession]);

// 	const editData = useCallback((sessionId: string, newData: Record<string, unknown>) => {
// 		setSessionIdMap(produce(draft=>{
// 			const session = checkSessionHelper(draft, sessionId, isDataSession);
// 			if (session){
// 				logHelper(draft, "D", "client", `Setting ${session.name} data = ${JSON.stringify(newData)}`);
// 				session.obj = newData;
// 			}
// 		}));
// 	}, []);

// 	const setConsoleLogLevel = useCallback((sessionId: string, level: LoggerLevel) => {
// 		setSessionIdMap(produce(draft=>{
// 			const session = checkSessionHelper(draft, sessionId, isConsoleSession);
// 			if (session) {
// 				logHelper(draft, "I", "client", `Setting ${session.name} logging level to ${level}`);
// 				session.level = level;
// 			}
// 		}));
// 	}, []);

// 	const setConsoleLogSource = useCallback((sessionId: string, source: LoggerSource, enable: boolean) => {
// 		setSessionIdMap(produce(draft=>{
// 			const session = checkSessionHelper(draft, sessionId, isConsoleSession);
// 			if (session) {
// 				logHelper(draft, "I", "client", `Setting ${session.name} to log ${source}: ${enable}`);
// 				session.enabled[source] = enable;
// 			}
// 		}));
// 	}, []);

// 	const setSerial = useCallback((sessionId: string, serialId: number) => {
// 		setSerialMap(produce(draft=>{
// 			draft[serialId] = sessionId;
// 		}));
// 	}, []);

// 	const activateOutput = useCallback((serialId: number, remoteSessionId: number)=>{
// 		// Find or create output
// 		const dataSessionId = serialMap[serialId];
// 		const dataSession = checkSessionHelperNonUpdate(log, sessionIdMap, dataSessionId, isDataSession);
// 		if(!dataSession){
// 			return;
// 		}
// 		const preferredOutputName = dataSession.obj.Output;
		
// 		const name = (preferredOutputName && typeof preferredOutputName === "string") ? preferredOutputName : "New Output";
// 		const outputSessionId = sessionIds.find((id)=>{
// 			const session = sessionIdMap[id];
// 			return isOutputSession(session) && session.name === preferredOutputName && !activeOutputSessionIds.includes(id);
// 		}) ?? getUniqueSessionId();

// 		// Set uidx
// 		setSessionIdMap(produce(draft=>{
// 			// Create output session if needed
// 			if (!(outputSessionId in draft)) {
// 				draft[outputSessionId] = newOutputSession(name);
// 			}

// 			logHelper(draft, "I", "client", `Activating output "${name}" for "${dataSession.name}"`);
// 			draft[dataSessionId].uidx = remoteSessionId;

// 		}));
// 		// Add to active output sessions
// 		setActiveOutputSessionIds(produce(draft=>{
// 			draft[remoteSessionId] = outputSessionId;
// 		}));
// 	}, [
// 		log,
// 		serialMap,
// 		sessionIdMap,
// 		sessionIds,
// 		activeOutputSessionIds
// 	]);

// 	const deactivateOutput = useCallback((remoteSessionId: number)=>{
// 		setSessionIdMap(produce(draft=>{
// 			for (const id in draft){
// 				const session = draft[id];
// 				if (session.uidx === remoteSessionId){
// 					logHelper(draft, "I", "client", `Deactivating Session "${session.name}"`);
// 					session.uidx = -1;
// 					break;
// 				}
// 			}
// 		}));
// 		setActiveOutputSessionIds(produce(draft=>{
// 			delete draft[remoteSessionId];
// 		}));
// 	}, []);

// 	const updateOutput = useCallback((remoteSessionId: number, data: Record<string, unknown>)=>{
// 		const outputSessionId = activeOutputSessionIds[remoteSessionId];
// 		if(!outputSessionId){
// 			log("E", "client", `No output session for remote session ${remoteSessionId}`)
// 			return;
// 		}
// 		setSessionIdMap(produce(draft=>{
// 			const session = draft[outputSessionId];
// 			if(!isOutputSession(session)){
// 				logHelper(draft, "E", "client", `Session ${outputSessionId} is not an output session`);
// 				return;
// 			}
// 			if (session){
// 				session.obj = data;
// 			}
// 		}));
// 	}, [activeOutputSessionIds, log]);


// 	return useMemo(()=>({
// 		sessions: sessionIdMap,
// 		sessionIds,
// 		activeOutputSessionIds,
// 		log,
// 		createConsoleSession,
// 		createDataSession,
// 		createOutputSession,
// 		canCloseSession,
// 		closeSession,
// 		closeAllSessions,
// 		setSessionName,
// 		editData,
// 		setConsoleLogLevel,
// 		setConsoleLogSource,
// 		setSerial,
// 		activateOutput,
// 		deactivateOutput,
// 		updateOutput,
// 	}), [
// 		sessionIdMap,
// 		sessionIds,
// 		activeOutputSessionIds,
// 		log,
// 		createConsoleSession,
// 		createDataSession,
// 		createOutputSession,
// 		canCloseSession,
// 		closeSession,
// 		closeAllSessions,
// 		setSessionName,
// 		editData,
// 		setConsoleLogLevel,
// 		setConsoleLogSource,
// 		setSerial,
// 		activateOutput,
// 		deactivateOutput,
// 		updateOutput
// 	]);
// };

// export type SessionApi = Readonly<ReturnType<typeof useSessionApi>>;