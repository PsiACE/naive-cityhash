use naive_cityhash::cityhash128;
use naive_cityhash::cityhash64;
use naive_cityhash::U128;

#[test]
fn test_64_len0() {
    assert_eq!(cityhash64(b""), 11160318154034397263);
}

#[test]
fn test_64_len1() {
    assert_eq!(cityhash64(b"1"), 11413460447292444913);
}

#[test]
fn test_64_len2() {
    assert_eq!(cityhash64(b"12"), 12074748272894662792);
}

#[test]
fn test_64_len3() {
    assert_eq!(cityhash64(b"abc"), 4220206313085259313);
}

#[test]
fn test_64_len4() {
    assert_eq!(cityhash64(b"1234"), 11914632649014994877);
}

#[test]
fn test_64_len5() {
    assert_eq!(cityhash64(b"12345"), 16429329056539592968);
}

#[test]
fn test_64_len6() {
    assert_eq!(cityhash64(b"123456"), 9260297286307356373);
}

#[test]
fn test_64_len7() {
    assert_eq!(cityhash64(b"1234567"), 11025202622668490255);
}

#[test]
fn test_64_len8() {
    assert_eq!(cityhash64(b"12345678"), 7177601938557627951);
}

#[test]
fn test_64_len9() {
    assert_eq!(cityhash64(b"123456789"), 12390271160407166709);
}

#[test]
fn test_64_len10() {
    assert_eq!(cityhash64(b"1234567890"), 11159486737701695049);
}

#[test]
fn test_64_len11() {
    assert_eq!(cityhash64(b"1234567890A"), 12461606063103015484);
}

#[test]
fn test_64_len12() {
    assert_eq!(cityhash64(b"1234567890Ab"), 3962957222420222636);
}

#[test]
fn test_64_len13() {
    assert_eq!(cityhash64(b"1234567890Abc"), 10943934074830884361);
}

#[test]
fn test_64_len14() {
    assert_eq!(cityhash64(b"1234567890AbcD"), 14566583638543629997);
}

#[test]
fn test_64_len15() {
    assert_eq!(cityhash64(b"1234567890AbcDE"), 1470766631700230904);
}

#[test]
fn test_64_len16() {
    assert_eq!(cityhash64(b"1234567890abcdef"), 10283158570132023530);
}

#[test]
fn test_64_len17() {
    assert_eq!(cityhash64(b"4RNfAbDSysH78xK5s"), 16686325262955297357);
}

#[test]
fn test_64_longer() {
    assert_eq!(
        cityhash64(b"1234567890abcdefghijklmnopqrstuvwxyz"),
        6062807976406716385
    );
}

#[test]
fn test_64_len64() {
    let data = &b"7zxsqkZNsEoNfRz83hct4HH5ytE3SFvx0MX9ACbDDBZhtUcR30pGvmJIPAoXwJCq"[..];
    assert_eq!(data.len(), 64); // test validity
    assert_eq!(cityhash64(data), 12341453991847893643);
}

#[test]
fn test_64_len65() {
    let data = &b"DSDXf2J5gvPZWtzo4qdrdbXw6qGKkVuzrV7zEZA3x6xNnGdQdSTr7YocaJWpgDgzq"[..];
    assert_eq!(data.len(), 65); // test validity
    assert_eq!(cityhash64(data), 12363235829494951337);
}

#[test]
fn test_64_verylong() {
    let data =
        &b"DMqhuXQxgAmJ9EOkT1n2lpzu7YD6zKc6ESSDWfJfohaQDwu0ba61bfGMiuS5GXpr0bIVcCtLwRtIVGmK"[..];
    assert_eq!(data.len(), 80); // test validity
    assert_eq!(cityhash64(data), 12512298373611890505);
}

#[test]
fn test_64_binary() {
    let data = b"\xe4x\x98\xa4*\xd7\xdc\x02p.\xdeI$\x9fp\xd4\xe3\xd7\xe7L\x86<5h75\xdf0B\x16\xe0\x86\xbeP\xb1rL\x8b\x07\x14!\x9e\xf5\xe0\x9cN\xa5\xfdJ]\xd8J\xc1\xc2.\xe6\xae\x14\xad^sW\x15&";
    assert_eq!(cityhash64(data.as_ref()), 5932484233276644677);
}

#[test]
fn test_cityhash128() {
    assert_eq!(
        cityhash128("abc".as_ref()),
        U128::new(0x900ff195577748fe, 0x13a9176355b20d7e)
    );
}

