export type View = MapView | ZoneView;

export interface MapView {
  type: "map",
  occupied: Record<string, boolean>
}

export interface ZoneView {
  type: "zone",
  switches: SwitchView[],
  signals: SignalView[],
  platforms: PlatformView[]
}

export interface SwitchView {
  id: string,
  direction: "left" | "right" | null
}

export interface SignalView {
  id: string,
  clear: boolean | null
}

export interface PlatformView {
  trains: TrainView[]
}

export interface TrainView {
  id: string,
  stopped: boolean
}
