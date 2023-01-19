const { invoke } = window.__TAURI__.tauri;


window.addEventListener("DOMContentLoaded", () => {
});

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

window.greet = greet;
