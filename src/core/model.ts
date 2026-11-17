// TODO(T-009): the complete types of 05 §3.1 and §10.1. Until then the bridge carries the
// engine snapshot without looking inside it.
export type EngineSnapshot = Readonly<Record<string, unknown>>;
