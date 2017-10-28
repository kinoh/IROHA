import * as assert from "power-assert";
import { Hoge } from "../src/hoge";

describe("Hoge", () => {
	var hoge = new Hoge();

	describe("#double", () => {
		it("doubled", () => {
			assert(hoge.double(123) === 246);
		});
	});
});
