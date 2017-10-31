import { Position, Size } from "./drawing";

export class Balloon {
	private canvas: HTMLCanvasElement;
	private context: CanvasRenderingContext2D;
	private text: HTMLElement;

	private readonly radius = 20;
	private tail: Position;
	private center: Position;
	private size: Size;

	public constructor(canvas: HTMLCanvasElement, text: HTMLElement) {
		this.canvas = canvas;

		this.canvas.style.position = "absolute";
		this.canvas.style.left = "0";
		this.canvas.style.top = "0";

		this.adjustSize();
		window.addEventListener("resize", () => {
			this.adjustSize();
			this.draw();
		});

		this.context = canvas.getContext("2d");
		this.text = text;

		this.text.style.position = "absolute";
	}

	private adjustSize(): void {
		this.canvas.width = window.innerWidth;
		this.canvas.height = window.innerHeight;
	}

	public update(tail: Position, center: Position, size: Size, message: string): void {
		this.tail = tail;
		this.center = center;
		this.size = size;

		this.draw();

		this.text.innerText = message;
		this.text.style.padding = (0.7 * this.radius) + "px";
		this.text.style.left = (this.center.x - this.size.width / 2) + "px";
		this.text.style.top = (this.center.y - this.size.height / 2) + "px";
		this.text.style.width = size.width + "px";
		this.text.style.height = size.height + "px";
	}

	private draw(): void {
		const c = this.context;

		c.clearRect(0, 0, this.canvas.width, this.canvas.height);

		const r = this.radius;
		const left = this.center.x - this.size.width / 2;
		const right = this.center.x + this.size.width / 2;
		const top = this.center.y - this.size.height / 2;
		const bottom = this.center.y + this.size.height / 2;

		const tailPosiition = 0.3;
		const tailSize = { width: 20, height: 20 };
		const tailSkew = Math.min(tailSize.width / 2, this.tail.x - (left + this.size.width * tailPosiition));

		c.beginPath();
		c.lineWidth = 1;
		c.strokeStyle = "black";
		c.fillStyle = "white";
		c.moveTo(left + this.size.width * tailPosiition + tailSize.width / 2, top);
		c.lineTo(right - r, top);
		c.arcTo(right, top, right, top + r, r);
		c.lineTo(right, bottom - r);
		c.arcTo(right, bottom, right - r, bottom, r);
		c.lineTo(left + r, bottom);
		c.arcTo(left, bottom, left, bottom - r, r);
		c.lineTo(left, top + r);
		c.arcTo(left, top, left + r, top, r);
		c.lineTo(left + this.size.width * tailPosiition - tailSize.width / 2, top);
		c.lineTo(left + this.size.width * tailPosiition + tailSkew, top - tailSize.height);
		c.closePath();
		c.fill();
		c.stroke();
	}
}
