export type View = MapView | ZoneView;

export interface MapView {
  type: "map",
  occupied: Record<string, boolean>
}

export interface ZoneView {
  type: "zone",
  switches: SwitchView[],
  signals: SignalView[],
}

export interface SwitchView {
  id: string,
  direction: "left" | "right" | null
}

export interface SignalView {
  id: string,
  clear: boolean | null
}
