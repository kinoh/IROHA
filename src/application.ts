import Electron = require("electron");

export class Application {
	window: Electron.BrowserWindow = null;

	public constructor(public app: Electron.App) {
		this.app.on("ready", this.onReady);
	}

	public onReady(): void {
		this.window = new Electron.BrowserWindow({
			width: 800,
			height: 600,
		});

		this.window.loadURL("file://" + __dirname + "/../index.html");

		this.window.on("closed", () => {
			this.window = null;
		});
	}
}
