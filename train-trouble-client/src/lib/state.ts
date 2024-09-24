export type Channel = "view" | "change" | "reset";

export type View = {
    type: "value",
    value: number
} | {
    type: "change"
} | {
    type: "reset"
};

export type Action = "increment" | "decrement" | "reset";
