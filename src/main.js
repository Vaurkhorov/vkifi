import { checkKifi, preview, meta, log } from "./handlers.js";

let states = Object.freeze({
	init: "init",
	info: "info",
	action: "action"
});

let currentState = states.init;
let repoPath = null;
let pathElement;
let outputElement;
let logOutputElement;

window.addEventListener("DOMContentLoaded", () => {
	pathElement = document.querySelector("#path");
	outputElement = document.querySelector("#output");
	logOutputElement = document.querySelector("#output-log");

	document.querySelector("#fetch-dir").addEventListener("submit", async (e) => {
		outputElement.textContent = "Fetching kifi...";
		e.preventDefault();

		let returned_path = await checkKifi(outputElement, pathElement);
		if (returned_path !== repoPath && returned_path !== null) {
			repoPath = returned_path;
			updateState(states.info);
		}
	});

	document.querySelector("#get-preview").addEventListener("click", async (e) => {
		e.preventDefault();
		updateLog();
		await preview(outputElement, repoPath);
	});

	document.querySelector("#get-info").addEventListener("click", async (e) => {
		e.preventDefault();
		updateLog();
		await meta(outputElement, repoPath);
	});

	document.querySelector("#get-log").addEventListener("click", async (e) => {
		e.preventDefault();
		updateLog();
		await log(outputElement, repoPath);
	});
});

function updateState(state) {
	// navbar is only hidden before the repository is initialised/selected
	if (state != states.init) {
		document.querySelector("#navbar").classList.remove("hidden");
	} else {
		document.querySelector("#navbar").classList.add("hidden");
	}

	document.querySelector("#" + currentState).classList.add("hidden");
	currentState = state;
	document.querySelector("#" + currentState).classList.remove("hidden");
}

function updateLog() {
	logOutputElement.textContent = outputElement.value + "\n~~~\n" + logOutputElement.value
}