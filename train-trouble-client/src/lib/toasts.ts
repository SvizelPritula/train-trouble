import { writable, type Writable } from "svelte/store";
import { SubmitError } from "train-trouble-engine-client"

const toastExpirationTime = 20 * 1000;

export interface Toast {
  message: string,
  type: "error" | "status"
}

export function toastFromError(error: any): Toast {
  let message: string = "Neznámá chyba";

  if (error instanceof SubmitError) {
    switch (error.reason) {
      case "connection_broken":
        message = "Nepodařilo se provést akci, spojení k serveru bylo přerušeno";
        break;
      case "not_connected":
        message = "Nepodařilo se provést akci, spojení k serveru je momentálně nedostupné";
        break;
      case "rejected":
        message = error.message;
        break;
    }
  }

  return { message, type: "error" };
}

export interface ToastCollection {
  toasts: Writable<Toast[]>,
  add: (toast: Toast) => void,
  dismiss: (toast: Toast) => void,
}

export function createToastCollection(): ToastCollection {
  let toasts: Writable<Toast[]> = writable([]);

  function dismiss(toast: Toast) {
    toasts.update(toasts => toasts.filter((t) => t != toast));
  }

  function add(toast: Toast) {
    toasts.update(toasts => [...toasts, toast]);
    setTimeout(() => dismiss(toast), toastExpirationTime);
  }

  return { toasts, add, dismiss };
}
