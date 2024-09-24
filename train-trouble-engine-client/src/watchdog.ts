const expireTime = 20 * 1000;
const warningTime = 20 * 1000;

export interface Watchdog {
  feed(): void,
  stop(): void,
}

export function createWatchdog(warning: () => void, expire: () => void): Watchdog {
  let warningTimeout: number | null = null;
  let expireTimeout: number | null = null;

  function start() {
    warningTimeout = setTimeout(() => { warningTimeout = null; warning(); }, warningTime);
    expireTimeout = setTimeout(() => { expireTimeout = null; expire(); }, expireTime);
  }

  function stop() {
    if (warningTimeout != null) clearTimeout(warningTimeout);
    if (expireTimeout != null) clearTimeout(expireTimeout);
  }

  function feed() {
    if (expireTimeout != null) {
      start();
      stop();
    }
  }

  start();
  return { feed, stop };
}
