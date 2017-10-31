import fs = require("fs");
import path = require("path");

class ExpressionPair {
	public constructor(readonly arousing: number, readonly pleasure: number, readonly path: string) {
	}
}

export class Figure {
	readonly mapping: ExpressionPair[];

	public constructor() {
		this.mapping = [];
	}

	public register(arousing: number, pleasure: number, path: string): void {
		this.mapping.push(new ExpressionPair(arousing, pleasure, path));
	}

	public load(file: string, callback: (success: boolean) => void = () => {}): void {
		let dir = path.dirname(file);

		fs.readFile(file, "utf8", (err, data) => {
			if (err) {
				console.log(err);
				callback(false);
			} else {
				for (let line of data.split("\n")) {
					let data = line.split(",");
					if (data.length < 3) {
						continue;
					}
					this.register(parseFloat(data[0]), parseFloat(data[1]), dir + path.sep + data[2]);
				}
				callback(true);
			}
		});
	}

	public get(arousing: number, pleasure: number): string {
		if (this.mapping.length == 0) {
			return null;
		}

		let min = 2 * 2 + 2 * 2;
		let minExpr: ExpressionPair;

		for (let expr of this.mapping) {
			let v = [arousing - expr.arousing, pleasure - expr.pleasure];
			let d = v[0] * v[0] + v[1] * v[1];
			if (d < min) {
				min = d;
				minExpr = expr;
			}
		}

		return minExpr.path;
	}
}
