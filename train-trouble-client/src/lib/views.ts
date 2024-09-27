export type View = MapView | ZoneView;

export interface MapView {
  type: "map",
  occupied: Record<string, boolean>
}

export interface ZoneView {
  type: "zone",
  name: "string",
  switches: SwitchView[],
  signals: SignalView[],
  platforms: PlatformView[],
  rates: RateView[],
  balance: number
}

export interface SwitchView {
  id: string,
  name: "string",
  direction: "left" | "right" | null
}

export interface SignalView {
  id: string,
  name: "string",
  clear: boolean | null
}

export interface PlatformView {
  trains: TrainView[]
}

export interface TrainView {
  id: string,
  name: "string",
  stopped: boolean,
  load: Record<string, number> | null
}

export interface RateView {
  id: string,
  rate: number
}
