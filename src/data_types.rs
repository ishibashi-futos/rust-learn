pub fn data_types() {
    println!("data types!");

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("parsed={}", &guess);

    // 8bit符号付き()
    let mut _8bit_signed: i8 = -128;
    _8bit_signed = 127;
    // 8bit符号なし
    let mut _8bit_unsigned: u8 = 0;
    _8bit_unsigned = 255;

    // isize -> アーキテクチャのビット数依存の符号付(32bit OSではエラーになる)
    let _isize_signed: isize = 9_223_372_036_854_775_807;

    // isize -> アーキテクチャのビット数依存の符号なし(32bit OSではエラーになる)
    let _isize_unsigned: usize = 18_446_744_073_709_551_615;

    // Sum
    let _sum = 5 + 10; // i32

    // Difference
    let _difference = 95.5 - 4.3; // f64

    // Product
    let _product = 4 * 30; // i32

    // Quotient
    let _quotient = 56.7 / 32.2; // f64

    // Remainder
    let _remainder = 43 % 5; // i32

    /* 文字型(Not String) */
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻'; //ハート目の猫

    // Tuple
    let _tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = _tuple;
    println!("x={}, y={}, z={}", &_x, &_y, &_z);

    // Tupleには添え字でもアクセスできる
    println!("tuple[0]={}", &_tuple.0); // 500
    println!("tuple[1]={}", &_tuple.1); // 6.4
    println!("tuple[2]={}", &_tuple.2); // 1

    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("Month={}", &_months[0]); // January
    println!("Month={}", &_months[11]); // December

    // 以下の行はコンパイルエラーになる(配列はコンパイル時に長さが決定されており、不変であるため)
    // println!("Month={}", &_months[12]); // index out of bounds: the length is 12 but the index is 12
}
