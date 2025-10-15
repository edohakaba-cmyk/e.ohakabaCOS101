fn  main() {
	let p:f64=510_000.0;
	let r:f64=5.0;
	let n:f64=3.0;

	// compound intrest calculator
	let a=p*(1.0-(r/100.0)).powf (n);
	println! ("amount ={}",a);
	let ci=a-p;
	println!("compound intrest of ms Anjola Olowokere is {}", ci);
}