#[test]
fn test_from_u128() {
    let v = U128::from(0x11212312341234512345612345671234u128);
    assert_eq!(v.lo, 0x2345612345671234u64);
    assert_eq!(v.hi, 0x1121231234123451u64);
}

#[test]
fn test_into_u128() {
    let v: u128 = U128::new(0x2345612345671234u64, 0x1121231234123451u64).into();
    assert_eq!(v, 0x11212312341234512345612345671234u128);
}

#[test]
fn test_128_len0() {
    assert_eq!(
        cityhash128(b""),
        U128::new(4463240938071824939, 4374473821787594281)
    );
}

#[test]
fn test_128_len1() {
    assert_eq!(
        cityhash128(b"1"),
        U128::new(6359294370932160835, 9352172043616825891)
    );
}

#[test]
fn test_128_len2() {
    assert_eq!(
        cityhash128(b"12"),
        U128::new(16369832005849840265, 11285803613326688650)
    );
}

#[test]
fn test_128_len3() {
    assert_eq!(
        cityhash128(b"123"),
        U128::new(11385344155619871181, 565130349297615695)
    );
}

#[test]
fn test_128_len4() {
    assert_eq!(
        cityhash128(b"1234"),
        U128::new(2764810728135862028, 5901424084875196719)
    );
}

#[test]
fn test_128_len5() {
    assert_eq!(
        cityhash128(b"12345"),
        U128::new(11980518989907363833, 93456746504981291)
    );
}

#[test]
fn test_128_len6() {
    assert_eq!(
        cityhash128(b"123456"),
        U128::new(2350911489181485812, 12095241732236332703)
    );
}

#[test]
fn test_128_len7() {
    assert_eq!(
        cityhash128(b"1234567"),
        U128::new(10270309315532912023, 9823143772454143291)
    );
}

#[test]
fn test_128_len8() {
    assert_eq!(
        cityhash128(b"12345678"),
        U128::new(2123262123519760883, 8251334461883709976)
    );
}

#[test]
fn test_128_len9() {
    assert_eq!(
        cityhash128(b"123456789"),
        U128::new(14140762465907274276, 13893707330375041594)
    );
}

#[test]
fn test_128_len10() {
    assert_eq!(
        cityhash128(b"1234567890"),
        U128::new(8211333661328737896, 17823093577549856754)
    );
}

#[test]
fn test_128_len11() {
    assert_eq!(
        cityhash128(b"1234567890A"),
        U128::new(1841684041954399514, 6623964278873157363)
    );
}

#[test]
fn test_128_len12() {
    assert_eq!(
        cityhash128(b"1234567890Ab"),
        U128::new(3349064628685767173, 12952593207096460945)
    );
}

#[test]
fn test_128_len13() {
    assert_eq!(
        cityhash128(b"1234567890Abc"),
        U128::new(6572961695122645386, 13774858861848724400)
    );
}

#[test]
fn test_128_len14() {
    assert_eq!(
        cityhash128(b"1234567890AbcD"),
        U128::new(18041930573402443112, 5778672772533284640)
    );
}

#[test]
fn test_128_len15() {
    assert_eq!(
        cityhash128(b"1234567890AbcDE"),
        U128::new(11266190325599732773, 348002394938205539)
    );
}

#[test]
fn test_128_len16() {
    assert_eq!(
        cityhash128(b"1234567890AbcDEF"),
        U128::new(15073733098592741404, 5913034415582713572)
    );
}

#[test]
fn test_128_long() {
    assert_eq!(
        cityhash128(b"this is somewhat long string"),
        U128::new(2957911805285034456, 6923665615086076251)
    );
}

#[test]
fn test_128_longer() {
    assert_eq!(
        cityhash128(
            b"DMqhuXQxgAmJ9EOkT1n2lpzu7YD6zKc6ESSDWfJfohaQDwu0ba61bfGMiuS5GXpr0bIVcCtLwRtIVGmK"
        ),
        U128::new(9681404383092874918, 15631953994107571989)
    );
}

#[test]
fn test_128_binary() {
    let data = b"\xe4x\x98\xa4*\xd7\xdc\x02p.\xdeI$\x9fp\xd4\xe3\xd7\xe7L\x86<5h75\xdf0B\x16\xe0\x86\xbeP\xb1rL\x8b\x07\x14!\x9e\xf5\xe0\x9cN\xa5\xfdJ]\xd8J\xc1\xc2.\xe6\xae\x14\xad^sW\x15&";
    assert_eq!(
        cityhash128(data.as_ref()),
        U128::new(5907140908903622203, 10088853506155899265)
    );
}
