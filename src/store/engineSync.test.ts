import { createRhythmTrack } from "../core/commands/tracks";
import { toggleStep } from "../core/commands/patterns";
import { newProject } from "../core/factory";
import type { EngineSnapshot, ProjectDoc } from "../core/model";
import { buildEngineSnapshot } from "../core/snapshot";
import { startEngineSync } from "./engineSync";
import { useProjectStore } from "./projectStore";

/** A fake engine that records each snapshot and the moment it arrives. */
function fakeEngine() {
  const received: { snapshot: EngineSnapshot; at: number }[] = [];
  return {
    received,
    engineSetProject(snapshot: EngineSnapshot) {
      received.push({ snapshot, at: performance.now() });
      return Promise.resolve();
    },
  };
}

function addRhythmTrack(draft: ProjectDoc) {
  return createRhythmTrack(
    draft,
    (n) => `Ritmo ${n}`,
    (n) => `Patrón ${n}`,
  );
}

function currentSnapshot(): EngineSnapshot {
  const { doc, dir, missingFiles } = useProjectStore.getState();
  return buildEngineSnapshot(doc, dir, missingFiles);
}

let stop: () => void = () => undefined;

beforeEach(() => {
  vi.useFakeTimers();
  useProjectStore.getState().load(newProject(), "C:\\Temp\\carabo-1", true, []);
});

afterEach(() => {
  stop();
  vi.useRealTimers();
});

describe("engineSync", () => {
  it("after a commit, calls engineSetProject once, on the next frame", () => {
    const engine = fakeEngine();
    stop = startEngineSync(engine);

    useProjectStore.getState().commit("history.createRhythmTrack", addRhythmTrack);
    expect(engine.received).toEqual([]);

    vi.advanceTimersToNextFrame();
    expect(engine.received.map((r) => r.snapshot)).toEqual([currentSnapshot()]);

    vi.advanceTimersByTime(1000);
    expect(engine.received).toHaveLength(1);
  });

  it("sends one snapshot, the latest, for several changes in the same frame", () => {
    const engine = fakeEngine();
    stop = startEngineSync(engine);
    const { commit } = useProjectStore.getState();

    commit("history.createRhythmTrack", addRhythmTrack);
    commit("history.toggleStep", (d) => toggleStep(d, "trk-1", "pat-2", 0, 0));
    commit("history.toggleStep", (d) => toggleStep(d, "trk-1", "pat-2", 0, 4));
    vi.advanceTimersToNextFrame();

    expect(engine.received.map((r) => r.snapshot)).toEqual([currentSnapshot()]);
  });

  it("sends at most once every 33 ms", () => {
    const engine = fakeEngine();
    stop = startEngineSync(engine);
    const { commit } = useProjectStore.getState();

    commit("history.createRhythmTrack", addRhythmTrack);
    vi.advanceTimersToNextFrame();
    commit("history.toggleStep", (d) => toggleStep(d, "trk-1", "pat-2", 0, 0));
    vi.advanceTimersToNextFrame();
    expect(engine.received).toHaveLength(1);

    vi.advanceTimersByTime(33);
    expect(engine.received).toHaveLength(2);
    expect(engine.received[1]!.at - engine.received[0]!.at).toBeGreaterThanOrEqual(33);
    expect(engine.received[1]!.snapshot).toEqual(currentSnapshot());
  });

  it("sends when the folder or the missing files change", () => {
    const engine = fakeEngine();
    stop = startEngineSync(engine);

    useProjectStore.setState({ dir: "C:\\Users\\ana\\Documents\\Carabo\\Demo" });
    vi.advanceTimersByTime(100);
    useProjectStore.setState({ missingFiles: ["audio/take-0001.wav"] });
    vi.advanceTimersByTime(100);

    expect(engine.received).toHaveLength(2);
  });

  it("does not send for changes that do not sound, such as the view state", () => {
    const engine = fakeEngine();
    stop = startEngineSync(engine);

    const { view } = useProjectStore.getState();
    useProjectStore.setState({ view: { ...view, zoom: 48 } });
    vi.advanceTimersByTime(1000);

    expect(engine.received).toEqual([]);
  });

  it("sends nothing more once stopped, not even a change already pending", () => {
    const engine = fakeEngine();
    stop = startEngineSync(engine);

    useProjectStore.getState().commit("history.createRhythmTrack", addRhythmTrack);
    stop();
    vi.advanceTimersByTime(1000);
    useProjectStore.getState().commit("history.createRhythmTrack", addRhythmTrack);
    vi.advanceTimersByTime(1000);

    expect(engine.received).toEqual([]);
  });

  it("keeps sending after the engine rejects a snapshot", async () => {
    const calls: EngineSnapshot[] = [];
    stop = startEngineSync({
      engineSetProject(snapshot) {
        calls.push(snapshot);
        // eslint-disable-next-line @typescript-eslint/prefer-promise-reject-errors -- 04 §6.1: the bridge rejects with a BridgeError object.
        return Promise.reject({ code: "INVALID_SNAPSHOT", detail: "bad" });
      },
    });

    useProjectStore.getState().commit("history.createRhythmTrack", addRhythmTrack);
    await vi.advanceTimersByTimeAsync(100);
    useProjectStore.getState().commit("history.createRhythmTrack", addRhythmTrack);
    await vi.advanceTimersByTimeAsync(100);

    expect(calls).toHaveLength(2);
  });
});
