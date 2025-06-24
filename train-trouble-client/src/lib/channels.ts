export type Zone = "nová-ves" | "kolnov" | "horní-mechoklaty" | "dolní-mechoklaty" | "předvořany";
export type Team = "red" | "blue";

export type Channel = {
  type: "map"
} | {
  type: "zone",
  zone: Zone,
  team: Team,
};

export type Route = {
  type: "map"
} | {
  type: "zone",
  zone: Zone
};

export const hashToRoute: Record<string, Route> = {
  "#map": { type: "map" },
  "#nova-ves": { type: "zone", zone: "nová-ves" },
  "#kolnov": { type: "zone", zone: "kolnov" },
  "#horni-mechoklaty": { type: "zone", zone: "horní-mechoklaty" },
  "#dolni-mechoklaty": { type: "zone", zone: "dolní-mechoklaty" },
  "#predvorany": { type: "zone", zone: "předvořany" },
};

export interface TeamInfo {
  id: Team,
  name: string,
  color: string,
}

export const teams: TeamInfo[] = [
  { id: "red", name: "Červení", color: "#a00" },
  { id: "blue", name: "Modří", color: "#00a" }
];
