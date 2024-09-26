<script lang="ts">
  import type { Toast } from "../lib/toasts";

  export let toast: Toast;
  export let dismiss: (() => void) | null = null;
</script>

<div
  class="toast"
  class:error={toast.type == "error"}
  class:status={toast.type == "status"}
  role="alert"
>
  <div class="message">{toast.message}</div>

  {#if dismiss != null}
    <button on:click={dismiss} aria-label="Zavřít">
      <svg viewBox="0 0 6 6" aria-hidden="true">
        <line x1="1" y1="1" x2="5" y2="5" stroke="currentColor" />
        <line x1="1" y1="5" x2="5" y2="1" stroke="currentColor" />
      </svg>
    </button>
  {/if}
</div>

<style>
  .toast {
    padding: 0.25rem 1rem;
    width: 100%;

    display: flex;
    flex-direction: row;
    align-items: center;
  }

  .error {
    background-color: #b00;
  }

  .status {
    background-color: #00b;
    text-align: center;
  }

  .message {
    flex-grow: 1;
  }

  .toast button {
    background-color: transparent;
    border: none;
    color: #bbb;

    height: 1rem;
    padding: 0;
  }

  .toast button:hover {
    color: #fff;
  }

  .toast svg {
    height: 100%;
  }
</style>
