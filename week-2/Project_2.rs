fn main() {

	//x

	let tx:f64 = 450000.0;
	let mx:f64 = 1500000.0;
	let hx:f64 = 750000.0;
	let dx:f64 = 2850000.0;
	let ax:f64 = 250000.0;

	//f

	let tf:f64 = 2.0;
	let mf:f64 = 1.0;
	let hf:f64 = 3.0;
	let df:f64 = 3.0;
	let af:f64 = 1.0;

	// fx
	let tfx:f64 = tf * tx;
	let mfx:f64 = mf * mx;
	let hfx:f64 = hf * hx;
	let dfx:f64 = df * dx;
	let afx:f64 = af * ax;

	// total amount of sales
	let t = tfx + mfx + hfx + dfx + afx;
	println!("total amount of sales is {}", t);

	// total number of items solds
	let n = tf + mf + hf + df + af;
	println!("total number of items sold is {}", n);

	// average of sales record
	let a = t / n;
	println!("average of sales records is {}", a);

}
	