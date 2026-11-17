// What the engine reports (04 §3.2): a copy of the last telemetry. Only the bridge event
// handlers modify it.
// TODO(T-088): the devices in use and the state of the audio.
import { create } from "zustand";
import { bridge } from "../bridge";
import type { Bridge, Telemetry, Unsubscribe } from "../bridge/bridge";

export interface EngineState {
  /** The last telemetry received (04 §6.4), or null before the first one. */
  telemetry: Telemetry | null;
}

export const useEngineStore = create<EngineState>()(() => ({ telemetry: null }));

/** Keeps the store up to date with the events of `source`. */
export function listenToBridge(source: Pick<Bridge, "onTelemetry">): Unsubscribe {
  return source.onTelemetry((telemetry) => {
    useEngineStore.setState({ telemetry });
  });
}

// The store listens to the bridge in use for as long as the interface runs.
listenToBridge(bridge);
