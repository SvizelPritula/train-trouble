import { reconnect } from "./reconnect";
import { createWatchdog } from "./watchdog";

export interface Connection<Action> {
  stop(): void,
  submit(action: Action): Promise<void>,
}

const path = "/api/sync";

export function connect<Channel, View, Action>(
  channel: Channel,
  onNewView: (view: View) => void,
  onConnectionChange: (connected: boolean) => void
): Connection<Action> {
  let pendingActions: Map<number, [() => void, (reason: any) => void]> = new Map();
  let currentSocket: WebSocket | null = null;
  let nextId: number = 0;

  function processMessage(message: IncomingMessage<View>) {
    if (message.type == "state") {
      onNewView(message.state);
    } else if (message.type == "confirm") {
      let callbacks = pendingActions.get(message.id);
      pendingActions.delete(message.id);

      if (callbacks == null)
        return;

      let [resolve, reject] = callbacks;

      if (message.error == null)
        resolve();
      else
        reject(new SubmitError("rejected", message.error))
    } else if (message.type == "error") {
      console.error(`Server reported an error: ${message.error}`);
    }
  }

  let stop = reconnect(retry => {
    let socket = new WebSocket(new URL(path, window.location.href));
    currentSocket = socket;
    onConnectionChange(true);

    let watchdog = createWatchdog(() => {
      socket.send(JSON.stringify({ type: "ping" } as OutgoingMessage<Channel, Action>));
    }, retry);

    socket.addEventListener("open", () => {
      socket.send(JSON.stringify({ type: "login", channel } as OutgoingMessage<Channel, Action>));
    });

    socket.addEventListener("message", event => {
      watchdog.feed();

      if (typeof event.data == "string")
        processMessage(JSON.parse(event.data) as IncomingMessage<View>);
    });

    socket.addEventListener("error", event => console.error(event));
    socket.addEventListener("close", retry);

    return () => {
      socket.close();
      watchdog.stop();

      currentSocket = null;
      onConnectionChange(false);

      for (var [resolve, reject] of pendingActions.values()) {
        reject(new SubmitError("connection_broken"));
      }

      pendingActions = new Map();
    };
  });

  function submit(action: Action): Promise<void> {
    return new Promise((resolve, reject) => {
      let id = nextId++;

      if (currentSocket == null)
        return reject(new SubmitError("not_connected"));

      currentSocket.send(JSON.stringify({ type: "submit", id, action } as OutgoingMessage<Channel, Action>));

      pendingActions.set(id, [resolve, reject]);
    });
  }

  return { stop, submit };
}

type OutgoingMessage<Channel, Action> = {
  type: "ping"
} | {
  type: "login",
  channel: Channel
} | {
  type: "submit",
  id: number,
  action: Action
};

type IncomingMessage<View> = {
  type: "ping"
} | {
  type: "state",
  state: View
} | {
  type: "confirm",
  id: number,
  error: string | null
} | {
  type: "error",
  error: string
};

export type SubmitErrorReason = "not_connected" | "connection_broken" | "rejected";

export class SubmitError extends Error {
  reason: SubmitErrorReason;

  constructor(reason: SubmitErrorReason, message?: string) {
    super(message ?? reason);
    this.reason = reason;
    this.name = "SubmitError";
  }
}
