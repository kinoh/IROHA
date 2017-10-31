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

document.addEventListener("DOMContentLoaded", () => {
	let img = document.createElement("img");
	img.id = "figure";
	img.style.width = "100%";
	img.style.transform = "scale(-1, 1)";

	figure.load(__dirname + "/../resources/figure.csv", (success) => {
		if (success) {
			img.src = figure.get(0.5, 0.5);
		}
	});

	document.getElementById("canvas").appendChild(img);
});
