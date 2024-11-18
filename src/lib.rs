pub const INVALID: u16 = 0x2020;

static DAY_TO_STR: [u16; 32] = [
    INVALID, // 00
    0x3031, 0x3032, 0x3033, 0x3034, 0x3035, 0x3036, 0x3037, 0x3038, 0x3039, // 01-09
    0x3130, 0x3131, 0x3132, 0x3133, 0x3134, 0x3135, 0x3136, 0x3137, 0x3138, 0x3139, // 10-19
    0x3230, 0x3231, 0x3232, 0x3233, 0x3234, 0x3235, 0x3236, 0x3237, 0x3238, 0x3239, // 20-29
    0x3330, 0x3331, // 30,31
];

pub fn day_to_string(day: u8) -> u16 {
    let ix: u8 = day & 0x1f;
    DAY_TO_STR[ix as usize]
}

#[cfg(target_family = "wasm")]
#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn day2str(day: u8) -> u16 {
    day_to_string(day)
}

#[cfg(test)]
mod test_day2str {

    mod day_to_string {
        macro_rules! create_test_day2str {
            ($fname: ident, $input: expr, $expected: expr) => {
                #[test]
                fn $fname() {
                    let input: u8 = $input;
                    let expected: u16 = $expected;

                    let got: u16 = crate::day_to_string(input);

                    assert_eq!(got, expected);
                }
            };
        }

        create_test_day2str!(test01, 1, 0x3031);
        create_test_day2str!(test02, 2, 0x3032);
        create_test_day2str!(test03, 3, 0x3033);
        create_test_day2str!(test04, 4, 0x3034);
        create_test_day2str!(test05, 5, 0x3035);
        create_test_day2str!(test06, 6, 0x3036);
        create_test_day2str!(test07, 7, 0x3037);
        create_test_day2str!(test08, 8, 0x3038);
        create_test_day2str!(test09, 9, 0x3039);

        create_test_day2str!(test10, 10, 0x3130);
        create_test_day2str!(test11, 11, 0x3131);
        create_test_day2str!(test12, 12, 0x3132);
        create_test_day2str!(test13, 13, 0x3133);
        create_test_day2str!(test14, 14, 0x3134);
        create_test_day2str!(test15, 15, 0x3135);
        create_test_day2str!(test16, 16, 0x3136);
        create_test_day2str!(test17, 17, 0x3137);
        create_test_day2str!(test18, 18, 0x3138);
        create_test_day2str!(test19, 19, 0x3139);

        create_test_day2str!(test20, 20, 0x3230);
        create_test_day2str!(test21, 21, 0x3231);
        create_test_day2str!(test22, 22, 0x3232);
        create_test_day2str!(test23, 23, 0x3233);
        create_test_day2str!(test24, 24, 0x3234);
        create_test_day2str!(test25, 25, 0x3235);
        create_test_day2str!(test26, 26, 0x3236);
        create_test_day2str!(test27, 27, 0x3237);
        create_test_day2str!(test28, 28, 0x3238);
        create_test_day2str!(test29, 29, 0x3239);

        create_test_day2str!(test30, 30, 0x3330);
        create_test_day2str!(test31, 31, 0x3331);
    }
}
