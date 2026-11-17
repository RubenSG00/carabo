// Contract of the bridge types (08 §7): each file of fixtures/bridge/ can be read as its type of
// 04 §6 (checked by the compiler) and has exactly the keys of that type.
import type { BridgeError, Telemetry } from "./bridge";
import telemetryJson from "../../fixtures/bridge/telemetry.json";
import bridgeErrorJson from "../../fixtures/bridge/bridge-error.json";

/**
 * The type that TypeScript gives to a JSON file that holds a value of type `T`: a union of texts
 * becomes `string` and a tuple becomes an array.
 */
type AsJson<T> = T extends string
  ? string
  : T extends readonly (infer Item)[]
    ? AsJson<Item>[]
    : T extends object
      ? { [Key in keyof T]: AsJson<T[Key]> }
      : T;

// The compiler rejects these assignments if a value of the file cannot be of its type, or if a
// key of the type is missing from the file.
const telemetry: AsJson<Telemetry> = telemetryJson;
const bridgeError: AsJson<BridgeError> = bridgeErrorJson;

// The compiler rejects these lists if a key of the type is missing or one is left over.
const telemetryKeys = {
  state: true,
  mode: true,
  positionFrame: true,
  patternStep: true,
  trackPeaks: true,
  masterPeak: true,
  inputPeak: true,
  xruns: true,
  recordedFrames: true,
  outputAvailable: true,
} satisfies Record<keyof Telemetry, true>;
const bridgeErrorKeys = { code: true, detail: true } satisfies Record<keyof BridgeError, true>;

const sorted = (keys: string[]) => [...keys].sort();

describe("bridge types (contract with fixtures/bridge/, 08 §7)", () => {
  it("telemetry.json has exactly the keys of Telemetry", () => {
    expect(sorted(Object.keys(telemetry))).toEqual(sorted(Object.keys(telemetryKeys)));
  });

  it("bridge-error.json has exactly the keys of BridgeError", () => {
    expect(sorted(Object.keys(bridgeError))).toEqual(sorted(Object.keys(bridgeErrorKeys)));
  });
});
