import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export interface SyncEvent<T = any> {
  payload: T;
}

export interface SyncEventHandlers {
  onStarted: (payload: any) => void;
  onProgress: (payload: any) => void;
  onFinished: (payload: any) => void;
  onError: (payload: any) => void;
}

export function subscribeSyncEvents(handlers: SyncEventHandlers): () => void {

  const unlisten: UnlistenFn[] = [];

  async function setup(): Promise<void> {

    unlisten.push(
      await listen<SyncEvent>("sync:started", e => handlers.onStarted(e.payload))
    );

    unlisten.push(
      await listen<SyncEvent>("sync:progress", e => handlers.onProgress(e.payload))
    );

    unlisten.push(
      await listen<SyncEvent>("sync:finished", e => handlers.onFinished(e.payload))
    );

    unlisten.push(
      await listen<SyncEvent>("sync:error", e => handlers.onError(e.payload))
    );
  }

  setup();

  return () => {
    unlisten.forEach(fn => fn());
  };
}