const delay = 5 * 1000;

export type Action = () => void;

export function reconnect(connect: (retry: Action) => Action): Action {
  let stopped = false;

  let disconnect: (() => void) | null = null;
  let retryTimeout: number | null = null;

  function createConnection() {
    retryTimeout = null;
    disconnect = connect(limitCallsToOne(retry));
  }

  function retry() {
    if (stopped) return;
    if (disconnect != null) disconnect();

    disconnect = null;
    retryTimeout = setTimeout(createConnection, delay);
  }

  createConnection();

  return () => {
    if (stopped) return;
    stopped = true;

    if (disconnect != null) disconnect();
    if (retryTimeout != null) clearTimeout(retryTimeout);
  };
}

function limitCallsToOne(func: () => void): () => void {
  let called = false;

  return () => {
    if (called) return;
    called = true;

    func();
  }
}
