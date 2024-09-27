export type Zone = "nová-ves" | "kolnov" | "horní-mechoklaty" | "dolní-mechoklaty" | "předvořany";

export type Channel = {
  type: "map"
} | {
  type: "zone",
  zone: Zone
};

export const hashToChannel: Record<string, Channel> = {
  "#map": { type: "map" },
  "#nova-ves": { type: "zone", zone: "nová-ves" },
  "#kolnov": { type: "zone", zone: "kolnov" },
  "#horni-mechoklaty": { type: "zone", zone: "horní-mechoklaty" },
  "#dolni-mechoklaty": { type: "zone", zone: "dolní-mechoklaty" },
  "#predvorany": { type: "zone", zone: "předvořany" },
};
