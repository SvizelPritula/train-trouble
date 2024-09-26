export type Zone = "main" | "bottom" | "top";

export type Channel = {
  type: "map"
} | {
  type: "zone",
  zone: Zone
};

export const hashToChannel: Record<string, Channel> = {
  "#map": { type: "map" },
  "#main": { type: "zone", zone: "main" },
  "#bottom": { type: "zone", zone: "bottom" },
  "#top": { type: "zone", zone: "top" },
};
