export type Action = SwitchAction | SignalAction | TradeAction;
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

export type TradeActionType = "buy" | "sell";

export interface TradeAction {
  type: "trade",
  action: TradeActionType,
  train: string,
  resource: string,
  amount: number
}
