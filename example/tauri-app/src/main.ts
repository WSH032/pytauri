import { invoke } from "@tauri-apps/api/core";
import { pyInvoke } from "tauri-plugin-pytauri-api";
// or if tauri config `app.withGlobalTauri = true`:
//
// ```js
// const { pyInvoke } = window.__TAURI__.pytauri;
// ```

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

interface Greeting {
  message: string;
}

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    const rsGreeting = await invoke<string>("greet", {
      name: greetInputEl.value,
    });
    const pyGreeting = await pyInvoke<Greeting>("greet", {
      name: greetInputEl.value,
    });
    greetMsgEl.textContent = rsGreeting + "\n" + pyGreeting.message;
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
