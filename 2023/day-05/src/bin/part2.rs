use std::{borrow::BorrowMut, ops::Range};

use itertools::Itertools;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete as cc,
    character::complete::{alpha1, one_of, space1},
    combinator::{all_consuming, map, value},
    error::ParseError,
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};

fn main() {
    let input = "seeds: 432986705 28073546 1364097901 88338513 2733524843 234912494 3151642679 224376393 485709676 344068331 1560394266 911616092 3819746175 87998136 892394515 435690182 4218056486 23868437 848725444 8940450

seed-to-soil map:
748585809 2125564114 88980459
1317392128 775565564 217595062
1218610825 676784261 98781303
954230685 2235762425 141777617
2920242079 4081180892 51765553
2972007632 3159586797 16102841
0 2377540042 17565155
2834452876 3712797875 58062179
2892515055 2917079842 6424918
3327351062 3175689638 162608005
673338549 647264576 29519685
1197392973 2214544573 21217852
738232750 116664417 10353059
2988110473 2429807442 71556277
17565155 334379348 277510712
1700771639 228674051 105705297
3059666750 4132946445 162020851
1806476936 993160626 588628261
1096008302 127289380 101384671
622123656 1908836676 50942989
3221687601 3338297643 28028532
2408505336 3770860054 310320838
4175210607 3039830108 119756689
3326652416 3039131462 698646
2898939973 2408505336 21302106
673066645 127017476 271904
3489959067 3382623558 330174317
702858234 611890060 35374516
4086270124 2562002619 88940483
837566268 0 116664417
1534987190 1959779665 165784449
2718826174 2923504760 115626702
3249716133 3366326175 16297383
3820133384 2650943102 266136740
3266013516 2501363719 60638900
295075867 1581788887 327047789

soil-to-fertilizer map:
2018515973 2192795257 82329405
3722326327 3015971185 249665840
3046459770 3689390318 25519185
3971992167 3265637025 40217941
3071978955 3453653215 203407731
0 443504340 17965088
584437096 1722124969 470670288
1055107384 744431503 164966659
1489299099 461469428 282962075
2321848831 2380372526 153650776
2100845378 269225056 174279284
3487660258 2648616968 234666069
3275386686 3305854966 147798249
1772261174 1172578553 246254799
4012210108 2883283037 132688148
3423184935 4138946628 64475323
4144898256 2321848831 58523695
538253726 1418833352 46183370
1220074043 0 269225056
17965088 909398162 263180391
2590093273 3657060946 32329372
281145479 1465016722 257108247
2622422645 3714909503 424037125
2475499607 2534023302 114593666

fertilizer-to-water map:
3731805434 353192162 37567806
926873139 889685769 255250442
3170336676 695153543 194532226
679924479 451681440 193671776
3009343704 3081959489 160992972
1242360754 3579359343 278026518
1861131448 2500688596 20068354
4028837903 4006213119 266129393
1182123581 3242952461 60237173
3877550443 645353216 49800327
2223776164 1371077033 341527178
3364868902 2566566565 36440100
1773121333 0 76664401
264823995 2444756861 55931735
3929841219 3857385861 27802851
2166799431 1712604211 56976733
873596255 1769580944 53276884
645696746 3047731756 34227733
3927350770 3955153621 2490449
3769373240 177937131 108177203
0 3314535348 264823995
1942121274 3885188712 69964909
1881199802 390759968 60921472
1849785734 3303189634 11345714
3401309002 2855740726 104355610
2079164011 2960096336 87635420
544424016 76664401 101272730
2565303342 2520756950 45809615
1520387272 2603006665 252734061
2012086183 286114334 67077828
2611112957 1822857828 398230747
320755730 2221088575 223668286
3505664612 1144936211 226140822
4006213119 4272342512 22624784

water-to-light map:
62780592 544346201 30115959
2740764032 1352944740 34082945
377487729 807592920 35446631
1316419610 1454554942 34907962
986581913 756881718 50711202
4167758628 3240047125 127208668
818809239 1222506283 58684750
3649838514 2036598113 6212644
127663629 0 10715051
3023280854 1435387310 19167632
663070842 10715051 124076893
2774846977 2422700597 37614763
1812617371 2460315360 5121443
1640337506 1864318248 172279865
2986755724 1316419610 36525130
2023334670 2467203928 540327060
1159184084 248462172 14557802
1037293115 152894449 95567723
0 134791944 18102505
18102505 712203631 44678087
465375803 972369801 197695039
2576394916 3007530988 65274194
92896551 1281191033 34767078
3656051158 4289142647 5824649
412934360 1170064840 52441443
3417303830 3873138614 84580361
787147735 263019974 31661504
1817738814 2042810757 205595856
1285160111 843039551 30798000
2563661730 3123136764 12733186
138378680 305237152 239109049
3648071389 2465436803 1767125
1132860838 574462160 26323246
888049663 873837551 98532250
3626039273 3072805182 22032116
3530183657 4193287031 95855616
1404769450 3957718975 235568056
3042448486 1489462904 374855344
2641669110 1387027685 48359625
877493989 294681478 10555674
3501884191 3094837298 28299466
1351327572 3186605247 53441878
2690028735 3135869950 50735297
2812461740 2248406613 174293984
3661875807 3367255793 505882821
1173741886 600785406 111418225

light-to-temperature map:
964570004 989608620 226759942
2204148775 2545437438 20646474
233260112 338444213 39032265
958191857 332066066 6378147
2318799855 914518254 75090366
4247140372 3146297568 47826924
2224795249 1216368562 94004606
2871022952 1310373168 80313918
1400254919 233260112 98805954
445493256 487550555 149554087
2576473348 3962746668 294549604
3535295748 2775008885 371288683
1499060873 377476478 110074077
272292377 2215619580 173200879
3347481948 1867953550 157067409
4161267146 3794452372 85873226
3504549357 2184873189 30746391
1759636962 1780717197 87236353
2951336870 2388820459 6114967
1191329946 2566083912 208924973
1884544339 3880325598 82421070
595047343 3431307858 363144514
2393890221 731935127 182583127
4001414916 2025020959 159852230
2957451837 1390687086 390030111
1846873315 4257296272 37671024
1966965409 3194124492 237183366
1609134950 2394935426 150502012
3906584431 637104642 94830485

temperature-to-humidity map:
1406768592 2335526312 13344484
666958498 1862550129 472976183
558853371 843618476 74696086
1168798622 129171378 168640618
1713291209 297811996 183431863
1993628008 635748116 152317885
2560263686 2849350774 11516524
32266442 1212766321 287276323
2571780210 3319898101 11192927
375095240 995599149 183758131
2661986290 2353962919 50829838
3252020768 4280298713 14668583
1337439240 1793220777 69329352
3419718116 3502299739 574454544
2353962919 2650392505 198958269
633549457 1179357280 33409041
2582973137 4076754283 50515665
319542765 788066001 55552475
1896723072 32266442 96904936
1420113076 1500042644 293178133
3006421020 2404792757 245599748
2842554807 3331091028 163866213
2633488802 2990605977 28497488
2300450150 947178503 48420646
3266689351 4127269948 153028765
2145945893 481243859 154504257
3994172660 3019103465 300794636
1139934681 918314562 28863941
2712816128 2860867298 129738679
2552921188 3494957241 7342498

humidity-to-location map:
897459980 3171885613 268595078
506368722 1864971513 13322696
1166055058 2803961444 53745388
2572095034 667166679 114420176
687118932 1725187165 139784348
2478398695 0 14138781
3427672233 370325921 251085897
3888215738 3612891343 82449665
1674720770 1530101168 79955344
3970665403 925512154 2812137
519691418 2452425610 167427514
3884704963 3168374838 3510775
826903280 2381868910 70556700
2399774019 349568762 20757159
2972099388 3465151802 147739541
1754676114 131614075 217954687
2865104023 3440480691 24671111
2206760431 932309368 77882935
2284643366 1610056512 115130653
2492537476 14138781 35151040
2527688516 3695341008 44406518
3119838929 781586855 143925299
2732270071 2857706832 132833952
1599442846 2728683520 75277924
3263764228 3995626854 27783181
0 2990540784 177834054
2686515210 621411818 45754861
2420531178 2670816003 57867517
1219800446 1010192303 191374197
3678758130 3789680021 205946833
3973477540 3739747526 49932495
1972630801 2014419033 234129630
3291547409 1878294209 136124824
2889775134 49289821 82324254
1411174643 2619853124 50962879
1466122599 2248548663 133320247
177834054 1201566500 328534668
1462137522 928324291 3985077";
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, Clone)]
pub struct Seeds(Vec<usize>);

enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub struct SeedRanges {
    seed: usize,
    range: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct SeedMap {
    from: usize,
    to: usize,
    width: usize,
}

impl SeedMap {
    /// Returns true if the value is in the range of the from seedmap
    fn in_range_of_from(&self, seed: &usize) -> bool {
        let end = self.from + self.width;
        (self.from..end).contains(seed)
    }

    /// Returns true if the value is in the range of the transformed seedmap
    fn in_range_of_to(&self, value: &usize) -> bool {
        (self.to..(self.to + self.width)).contains(value)
    }

    fn max_from(&self) -> usize {
        self.from + self.width - 1
    }

    fn max_to(&self) -> usize {
        self.to + self.width - 1
    }

    fn is_overlapping(&self, range: &Range<usize>) -> bool {
        dbg!("in is_overlapping");
        dbg!(range.clone());
        let self_range = self.from..(self.from + self.width);
        dbg!(self_range);
        let range_min = range.clone().min().unwrap();
        let range_max = range.clone().max().unwrap();
        let overlapping = (range_min >= self.from && range_min <= self.max_from()) ||
         (range_max >= self.from && range_max <= self.max_from()) ||
         (self.max_from() >= range_min && self.from <= range_max) ||
         (self.max_from() >= range_min && self.max_from() <= range_max);
        dbg!(overlapping);
        overlapping
    }

    /// Value returned is the direction you transform
    // fn overlap_direction(&self, range: &Range<usize>) -> Direction {
    //     match self.from < range.min().unwrap() {
    //         true => Direction::Left,
    //         false => Direction::Right,
    //     }
    // }

    fn overlap(&self, range: &Range<usize>) -> Vec<Range<usize>> {
        // check which kind of overlapping

        let range_min = range.clone().min().unwrap();
        let range_max = range.clone().max().unwrap();

        // case 1: inside the seedmap
        // [[ 1, 2, 3, [ 4, 5, 6 ] 7, 8, 9 ]]
        if range_min >= self.from && range_max <= self.max_from() {
            dbg!("inside");
            vec![range.to_owned()]
        }
        else if range_min <= self.from && range_max >= self.max_from() {
            // case 3: seedmap is inside the range
            // [ 3, 4, [[ 5, 6 , 7 ]] 8, 9 ]
            //            ^overlap
            dbg!("spans");
            let lower = range_min..self.from;
            let middle = self.from..self.max_from();
            let higher = self.max_from()..range_max;
            let out = vec![lower, middle, higher];
            let out: Vec<Range<_>> = out.iter().filter(|x| x.len() != 0).map(|x| x.to_owned()).collect();
            out
        }
        // case 2: intersects with seedmap
        // [4, [[ 5,6 ], 7, 8]]
        //      ^overlap
        else {
            dbg!("intersects");
            let to;
            if self.from < range_min {
                to = self.max_from();
            } else {
                to = self.from;
            }
            dbg!(to);
            let lower = range_min..to;
            let higher = to..range_max;
            let out = vec![lower, higher];
            dbg!(out.clone());
            let out: Vec<Range<_>> = out.iter().filter(|x| x.len() != 0).map(|x| x.to_owned()).collect();
            out
        }
    }

    fn transform_range(&self, range: &Range<usize>) -> Range<usize> {
        let offset = self.offset(&range.clone().min().unwrap());
        let to = self.offset_to(&offset);
        let out = to..(to + range.len());
        dbg!(out.clone());
        out
    }

    /// Returns the offset of the seed value from the from value
    ///
    /// Note: will always be positive. Always check if the seed is in range using
    /// `in_range_of_from()` before using offset
    fn offset(&self, seed: &usize) -> usize {
        seed.abs_diff(self.from)
    }

    /// Converts the value from `offset()` into a return value
    fn offset_to(&self, offset: &usize) -> usize {
        self.to + offset
    }
}

// Test case
// seeds: 79 14 55 13
//
// seed-to-soil map:
// 50 98 2
// 52 50 48
//
// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15
//
// fertilizer-to-water map:
// 49 53 8
// 0 11 42
// 42 0 7
// 57 7 4
//
// water-to-light map:
// 88 18 7
// 18 25 70
//
// light-to-temperature map:
// 45 77 23
// 81 45 19
// 68 64 13
//
// temperature-to-humidity map:
// 0 69 1
// 1 0 69
//
// humidity-to-location map:
// 60 56 37
// 56 93 4

pub fn parse_map(i: &str) -> IResult<&str, SeedMap> {
    map(
        tuple((cc::u64, tag(" "), cc::u64, tag(" "), cc::u64)),
        |(x, _, y, _, z)| SeedMap {
            from: y as usize,
            to: x as usize,
            width: z as usize,
        },
    )(i)
}

pub fn parse_seedmap(i: &str) -> IResult<&str, Vec<SeedMap>> {
    // Sample input:
    // seed-to-soil map:
    // 50 98 2
    // 52 50 48

    let (i, _) = tuple((alpha1, tag("-to-"), alpha1, tag(" map:\n")))(i)?;
    separated_list1(tag("\n"), parse_map)(i)
}

pub fn parse_all_seedmaps(i: &str) -> IResult<&str, Vec<Vec<SeedMap>>> {
    separated_list1(tag("\n\n"), parse_seedmap)(i) // god i hope this works
}

fn parse(i: &str) -> IResult<&str, (Seeds, Vec<Vec<SeedMap>>)> {
    // seeds: 79 14 55 13
    //
    // seed-to-soil map:
    // 50 98 2
    // 52 50 48
    let (i, seeds) = map(
        delimited(
            tag("seeds: "),
            separated_list1(tag(" "), cc::u64),
            tag("\n\n"),
        ),
        |seed_list| {
            let seed_list = seed_list.iter().map(|x| *x as usize).collect();
            Seeds(seed_list)
        },
    )(i)?;
    let (i, seedmaps) = parse_all_seedmaps(i)?;

    Ok((i, (seeds, seedmaps)))
}

fn part1(i: &str) -> String {
    let i = i.trim_end();
    let (seeds, seedmaps) = all_consuming(parse)(i).finish().unwrap().1;

    dbg!(seeds.clone());
    dbg!(seedmaps.clone());

    let seed_ranges: Vec<_> = seeds
        .0
        .chunks(2)
        .map(|seed_range| {
            let seed = seed_range.first().unwrap_or(&0);
            let range = seed_range.last().unwrap_or(&0);

            SeedRanges {
                seed: *seed,
                range: *range,
            }
        })
        .collect();
    dbg!(seed_ranges.clone());

    let seeds: Vec<_> = seed_ranges
        .iter()
        .map(|seed_range| (seed_range.seed..(seed_range.seed + seed_range.range)))
        .collect();

    // let new_seeds: Vec<_> = seeds
    //     .0
    //     .iter()
    //     .map(|seed| {
    //         let mut seed = seed;
    //         seedmaps.iter().for_each(|map| {
    //             map.iter()
    //                 .for_each(|layer| match layer.in_range_of_from(seed) {
    //                     true => {
    //                         let offset = layer.offset(seed);
    //                         let offset_to = layer.offset_to(&offset);
    //                         seed = &offset_to
    //                     }
    //                     false => {}
    //                 })
    //         });
    //         let seed = seed;
    //         seed
    //     })
    //     .collect();
    
    let tmp_seeds = seeds.clone();

    let new_seeds: Vec<_> = seedmaps.iter().fold(seeds, |accum, map| {
        let new_seeds: Vec<_> = accum
            .iter()
            .map(|seed| {
                let mut o = vec![seed.clone()];
                for layer in map.iter() {
                    if layer.is_overlapping(seed) {
                        dbg!(seed, layer);

                        let overlap = layer.overlap(seed);
                        dbg!(overlap.clone());
                        let overlap: Vec<_> = overlap
                            .iter()
                            .map(|x| {
                                dbg!(x.clone());
                                if layer.is_overlapping(x) {
                                    layer.transform_range(x).to_owned()
                                } else {
                                    x.to_owned()
                                }
                            })
                            .collect();
                        o = overlap;
                    }
                }
                dbg!(o.clone());
                o
            })
            .flatten()
            .collect();
        dbg!(new_seeds.clone());
        new_seeds
    });

    dbg!(new_seeds.clone());
    
    
    // let new_seeds = new_seeds.iter().map(|x| x.collect::<Vec<_>>().iter().filter(|y| {dbg!(y.clone()); tmp_seeds.iter().contains(y)}));

    // dbg!(new_seeds);    
    // let min = new_seeds.iter().reduce(|accum, x| {
    //     let accum_min = accum.clone().min().unwrap();
    //     let x_min = x.clone().min().unwrap();
    //     if std::cmp::min(accum_min, x_min) == accum_min {
    //         accum
    //     } else {
    //         x
    //     }
    // }).unwrap().clone().min().unwrap();
    
    let mut min = usize::MAX;
    for i in new_seeds.clone().into_iter() {
        for j in i {
            for k in tmp_seeds.clone().into_iter() {
                if k.contains(&j) && j < min {
                    min = j;
                }
            }
        }
    }
    dbg!(min);

    // let min = new_seeds.iter().r.unwrap_or(&0);

    "min".to_string()
}
