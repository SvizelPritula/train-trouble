import { writable, type Readable, type Writable } from "svelte/store";
import { reconnect } from "./reconnect";
import { createWatchdog } from "./watchdog";

export interface Connection {
  stop(): void,
}

const path = "/api/sync";

export function connect<Channel, View>(channel: Channel, onNewView: (view: View) => void): Connection {
  function processMessage(message: IncomingMessage<View>) {
    if (message.type == "state") {
      onNewView(message.state);
    } else if (message.type == "error") {
      console.error(`Server reported an error: ${message.error}`);
    }
  }

  let stop = reconnect(retry => {
    let socket = new WebSocket(new URL(path, window.location.href));

    let watchdog = createWatchdog(() => {
      socket.send(JSON.stringify({ type: "ping" } as OutgoingMessage<Channel>));
    }, retry);

    socket.addEventListener("open", () => {
      socket.send(JSON.stringify({ type: "login", channel } as OutgoingMessage<Channel>));
    });

    socket.addEventListener("message", event => {
      if (typeof event.data == "string")
        processMessage(JSON.parse(event.data) as IncomingMessage<View>);
    });

    socket.addEventListener("error", event => console.error(event));
    socket.addEventListener("close", retry);

    return () => {
      socket.close();
      watchdog.stop();
    };
  });

  return { stop };
}

type OutgoingMessage<Channel> = {
  type: "ping"
} | {
  type: "login",
  channel: Channel
};

type IncomingMessage<View> = {
  type: "ping"
} | {
  type: "state",
  state: View
} | {
  type: "error",
  error: string
};
