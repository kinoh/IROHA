import Electron = require("electron");

export class Application {
	window: Electron.BrowserWindow = null;

	public constructor(public app: Electron.App) {
		this.app.on("ready", this.onReady);
	}

	public onReady(): void {
		const size = { width: 300, height: 450 };

		this.window = new Electron.BrowserWindow({
			width: size.width,
			height: size.height,
			frame: false,
			transparent: true,
			skipTaskbar: true,
		});

		this.window.setAlwaysOnTop(true);

		const screenSize = Electron.screen.getPrimaryDisplay().size;
		this.window.setPosition(screenSize.width - size.width, screenSize.height - size.height);

		this.window.loadURL("file://" + __dirname + "/index.html");

		this.window.on("closed", () => {
			this.window = null;
		});
	}
}
