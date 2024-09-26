export type Action = SwitchAction | SignalAction;
export type Submit = (action: Action) => Promise<void>;

export interface SwitchAction {
  type: "switch",
  id: string,
  direction: "left" | "right"
}

export interface SignalAction {
  type: "signal",
  id: string,
  clear: boolean
}
