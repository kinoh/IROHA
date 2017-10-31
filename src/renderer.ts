import { Balloon } from "./balloon";
import { Figure } from "./figure";
const remote = require("electron").remote;

let menu = remote.Menu.buildFromTemplate([
	{ label: "Bye", click: () => { remote.app.quit(); } }
]);

window.addEventListener("contextmenu", (e) => {
	e.preventDefault();
	menu.popup(remote.getCurrentWindow());
}, false);

let figure = new Figure();
let balloon: Balloon;

document.addEventListener("DOMContentLoaded", () => {
	const container = document.getElementById("container");

	let img = document.createElement("img");
	img.style.width = "100%";
	img.style.transform = "scale(-1, 1)";
	container.appendChild(img);

	const canvas = document.createElement("canvas");
	container.appendChild(canvas);

	const balloonText = document.createElement("div");
	container.appendChild(balloonText);

	balloon = new Balloon(canvas, balloonText);
	balloon.update({ x: 130, y: 200 }, { x: 150, y: 270 }, { width: 300, height: 70 }, "てめー頭湧いてんのか");

	figure.load(__dirname + "/../resources/figure.csv", (success) => {
		if (!success) return;

		img.src = figure.get(0.5, 0.5);
	});
});
