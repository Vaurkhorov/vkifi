import { checkKifi, preview, meta } from "./handlers.js";

let states = Object.freeze({
  init: "init",
  info: "info",
  action: "action"
  // track: "track",
  // preview: "preview",
  // snapshot: "snapshot",
  // log: "log",
  // revert: "revert",
  // register: "register",
});
let currentState = states.init;

function updateState(state) {
  // navbar is only hidden before the repository is initialised/selected
  if (state != states.init) {
    document.querySelector("#navbar").classList.remove("hidden");
  } else if (state == states.init) {
    document.querySelector("#navbar").classList.add("hidden");
  }
  
  document.querySelector("#" + currentState).classList.add("hidden");
  currentState = state;
  document.querySelector("#" + currentState).classList.remove("hidden");
}

let repoPath = null;

let pathElement;
let outputElement;

window.addEventListener("DOMContentLoaded", () => {
  pathElement = document.querySelector("#path");
  outputElement = document.querySelector("#output");

  document.querySelector("#fetch-dir").addEventListener("submit", async (e) => {
    outputElement.textContent = "Fetching kifi...";
    e.preventDefault();
    
    let returned_path = await checkKifi(outputElement, pathElement);
    console.log(returned_path !== null);
    console.log(returned_path);
    if (returned_path !== repoPath && returned_path !== null) {
      repoPath = returned_path;
      updateState(states.info);
    }
  });

  document.querySelector("#get-preview").addEventListener("click", async (e) => {
    e.preventDefault();
    await preview(outputElement, repoPath);
  });

  document.querySelector("#get-info").addEventListener("click", async (e) => {
    e.preventDefault();
    await meta(outputElement, repoPath);
  });

});