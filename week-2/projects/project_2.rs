fn main () {
	let toshiba_qty:f64 = 2.0;
	let toshiba_amt:f64 = 450_000.00;
	let toshiba_ttl:f64 = toshiba_qty*toshiba_amt;

	let  mac_qty:f64 = 1.0;
	let mac_amt:f64 =1_500_000.00;
	let mac_ttl:f64 = mac_qty*mac_amt;


	let hp_qty:f64 =3.0;
	let hp_amt:f64 = 750_000.00;
	let hp_ttl:f64 = hp_qty*hp_amt;

	let dell_qty:f64 =3.0;
	let dell_amt:f64 = 2_850_000.00;
	let dell_ttl:f64 = dell_qty*dell_amt;

	let acer_qty:f64 =1.0;
	let acer_amt:f64 = 250_000.00; 
	let acer_ttl:f64 = acer_qty*acer_amt;

	let number_of_brands:f64 = 5.0;

	let sum:f64 = toshiba_ttl+mac_ttl+hp_ttl+dell_ttl+acer_ttl;

	let average:f64 = sum / number_of_brands;

	println! ("The total sum is {}", sum);
	println! ("The average sale is {}", average);


}