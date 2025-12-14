macro_rules! g {
    ($ s : expr) => {
        IndexMap::from($s)
    };
}
pub fn __action__() -> IndexMap<StateId, IndexMap<SymbolId, Action>> {
    IndexMap::from([
        (i(0usize), {
            g! { [(s (5usize) , S (i (3usize)))] }
        }),
        (i(1usize), {
            g! { [(s (1usize) , A)] }
        }),
        (i(2usize), {
            g! { [(s (65usize) , S (i (19usize))) , (s (74usize) , S (i (27usize))) , (s (66usize) , S (i (20usize))) , (s (67usize) , S (i (21usize))) , (s (68usize) , S (i (22usize))) , (s (69usize) , S (i (23usize))) , (s (71usize) , S (i (24usize))) , (s (108usize) , S (i (29usize))) , (s (72usize) , S (i (25usize))) , (s (73usize) , S (i (26usize))) , (s (101usize) , S (i (28usize)))] }
        }),
        (i(3usize), {
            g! { [(s (65usize) , R (p (2usize))) , (s (74usize) , R (p (2usize))) , (s (66usize) , R (p (2usize))) , (s (67usize) , R (p (2usize))) , (s (68usize) , R (p (2usize))) , (s (69usize) , R (p (2usize))) , (s (71usize) , R (p (2usize))) , (s (108usize) , R (p (2usize))) , (s (72usize) , R (p (2usize))) , (s (73usize) , R (p (2usize))) , (s (101usize) , R (p (2usize)))] }
        }),
        (i(4usize), {
            g! { [(s (1usize) , R (p (1usize)))] }
        }),
        (i(5usize), {
            g! { [(s (1usize) , R (p (3usize))) , (s (106usize) , R (p (3usize))) , (s (65usize) , S (i (19usize))) , (s (74usize) , S (i (27usize))) , (s (66usize) , S (i (20usize))) , (s (67usize) , S (i (21usize))) , (s (68usize) , S (i (22usize))) , (s (69usize) , S (i (23usize))) , (s (71usize) , S (i (24usize))) , (s (108usize) , S (i (29usize))) , (s (72usize) , S (i (25usize))) , (s (73usize) , S (i (26usize))) , (s (101usize) , S (i (28usize)))] }
        }),
        (i(6usize), {
            g! { [(s (109usize) , S (i (53usize))) , (s (107usize) , S (i (52usize))) , (s (108usize) , S (i (29usize))) , (s (110usize) , S (i (54usize))) , (s (111usize) , S (i (55usize))) , (s (70usize) , S (i (49usize))) , (s (87usize) , S (i (50usize))) , (s (103usize) , S (i (51usize)))] }
        }),
        (i(7usize), {
            g! { [(s (1usize) , R (p (6usize))) , (s (65usize) , R (p (6usize))) , (s (74usize) , R (p (6usize))) , (s (66usize) , R (p (6usize))) , (s (67usize) , R (p (6usize))) , (s (68usize) , R (p (6usize))) , (s (69usize) , R (p (6usize))) , (s (71usize) , R (p (6usize))) , (s (108usize) , R (p (6usize))) , (s (72usize) , R (p (6usize))) , (s (73usize) , R (p (6usize))) , (s (101usize) , R (p (6usize))) , (s (106usize) , R (p (6usize)))] }
        }),
        (i(8usize), {
            g! { [(s (109usize) , S (i (53usize))) , (s (107usize) , S (i (52usize))) , (s (108usize) , S (i (29usize))) , (s (110usize) , S (i (54usize))) , (s (111usize) , S (i (55usize))) , (s (70usize) , S (i (49usize))) , (s (87usize) , S (i (50usize))) , (s (103usize) , S (i (51usize)))] }
        }),
        (i(9usize), {
            g! { [(s (1usize) , R (p (8usize))) , (s (65usize) , R (p (8usize))) , (s (74usize) , R (p (8usize))) , (s (66usize) , R (p (8usize))) , (s (67usize) , R (p (8usize))) , (s (68usize) , R (p (8usize))) , (s (69usize) , R (p (8usize))) , (s (71usize) , R (p (8usize))) , (s (108usize) , R (p (8usize))) , (s (72usize) , R (p (8usize))) , (s (73usize) , R (p (8usize))) , (s (101usize) , R (p (8usize))) , (s (106usize) , R (p (8usize)))] }
        }),
        (i(10usize), {
            g! { [(s (1usize) , R (p (9usize))) , (s (65usize) , R (p (9usize))) , (s (74usize) , R (p (9usize))) , (s (66usize) , R (p (9usize))) , (s (67usize) , R (p (9usize))) , (s (68usize) , R (p (9usize))) , (s (69usize) , R (p (9usize))) , (s (71usize) , R (p (9usize))) , (s (108usize) , R (p (9usize))) , (s (72usize) , R (p (9usize))) , (s (73usize) , R (p (9usize))) , (s (101usize) , R (p (9usize))) , (s (106usize) , R (p (9usize)))] }
        }),
        (i(11usize), {
            g! { [(s (1usize) , R (p (10usize))) , (s (65usize) , R (p (10usize))) , (s (74usize) , R (p (10usize))) , (s (66usize) , R (p (10usize))) , (s (67usize) , R (p (10usize))) , (s (68usize) , R (p (10usize))) , (s (69usize) , R (p (10usize))) , (s (71usize) , R (p (10usize))) , (s (108usize) , R (p (10usize))) , (s (72usize) , R (p (10usize))) , (s (73usize) , R (p (10usize))) , (s (101usize) , R (p (10usize))) , (s (106usize) , R (p (10usize)))] }
        }),
        (i(12usize), {
            g! { [(s (109usize) , S (i (53usize))) , (s (107usize) , S (i (52usize))) , (s (108usize) , S (i (29usize))) , (s (110usize) , S (i (54usize))) , (s (111usize) , S (i (55usize))) , (s (70usize) , S (i (49usize))) , (s (87usize) , S (i (50usize))) , (s (103usize) , S (i (51usize)))] }
        }),
        (i(13usize), {
            g! { [(s (86usize) , S (i (59usize)))] }
        }),
        (i(14usize), {
            g! { [(s (109usize) , S (i (53usize))) , (s (107usize) , S (i (52usize))) , (s (108usize) , S (i (29usize))) , (s (110usize) , S (i (54usize))) , (s (111usize) , S (i (55usize))) , (s (70usize) , S (i (49usize))) , (s (87usize) , S (i (50usize))) , (s (103usize) , S (i (51usize)))] }
        }),
        (i(15usize), {
            g! { [(s (109usize) , S (i (53usize))) , (s (107usize) , S (i (52usize))) , (s (108usize) , S (i (29usize))) , (s (110usize) , S (i (54usize))) , (s (111usize) , S (i (55usize))) , (s (70usize) , S (i (49usize))) , (s (87usize) , S (i (50usize))) , (s (103usize) , S (i (51usize)))] }
        }),
        (i(16usize), {
            g! { [(s (1usize) , R (p (15usize))) , (s (65usize) , R (p (15usize))) , (s (74usize) , R (p (15usize))) , (s (66usize) , R (p (15usize))) , (s (67usize) , R (p (15usize))) , (s (68usize) , R (p (15usize))) , (s (69usize) , R (p (15usize))) , (s (71usize) , R (p (15usize))) , (s (108usize) , R (p (15usize))) , (s (72usize) , R (p (15usize))) , (s (73usize) , R (p (15usize))) , (s (101usize) , R (p (15usize))) , (s (106usize) , R (p (15usize)))] }
        }),
        (i(17usize), {
            g! { [(s (1usize) , R (p (16usize))) , (s (65usize) , R (p (16usize))) , (s (74usize) , R (p (16usize))) , (s (66usize) , R (p (16usize))) , (s (67usize) , R (p (16usize))) , (s (68usize) , R (p (16usize))) , (s (69usize) , R (p (16usize))) , (s (71usize) , R (p (16usize))) , (s (108usize) , R (p (16usize))) , (s (72usize) , R (p (16usize))) , (s (73usize) , R (p (16usize))) , (s (101usize) , R (p (16usize))) , (s (106usize) , R (p (16usize))) , (s (102usize) , S (i (66usize)))] }
        }),
        (i(18usize), {
            g! { [(s (109usize) , S (i (53usize))) , (s (107usize) , S (i (52usize))) , (s (108usize) , S (i (29usize))) , (s (110usize) , S (i (54usize))) , (s (111usize) , S (i (55usize))) , (s (70usize) , S (i (49usize))) , (s (87usize) , S (i (50usize))) , (s (103usize) , S (i (51usize)))] }
        }),
        (i(19usize), {
            g! { [(s (108usize) , R (p (57usize))) , (s (70usize) , R (p (57usize))) , (s (109usize) , R (p (57usize))) , (s (107usize) , R (p (57usize))) , (s (110usize) , R (p (57usize))) , (s (111usize) , R (p (57usize))) , (s (87usize) , R (p (57usize))) , (s (103usize) , R (p (57usize)))] }
        }),
        (i(20usize), {
            g! { [(s (108usize) , R (p (58usize))) , (s (70usize) , R (p (58usize))) , (s (109usize) , R (p (58usize))) , (s (107usize) , R (p (58usize))) , (s (110usize) , R (p (58usize))) , (s (111usize) , R (p (58usize))) , (s (87usize) , R (p (58usize))) , (s (103usize) , R (p (58usize)))] }
        }),
        (i(21usize), {
            g! { [(s (1usize) , R (p (59usize))) , (s (65usize) , R (p (59usize))) , (s (74usize) , R (p (59usize))) , (s (66usize) , R (p (59usize))) , (s (67usize) , R (p (59usize))) , (s (68usize) , R (p (59usize))) , (s (69usize) , R (p (59usize))) , (s (71usize) , R (p (59usize))) , (s (108usize) , R (p (59usize))) , (s (72usize) , R (p (59usize))) , (s (73usize) , R (p (59usize))) , (s (101usize) , R (p (59usize))) , (s (106usize) , R (p (59usize)))] }
        }),
        (i(22usize), {
            g! { [(s (1usize) , R (p (60usize))) , (s (65usize) , R (p (60usize))) , (s (74usize) , R (p (60usize))) , (s (66usize) , R (p (60usize))) , (s (67usize) , R (p (60usize))) , (s (68usize) , R (p (60usize))) , (s (69usize) , R (p (60usize))) , (s (71usize) , R (p (60usize))) , (s (108usize) , R (p (60usize))) , (s (72usize) , R (p (60usize))) , (s (73usize) , R (p (60usize))) , (s (101usize) , R (p (60usize))) , (s (106usize) , R (p (60usize)))] }
        }),
        (i(23usize), {
            g! { [(s (1usize) , R (p (61usize))) , (s (65usize) , R (p (61usize))) , (s (74usize) , R (p (61usize))) , (s (66usize) , R (p (61usize))) , (s (67usize) , R (p (61usize))) , (s (68usize) , R (p (61usize))) , (s (69usize) , R (p (61usize))) , (s (71usize) , R (p (61usize))) , (s (108usize) , R (p (61usize))) , (s (72usize) , R (p (61usize))) , (s (73usize) , R (p (61usize))) , (s (101usize) , R (p (61usize))) , (s (106usize) , R (p (61usize)))] }
        }),
        (i(24usize), {
            g! { [(s (108usize) , R (p (63usize))) , (s (70usize) , R (p (63usize))) , (s (109usize) , R (p (63usize))) , (s (107usize) , R (p (63usize))) , (s (110usize) , R (p (63usize))) , (s (111usize) , R (p (63usize))) , (s (87usize) , R (p (63usize))) , (s (103usize) , R (p (63usize)))] }
        }),
        (i(25usize), {
            g! { [(s (108usize) , R (p (64usize))) , (s (70usize) , R (p (64usize))) , (s (109usize) , R (p (64usize))) , (s (107usize) , R (p (64usize))) , (s (110usize) , R (p (64usize))) , (s (111usize) , R (p (64usize))) , (s (87usize) , R (p (64usize))) , (s (103usize) , R (p (64usize)))] }
        }),
        (i(26usize), {
            g! { [(s (108usize) , R (p (65usize))) , (s (70usize) , R (p (65usize))) , (s (109usize) , R (p (65usize))) , (s (107usize) , R (p (65usize))) , (s (110usize) , R (p (65usize))) , (s (111usize) , R (p (65usize))) , (s (87usize) , R (p (65usize))) , (s (103usize) , R (p (65usize)))] }
        }),
        (i(27usize), {
            g! { [(s (1usize) , R (p (66usize))) , (s (65usize) , R (p (66usize))) , (s (74usize) , R (p (66usize))) , (s (66usize) , R (p (66usize))) , (s (67usize) , R (p (66usize))) , (s (68usize) , R (p (66usize))) , (s (69usize) , R (p (66usize))) , (s (71usize) , R (p (66usize))) , (s (108usize) , R (p (66usize))) , (s (72usize) , R (p (66usize))) , (s (73usize) , R (p (66usize))) , (s (101usize) , R (p (66usize))) , (s (106usize) , R (p (66usize)))] }
        }),
        (i(28usize), {
            g! { [(s (108usize) , R (p (91usize))) , (s (70usize) , R (p (91usize))) , (s (109usize) , R (p (91usize))) , (s (107usize) , R (p (91usize))) , (s (110usize) , R (p (91usize))) , (s (111usize) , R (p (91usize))) , (s (87usize) , R (p (91usize))) , (s (103usize) , R (p (91usize)))] }
        }),
        (i(29usize), {
            g! { [(s (86usize) , R (p (98usize))) , (s (1usize) , R (p (98usize))) , (s (65usize) , R (p (98usize))) , (s (74usize) , R (p (98usize))) , (s (66usize) , R (p (98usize))) , (s (67usize) , R (p (98usize))) , (s (68usize) , R (p (98usize))) , (s (69usize) , R (p (98usize))) , (s (71usize) , R (p (98usize))) , (s (108usize) , R (p (98usize))) , (s (72usize) , R (p (98usize))) , (s (73usize) , R (p (98usize))) , (s (101usize) , R (p (98usize))) , (s (93usize) , R (p (98usize))) , (s (94usize) , R (p (98usize))) , (s (95usize) , R (p (98usize))) , (s (96usize) , R (p (98usize))) , (s (97usize) , R (p (98usize))) , (s (98usize) , R (p (98usize))) , (s (88usize) , R (p (98usize))) , (s (89usize) , R (p (98usize))) , (s (90usize) , R (p (98usize))) , (s (91usize) , R (p (98usize))) , (s (92usize) , R (p (98usize))) , (s (99usize) , R (p (98usize))) , (s (100usize) , R (p (98usize))) , (s (84usize) , R (p (98usize))) , (s (105usize) , R (p (98usize))) , (s (104usize) , R (p (98usize))) , (s (81usize) , R (p (98usize))) , (s (106usize) , R (p (98usize)))] }
        }),
        (i(30usize), {
            g! { [(s (1usize) , R (p (4usize))) , (s (106usize) , R (p (4usize)))] }
        }),
        (i(31usize), {
            g! { [(s (1usize) , R (p (5usize))) , (s (65usize) , R (p (5usize))) , (s (74usize) , R (p (5usize))) , (s (66usize) , R (p (5usize))) , (s (67usize) , R (p (5usize))) , (s (68usize) , R (p (5usize))) , (s (69usize) , R (p (5usize))) , (s (71usize) , R (p (5usize))) , (s (108usize) , R (p (5usize))) , (s (72usize) , R (p (5usize))) , (s (73usize) , R (p (5usize))) , (s (101usize) , R (p (5usize))) , (s (106usize) , R (p (5usize))) , (s (93usize) , S (i (86usize))) , (s (94usize) , S (i (87usize))) , (s (95usize) , S (i (88usize))) , (s (96usize) , S (i (89usize))) , (s (97usize) , S (i (90usize))) , (s (98usize) , S (i (91usize))) , (s (88usize) , S (i (81usize))) , (s (89usize) , S (i (82usize))) , (s (90usize) , S (i (83usize))) , (s (91usize) , S (i (84usize))) , (s (92usize) , S (i (85usize))) , (s (99usize) , S (i (92usize))) , (s (100usize) , S (i (93usize)))] }
        }),
        (i(32usize), {
            g! { [(s (1usize) , R (p (34usize))) , (s (65usize) , R (p (34usize))) , (s (74usize) , R (p (34usize))) , (s (66usize) , R (p (34usize))) , (s (67usize) , R (p (34usize))) , (s (68usize) , R (p (34usize))) , (s (69usize) , R (p (34usize))) , (s (71usize) , R (p (34usize))) , (s (108usize) , R (p (34usize))) , (s (72usize) , R (p (34usize))) , (s (73usize) , R (p (34usize))) , (s (101usize) , R (p (34usize))) , (s (93usize) , R (p (34usize))) , (s (94usize) , R (p (34usize))) , (s (95usize) , R (p (34usize))) , (s (96usize) , R (p (34usize))) , (s (97usize) , R (p (34usize))) , (s (98usize) , R (p (34usize))) , (s (88usize) , R (p (34usize))) , (s (89usize) , R (p (34usize))) , (s (90usize) , R (p (34usize))) , (s (91usize) , R (p (34usize))) , (s (92usize) , R (p (34usize))) , (s (99usize) , R (p (34usize))) , (s (100usize) , R (p (34usize))) , (s (84usize) , R (p (34usize))) , (s (105usize) , R (p (34usize))) , (s (104usize) , R (p (34usize))) , (s (81usize) , R (p (34usize))) , (s (106usize) , R (p (34usize)))] }
        }),
        (i(33usize), {
            g! { [(s (75usize) , S (i (97usize))) , (s (85usize) , S (i (99usize))) , (s (78usize) , S (i (98usize)))] }
        }),
        (i(34usize), {
            g! { [(s (1usize) , R (p (31usize))) , (s (65usize) , R (p (31usize))) , (s (74usize) , R (p (31usize))) , (s (66usize) , R (p (31usize))) , (s (67usize) , R (p (31usize))) , (s (68usize) , R (p (31usize))) , (s (69usize) , R (p (31usize))) , (s (71usize) , R (p (31usize))) , (s (108usize) , R (p (31usize))) , (s (72usize) , R (p (31usize))) , (s (73usize) , R (p (31usize))) , (s (101usize) , R (p (31usize))) , (s (93usize) , R (p (31usize))) , (s (94usize) , R (p (31usize))) , (s (95usize) , R (p (31usize))) , (s (96usize) , R (p (31usize))) , (s (97usize) , R (p (31usize))) , (s (98usize) , R (p (31usize))) , (s (88usize) , R (p (31usize))) , (s (89usize) , R (p (31usize))) , (s (90usize) , R (p (31usize))) , (s (91usize) , R (p (31usize))) , (s (92usize) , R (p (31usize))) , (s (99usize) , R (p (31usize))) , (s (100usize) , R (p (31usize))) , (s (84usize) , R (p (31usize))) , (s (105usize) , R (p (31usize))) , (s (104usize) , R (p (31usize))) , (s (81usize) , R (p (31usize))) , (s (106usize) , R (p (31usize)))] }
        }),
        (i(35usize), {
            g! { [(s (1usize) , R (p (28usize))) , (s (65usize) , R (p (28usize))) , (s (74usize) , R (p (28usize))) , (s (66usize) , R (p (28usize))) , (s (67usize) , R (p (28usize))) , (s (68usize) , R (p (28usize))) , (s (69usize) , R (p (28usize))) , (s (71usize) , R (p (28usize))) , (s (108usize) , R (p (28usize))) , (s (72usize) , R (p (28usize))) , (s (73usize) , R (p (28usize))) , (s (101usize) , R (p (28usize))) , (s (93usize) , R (p (28usize))) , (s (94usize) , R (p (28usize))) , (s (95usize) , R (p (28usize))) , (s (96usize) , R (p (28usize))) , (s (97usize) , R (p (28usize))) , (s (98usize) , R (p (28usize))) , (s (88usize) , R (p (28usize))) , (s (89usize) , R (p (28usize))) , (s (90usize) , R (p (28usize))) , (s (91usize) , R (p (28usize))) , (s (92usize) , R (p (28usize))) , (s (99usize) , R (p (28usize))) , (s (100usize) , R (p (28usize))) , (s (84usize) , R (p (28usize))) , (s (105usize) , R (p (28usize))) , (s (104usize) , R (p (28usize))) , (s (81usize) , R (p (28usize))) , (s (106usize) , R (p (28usize)))] }
        }),
        (i(36usize), {
            g! { [(s (1usize) , R (p (29usize))) , (s (65usize) , R (p (29usize))) , (s (74usize) , R (p (29usize))) , (s (66usize) , R (p (29usize))) , (s (67usize) , R (p (29usize))) , (s (68usize) , R (p (29usize))) , (s (69usize) , R (p (29usize))) , (s (71usize) , R (p (29usize))) , (s (108usize) , R (p (29usize))) , (s (72usize) , R (p (29usize))) , (s (73usize) , R (p (29usize))) , (s (101usize) , R (p (29usize))) , (s (93usize) , R (p (29usize))) , (s (94usize) , R (p (29usize))) , (s (95usize) , R (p (29usize))) , (s (96usize) , R (p (29usize))) , (s (97usize) , R (p (29usize))) , (s (98usize) , R (p (29usize))) , (s (88usize) , R (p (29usize))) , (s (89usize) , R (p (29usize))) , (s (90usize) , R (p (29usize))) , (s (91usize) , R (p (29usize))) , (s (92usize) , R (p (29usize))) , (s (99usize) , R (p (29usize))) , (s (100usize) , R (p (29usize))) , (s (84usize) , R (p (29usize))) , (s (105usize) , R (p (29usize))) , (s (104usize) , R (p (29usize))) , (s (81usize) , R (p (29usize))) , (s (106usize) , R (p (29usize)))] }
        }),
        (i(37usize), {
            g! { [(s (1usize) , R (p (30usize))) , (s (65usize) , R (p (30usize))) , (s (74usize) , R (p (30usize))) , (s (66usize) , R (p (30usize))) , (s (67usize) , R (p (30usize))) , (s (68usize) , R (p (30usize))) , (s (69usize) , R (p (30usize))) , (s (71usize) , R (p (30usize))) , (s (108usize) , R (p (30usize))) , (s (72usize) , R (p (30usize))) , (s (73usize) , R (p (30usize))) , (s (101usize) , R (p (30usize))) , (s (93usize) , R (p (30usize))) , (s (94usize) , R (p (30usize))) , (s (95usize) , R (p (30usize))) , (s (96usize) , R (p (30usize))) , (s (97usize) , R (p (30usize))) , (s (98usize) , R (p (30usize))) , (s (88usize) , R (p (30usize))) , (s (89usize) , R (p (30usize))) , (s (90usize) , R (p (30usize))) , (s (91usize) , R (p (30usize))) , (s (92usize) , R (p (30usize))) , (s (99usize) , R (p (30usize))) , (s (100usize) , R (p (30usize))) , (s (84usize) , R (p (30usize))) , (s (105usize) , R (p (30usize))) , (s (104usize) , R (p (30usize))) , (s (81usize) , R (p (30usize))) , (s (106usize) , R (p (30usize)))] }
        }),
        (i(38usize), {
            g! { [(s (1usize) , R (p (32usize))) , (s (65usize) , R (p (32usize))) , (s (74usize) , R (p (32usize))) , (s (66usize) , R (p (32usize))) , (s (67usize) , R (p (32usize))) , (s (68usize) , R (p (32usize))) , (s (69usize) , R (p (32usize))) , (s (71usize) , R (p (32usize))) , (s (108usize) , R (p (32usize))) , (s (72usize) , R (p (32usize))) , (s (73usize) , R (p (32usize))) , (s (101usize) , R (p (32usize))) , (s (93usize) , R (p (32usize))) , (s (94usize) , R (p (32usize))) , (s (95usize) , R (p (32usize))) , (s (96usize) , R (p (32usize))) , (s (97usize) , R (p (32usize))) , (s (98usize) , R (p (32usize))) , (s (88usize) , R (p (32usize))) , (s (89usize) , R (p (32usize))) , (s (90usize) , R (p (32usize))) , (s (91usize) , R (p (32usize))) , (s (92usize) , R (p (32usize))) , (s (99usize) , R (p (32usize))) , (s (100usize) , R (p (32usize))) , (s (84usize) , R (p (32usize))) , (s (105usize) , R (p (32usize))) , (s (104usize) , R (p (32usize))) , (s (81usize) , R (p (32usize))) , (s (106usize) , R (p (32usize)))] }
        }),
        (i(39usize), {
            g! { [(s (1usize) , R (p (33usize))) , (s (65usize) , R (p (33usize))) , (s (74usize) , R (p (33usize))) , (s (66usize) , R (p (33usize))) , (s (67usize) , R (p (33usize))) , (s (68usize) , R (p (33usize))) , (s (69usize) , R (p (33usize))) , (s (71usize) , R (p (33usize))) , (s (108usize) , R (p (33usize))) , (s (72usize) , R (p (33usize))) , (s (73usize) , R (p (33usize))) , (s (101usize) , R (p (33usize))) , (s (93usize) , R (p (33usize))) , (s (94usize) , R (p (33usize))) , (s (95usize) , R (p (33usize))) , (s (96usize) , R (p (33usize))) , (s (97usize) , R (p (33usize))) , (s (98usize) , R (p (33usize))) , (s (88usize) , R (p (33usize))) , (s (89usize) , R (p (33usize))) , (s (90usize) , R (p (33usize))) , (s (91usize) , R (p (33usize))) , (s (92usize) , R (p (33usize))) , (s (99usize) , R (p (33usize))) , (s (100usize) , R (p (33usize))) , (s (84usize) , R (p (33usize))) , (s (105usize) , R (p (33usize))) , (s (104usize) , R (p (33usize))) , (s (81usize) , R (p (33usize))) , (s (106usize) , R (p (33usize)))] }
        }),
        (i(40usize), {
            g! { [(s (1usize) , R (p (35usize))) , (s (65usize) , R (p (35usize))) , (s (74usize) , R (p (35usize))) , (s (66usize) , R (p (35usize))) , (s (67usize) , R (p (35usize))) , (s (68usize) , R (p (35usize))) , (s (69usize) , R (p (35usize))) , (s (71usize) , R (p (35usize))) , (s (108usize) , R (p (35usize))) , (s (72usize) , R (p (35usize))) , (s (73usize) , R (p (35usize))) , (s (101usize) , R (p (35usize))) , (s (93usize) , R (p (35usize))) , (s (94usize) , R (p (35usize))) , (s (95usize) , R (p (35usize))) , (s (96usize) , R (p (35usize))) , (s (97usize) , R (p (35usize))) , (s (98usize) , R (p (35usize))) , (s (88usize) , R (p (35usize))) , (s (89usize) , R (p (35usize))) , (s (90usize) , R (p (35usize))) , (s (91usize) , R (p (35usize))) , (s (92usize) , R (p (35usize))) , (s (99usize) , R (p (35usize))) , (s (100usize) , R (p (35usize))) , (s (84usize) , R (p (35usize))) , (s (105usize) , R (p (35usize))) , (s (104usize) , R (p (35usize))) , (s (81usize) , R (p (35usize))) , (s (106usize) , R (p (35usize)))] }
        }),
        (i(41usize), {
            g! { [(s (1usize) , R (p (36usize))) , (s (65usize) , R (p (36usize))) , (s (74usize) , R (p (36usize))) , (s (66usize) , R (p (36usize))) , (s (67usize) , R (p (36usize))) , (s (68usize) , R (p (36usize))) , (s (69usize) , R (p (36usize))) , (s (71usize) , R (p (36usize))) , (s (108usize) , R (p (36usize))) , (s (72usize) , R (p (36usize))) , (s (73usize) , R (p (36usize))) , (s (101usize) , R (p (36usize))) , (s (93usize) , R (p (36usize))) , (s (94usize) , R (p (36usize))) , (s (95usize) , R (p (36usize))) , (s (96usize) , R (p (36usize))) , (s (97usize) , R (p (36usize))) , (s (98usize) , R (p (36usize))) , (s (88usize) , R (p (36usize))) , (s (89usize) , R (p (36usize))) , (s (90usize) , R (p (36usize))) , (s (91usize) , R (p (36usize))) , (s (92usize) , R (p (36usize))) , (s (99usize) , R (p (36usize))) , (s (100usize) , R (p (36usize))) , (s (84usize) , R (p (36usize))) , (s (105usize) , R (p (36usize))) , (s (104usize) , R (p (36usize))) , (s (81usize) , R (p (36usize))) , (s (106usize) , R (p (36usize)))] }
        }),
        (i(42usize), {
            g! { [(s (1usize) , R (p (37usize))) , (s (65usize) , R (p (37usize))) , (s (74usize) , R (p (37usize))) , (s (66usize) , R (p (37usize))) , (s (67usize) , R (p (37usize))) , (s (68usize) , R (p (37usize))) , (s (69usize) , R (p (37usize))) , (s (71usize) , R (p (37usize))) , (s (108usize) , R (p (37usize))) , (s (72usize) , R (p (37usize))) , (s (73usize) , R (p (37usize))) , (s (101usize) , R (p (37usize))) , (s (93usize) , R (p (37usize))) , (s (94usize) , R (p (37usize))) , (s (95usize) , R (p (37usize))) , (s (96usize) , R (p (37usize))) , (s (97usize) , R (p (37usize))) , (s (98usize) , R (p (37usize))) , (s (88usize) , R (p (37usize))) , (s (89usize) , R (p (37usize))) , (s (90usize) , R (p (37usize))) , (s (91usize) , R (p (37usize))) , (s (92usize) , R (p (37usize))) , (s (99usize) , R (p (37usize))) , (s (100usize) , R (p (37usize))) , (s (84usize) , R (p (37usize))) , (s (105usize) , R (p (37usize))) , (s (104usize) , R (p (37usize))) , (s (81usize) , R (p (37usize))) , (s (106usize) , R (p (37usize)))] }
        }),
        (i(43usize), {
            g! { [(s (1usize) , R (p (38usize))) , (s (65usize) , R (p (38usize))) , (s (74usize) , R (p (38usize))) , (s (66usize) , R (p (38usize))) , (s (67usize) , R (p (38usize))) , (s (68usize) , R (p (38usize))) , (s (69usize) , R (p (38usize))) , (s (71usize) , R (p (38usize))) , (s (108usize) , R (p (38usize))) , (s (72usize) , R (p (38usize))) , (s (73usize) , R (p (38usize))) , (s (101usize) , R (p (38usize))) , (s (93usize) , R (p (38usize))) , (s (94usize) , R (p (38usize))) , (s (95usize) , R (p (38usize))) , (s (96usize) , R (p (38usize))) , (s (97usize) , R (p (38usize))) , (s (98usize) , R (p (38usize))) , (s (88usize) , R (p (38usize))) , (s (89usize) , R (p (38usize))) , (s (90usize) , R (p (38usize))) , (s (91usize) , R (p (38usize))) , (s (92usize) , R (p (38usize))) , (s (99usize) , R (p (38usize))) , (s (100usize) , R (p (38usize))) , (s (84usize) , R (p (38usize))) , (s (105usize) , R (p (38usize))) , (s (104usize) , R (p (38usize))) , (s (81usize) , R (p (38usize))) , (s (106usize) , R (p (38usize)))] }
        }),
        (i(44usize), {
            g! { [(s (103usize) , S (i (51usize)))] }
        }),
        (i(45usize), {
            g! { [(s (109usize) , S (i (53usize))) , (s (107usize) , S (i (52usize))) , (s (108usize) , S (i (29usize))) , (s (110usize) , S (i (54usize))) , (s (111usize) , S (i (55usize))) , (s (70usize) , S (i (49usize))) , (s (87usize) , S (i (50usize))) , (s (103usize) , S (i (51usize)))] }
        }),
        (i(46usize), {
            g! { [(s (1usize) , R (p (41usize))) , (s (65usize) , R (p (41usize))) , (s (74usize) , R (p (41usize))) , (s (66usize) , R (p (41usize))) , (s (67usize) , R (p (41usize))) , (s (68usize) , R (p (41usize))) , (s (69usize) , R (p (41usize))) , (s (71usize) , R (p (41usize))) , (s (108usize) , R (p (41usize))) , (s (72usize) , R (p (41usize))) , (s (73usize) , R (p (41usize))) , (s (101usize) , R (p (41usize))) , (s (93usize) , R (p (41usize))) , (s (94usize) , R (p (41usize))) , (s (95usize) , R (p (41usize))) , (s (96usize) , R (p (41usize))) , (s (97usize) , R (p (41usize))) , (s (98usize) , R (p (41usize))) , (s (88usize) , R (p (41usize))) , (s (89usize) , R (p (41usize))) , (s (90usize) , R (p (41usize))) , (s (91usize) , R (p (41usize))) , (s (92usize) , R (p (41usize))) , (s (99usize) , R (p (41usize))) , (s (100usize) , R (p (41usize))) , (s (84usize) , R (p (41usize))) , (s (105usize) , R (p (41usize))) , (s (104usize) , R (p (41usize))) , (s (81usize) , R (p (41usize))) , (s (106usize) , R (p (41usize)))] }
        }),
        (i(47usize), {
            g! { [(s (1usize) , R (p (42usize))) , (s (65usize) , R (p (42usize))) , (s (74usize) , R (p (42usize))) , (s (66usize) , R (p (42usize))) , (s (67usize) , R (p (42usize))) , (s (68usize) , R (p (42usize))) , (s (69usize) , R (p (42usize))) , (s (71usize) , R (p (42usize))) , (s (108usize) , R (p (42usize))) , (s (72usize) , R (p (42usize))) , (s (73usize) , R (p (42usize))) , (s (101usize) , R (p (42usize))) , (s (93usize) , R (p (42usize))) , (s (94usize) , R (p (42usize))) , (s (95usize) , R (p (42usize))) , (s (96usize) , R (p (42usize))) , (s (97usize) , R (p (42usize))) , (s (98usize) , R (p (42usize))) , (s (88usize) , R (p (42usize))) , (s (89usize) , R (p (42usize))) , (s (90usize) , R (p (42usize))) , (s (91usize) , R (p (42usize))) , (s (92usize) , R (p (42usize))) , (s (99usize) , R (p (42usize))) , (s (100usize) , R (p (42usize))) , (s (84usize) , R (p (42usize))) , (s (105usize) , R (p (42usize))) , (s (104usize) , R (p (42usize))) , (s (81usize) , R (p (42usize))) , (s (106usize) , R (p (42usize)))] }
        }),
        (i(48usize), {
            g! { [(s (1usize) , R (p (43usize))) , (s (65usize) , R (p (43usize))) , (s (74usize) , R (p (43usize))) , (s (66usize) , R (p (43usize))) , (s (67usize) , R (p (43usize))) , (s (68usize) , R (p (43usize))) , (s (69usize) , R (p (43usize))) , (s (71usize) , R (p (43usize))) , (s (108usize) , R (p (43usize))) , (s (72usize) , R (p (43usize))) , (s (73usize) , R (p (43usize))) , (s (101usize) , R (p (43usize))) , (s (93usize) , R (p (43usize))) , (s (94usize) , R (p (43usize))) , (s (95usize) , R (p (43usize))) , (s (96usize) , R (p (43usize))) , (s (97usize) , R (p (43usize))) , (s (98usize) , R (p (43usize))) , (s (88usize) , R (p (43usize))) , (s (89usize) , R (p (43usize))) , (s (90usize) , R (p (43usize))) , (s (91usize) , R (p (43usize))) , (s (92usize) , R (p (43usize))) , (s (99usize) , R (p (43usize))) , (s (100usize) , R (p (43usize))) , (s (84usize) , R (p (43usize))) , (s (105usize) , R (p (43usize))) , (s (104usize) , R (p (43usize))) , (s (81usize) , R (p (43usize))) , (s (106usize) , R (p (43usize)))] }
        }),
        (i(49usize), {
            g! { [(s (75usize) , R (p (62usize))) , (s (85usize) , R (p (62usize))) , (s (78usize) , R (p (62usize)))] }
        }),
        (i(50usize), {
            g! { [(s (103usize) , R (p (77usize)))] }
        }),
        (i(51usize), {
            g! { [(s (108usize) , R (p (93usize))) , (s (70usize) , R (p (93usize))) , (s (109usize) , R (p (93usize))) , (s (107usize) , R (p (93usize))) , (s (110usize) , R (p (93usize))) , (s (111usize) , R (p (93usize))) , (s (87usize) , R (p (93usize))) , (s (103usize) , R (p (93usize)))] }
        }),
        (i(52usize), {
            g! { [(s (1usize) , R (p (97usize))) , (s (65usize) , R (p (97usize))) , (s (74usize) , R (p (97usize))) , (s (66usize) , R (p (97usize))) , (s (67usize) , R (p (97usize))) , (s (68usize) , R (p (97usize))) , (s (69usize) , R (p (97usize))) , (s (71usize) , R (p (97usize))) , (s (108usize) , R (p (97usize))) , (s (72usize) , R (p (97usize))) , (s (73usize) , R (p (97usize))) , (s (101usize) , R (p (97usize))) , (s (93usize) , R (p (97usize))) , (s (94usize) , R (p (97usize))) , (s (95usize) , R (p (97usize))) , (s (96usize) , R (p (97usize))) , (s (97usize) , R (p (97usize))) , (s (98usize) , R (p (97usize))) , (s (88usize) , R (p (97usize))) , (s (89usize) , R (p (97usize))) , (s (90usize) , R (p (97usize))) , (s (91usize) , R (p (97usize))) , (s (92usize) , R (p (97usize))) , (s (99usize) , R (p (97usize))) , (s (100usize) , R (p (97usize))) , (s (84usize) , R (p (97usize))) , (s (105usize) , R (p (97usize))) , (s (104usize) , R (p (97usize))) , (s (81usize) , R (p (97usize))) , (s (106usize) , R (p (97usize)))] }
        }),
        (i(53usize), {
            g! { [(s (1usize) , R (p (99usize))) , (s (65usize) , R (p (99usize))) , (s (74usize) , R (p (99usize))) , (s (66usize) , R (p (99usize))) , (s (67usize) , R (p (99usize))) , (s (68usize) , R (p (99usize))) , (s (69usize) , R (p (99usize))) , (s (71usize) , R (p (99usize))) , (s (108usize) , R (p (99usize))) , (s (72usize) , R (p (99usize))) , (s (73usize) , R (p (99usize))) , (s (101usize) , R (p (99usize))) , (s (93usize) , R (p (99usize))) , (s (94usize) , R (p (99usize))) , (s (95usize) , R (p (99usize))) , (s (96usize) , R (p (99usize))) , (s (97usize) , R (p (99usize))) , (s (98usize) , R (p (99usize))) , (s (88usize) , R (p (99usize))) , (s (89usize) , R (p (99usize))) , (s (90usize) , R (p (99usize))) , (s (91usize) , R (p (99usize))) , (s (92usize) , R (p (99usize))) , (s (99usize) , R (p (99usize))) , (s (100usize) , R (p (99usize))) , (s (84usize) , R (p (99usize))) , (s (105usize) , R (p (99usize))) , (s (104usize) , R (p (99usize))) , (s (81usize) , R (p (99usize))) , (s (106usize) , R (p (99usize)))] }
        }),
        (i(54usize), {
            g! { [(s (1usize) , R (p (100usize))) , (s (65usize) , R (p (100usize))) , (s (74usize) , R (p (100usize))) , (s (66usize) , R (p (100usize))) , (s (67usize) , R (p (100usize))) , (s (68usize) , R (p (100usize))) , (s (69usize) , R (p (100usize))) , (s (71usize) , R (p (100usize))) , (s (108usize) , R (p (100usize))) , (s (72usize) , R (p (100usize))) , (s (73usize) , R (p (100usize))) , (s (101usize) , R (p (100usize))) , (s (93usize) , R (p (100usize))) , (s (94usize) , R (p (100usize))) , (s (95usize) , R (p (100usize))) , (s (96usize) , R (p (100usize))) , (s (97usize) , R (p (100usize))) , (s (98usize) , R (p (100usize))) , (s (88usize) , R (p (100usize))) , (s (89usize) , R (p (100usize))) , (s (90usize) , R (p (100usize))) , (s (91usize) , R (p (100usize))) , (s (92usize) , R (p (100usize))) , (s (99usize) , R (p (100usize))) , (s (100usize) , R (p (100usize))) , (s (84usize) , R (p (100usize))) , (s (105usize) , R (p (100usize))) , (s (104usize) , R (p (100usize))) , (s (81usize) , R (p (100usize))) , (s (106usize) , R (p (100usize)))] }
        }),
        (i(55usize), {
            g! { [(s (1usize) , R (p (101usize))) , (s (65usize) , R (p (101usize))) , (s (74usize) , R (p (101usize))) , (s (66usize) , R (p (101usize))) , (s (67usize) , R (p (101usize))) , (s (68usize) , R (p (101usize))) , (s (69usize) , R (p (101usize))) , (s (71usize) , R (p (101usize))) , (s (108usize) , R (p (101usize))) , (s (72usize) , R (p (101usize))) , (s (73usize) , R (p (101usize))) , (s (101usize) , R (p (101usize))) , (s (93usize) , R (p (101usize))) , (s (94usize) , R (p (101usize))) , (s (95usize) , R (p (101usize))) , (s (96usize) , R (p (101usize))) , (s (97usize) , R (p (101usize))) , (s (98usize) , R (p (101usize))) , (s (88usize) , R (p (101usize))) , (s (89usize) , R (p (101usize))) , (s (90usize) , R (p (101usize))) , (s (91usize) , R (p (101usize))) , (s (92usize) , R (p (101usize))) , (s (99usize) , R (p (101usize))) , (s (100usize) , R (p (101usize))) , (s (84usize) , R (p (101usize))) , (s (105usize) , R (p (101usize))) , (s (104usize) , R (p (101usize))) , (s (81usize) , R (p (101usize))) , (s (106usize) , R (p (101usize)))] }
        }),
        (i(56usize), {
            g! { [(s (1usize) , R (p (7usize))) , (s (65usize) , R (p (7usize))) , (s (74usize) , R (p (7usize))) , (s (66usize) , R (p (7usize))) , (s (67usize) , R (p (7usize))) , (s (68usize) , R (p (7usize))) , (s (69usize) , R (p (7usize))) , (s (71usize) , R (p (7usize))) , (s (108usize) , R (p (7usize))) , (s (72usize) , R (p (7usize))) , (s (73usize) , R (p (7usize))) , (s (101usize) , R (p (7usize))) , (s (106usize) , R (p (7usize))) , (s (93usize) , S (i (86usize))) , (s (94usize) , S (i (87usize))) , (s (95usize) , S (i (88usize))) , (s (96usize) , S (i (89usize))) , (s (97usize) , S (i (90usize))) , (s (98usize) , S (i (91usize))) , (s (88usize) , S (i (81usize))) , (s (89usize) , S (i (82usize))) , (s (90usize) , S (i (83usize))) , (s (91usize) , S (i (84usize))) , (s (92usize) , S (i (85usize))) , (s (99usize) , S (i (92usize))) , (s (100usize) , S (i (93usize)))] }
        }),
        (i(57usize), {
            g! { [(s (1usize) , R (p (11usize))) , (s (65usize) , R (p (11usize))) , (s (74usize) , R (p (11usize))) , (s (66usize) , R (p (11usize))) , (s (67usize) , R (p (11usize))) , (s (68usize) , R (p (11usize))) , (s (69usize) , R (p (11usize))) , (s (71usize) , R (p (11usize))) , (s (108usize) , R (p (11usize))) , (s (72usize) , R (p (11usize))) , (s (73usize) , R (p (11usize))) , (s (101usize) , R (p (11usize))) , (s (106usize) , R (p (11usize))) , (s (93usize) , S (i (86usize))) , (s (94usize) , S (i (87usize))) , (s (95usize) , S (i (88usize))) , (s (96usize) , S (i (89usize))) , (s (97usize) , S (i (90usize))) , (s (98usize) , S (i (91usize))) , (s (88usize) , S (i (81usize))) , (s (89usize) , S (i (82usize))) , (s (90usize) , S (i (83usize))) , (s (91usize) , S (i (84usize))) , (s (92usize) , S (i (85usize))) , (s (99usize) , S (i (92usize))) , (s (100usize) , S (i (93usize)))] }
        }),
        (i(58usize), {
            g! { [(s (109usize) , S (i (53usize))) , (s (107usize) , S (i (52usize))) , (s (108usize) , S (i (29usize))) , (s (110usize) , S (i (54usize))) , (s (111usize) , S (i (55usize))) , (s (70usize) , S (i (49usize))) , (s (87usize) , S (i (50usize))) , (s (103usize) , S (i (51usize)))] }
        }),
        (i(59usize), {
            g! { [(s (108usize) , R (p (76usize))) , (s (70usize) , R (p (76usize))) , (s (109usize) , R (p (76usize))) , (s (107usize) , R (p (76usize))) , (s (110usize) , R (p (76usize))) , (s (111usize) , R (p (76usize))) , (s (87usize) , R (p (76usize))) , (s (103usize) , R (p (76usize)))] }
        }),
        (i(60usize), {
            g! { [(s (1usize) , R (p (13usize))) , (s (65usize) , R (p (13usize))) , (s (74usize) , R (p (13usize))) , (s (66usize) , R (p (13usize))) , (s (67usize) , R (p (13usize))) , (s (68usize) , R (p (13usize))) , (s (69usize) , R (p (13usize))) , (s (71usize) , R (p (13usize))) , (s (108usize) , R (p (13usize))) , (s (72usize) , R (p (13usize))) , (s (73usize) , R (p (13usize))) , (s (101usize) , R (p (13usize))) , (s (106usize) , R (p (13usize))) , (s (93usize) , S (i (86usize))) , (s (94usize) , S (i (87usize))) , (s (95usize) , S (i (88usize))) , (s (96usize) , S (i (89usize))) , (s (97usize) , S (i (90usize))) , (s (98usize) , S (i (91usize))) , (s (88usize) , S (i (81usize))) , (s (89usize) , S (i (82usize))) , (s (90usize) , S (i (83usize))) , (s (91usize) , S (i (84usize))) , (s (92usize) , S (i (85usize))) , (s (99usize) , S (i (92usize))) , (s (100usize) , S (i (93usize)))] }
        }),
        (i(61usize), {
            g! { [(s (84usize) , S (i (104usize))) , (s (93usize) , S (i (86usize))) , (s (94usize) , S (i (87usize))) , (s (95usize) , S (i (88usize))) , (s (96usize) , S (i (89usize))) , (s (97usize) , S (i (90usize))) , (s (98usize) , S (i (91usize))) , (s (88usize) , S (i (81usize))) , (s (89usize) , S (i (82usize))) , (s (90usize) , S (i (83usize))) , (s (91usize) , S (i (84usize))) , (s (92usize) , S (i (85usize))) , (s (99usize) , S (i (92usize))) , (s (100usize) , S (i (93usize)))] }
        }),
        (i(62usize), {
            g! { [(s (1usize) , R (p (17usize))) , (s (65usize) , R (p (17usize))) , (s (74usize) , R (p (17usize))) , (s (66usize) , R (p (17usize))) , (s (67usize) , R (p (17usize))) , (s (68usize) , R (p (17usize))) , (s (69usize) , R (p (17usize))) , (s (71usize) , R (p (17usize))) , (s (108usize) , R (p (17usize))) , (s (72usize) , R (p (17usize))) , (s (73usize) , R (p (17usize))) , (s (101usize) , R (p (17usize))) , (s (106usize) , R (p (17usize)))] }
        }),
        (i(63usize), {
            g! { [(s (1usize) , R (p (18usize))) , (s (65usize) , R (p (18usize))) , (s (74usize) , R (p (18usize))) , (s (66usize) , R (p (18usize))) , (s (67usize) , R (p (18usize))) , (s (68usize) , R (p (18usize))) , (s (69usize) , R (p (18usize))) , (s (71usize) , R (p (18usize))) , (s (108usize) , R (p (18usize))) , (s (72usize) , R (p (18usize))) , (s (73usize) , R (p (18usize))) , (s (101usize) , R (p (18usize))) , (s (106usize) , R (p (18usize)))] }
        }),
        (i(64usize), {
            g! { [(s (1usize) , R (p (19usize))) , (s (65usize) , R (p (19usize))) , (s (74usize) , R (p (19usize))) , (s (66usize) , R (p (19usize))) , (s (67usize) , R (p (19usize))) , (s (68usize) , R (p (19usize))) , (s (69usize) , R (p (19usize))) , (s (71usize) , R (p (19usize))) , (s (108usize) , R (p (19usize))) , (s (72usize) , R (p (19usize))) , (s (73usize) , R (p (19usize))) , (s (101usize) , R (p (19usize))) , (s (106usize) , R (p (19usize))) , (s (102usize) , S (i (66usize)))] }
        }),
        (i(65usize), {
            g! { [(s (105usize) , S (i (109usize))) , (s (101usize) , S (i (28usize)))] }
        }),
        (i(66usize), {
            g! { [(s (105usize) , R (p (92usize))) , (s (101usize) , R (p (92usize)))] }
        }),
        (i(67usize), {
            g! { [(s (105usize) , S (i (109usize))) , (s (93usize) , S (i (86usize))) , (s (94usize) , S (i (87usize))) , (s (95usize) , S (i (88usize))) , (s (96usize) , S (i (89usize))) , (s (97usize) , S (i (90usize))) , (s (98usize) , S (i (91usize))) , (s (88usize) , S (i (81usize))) , (s (89usize) , S (i (82usize))) , (s (90usize) , S (i (83usize))) , (s (91usize) , S (i (84usize))) , (s (92usize) , S (i (85usize))) , (s (99usize) , S (i (92usize))) , (s (100usize) , S (i (93usize)))] }
        }),
        (i(68usize), {
            g! { [(s (109usize) , S (i (53usize))) , (s (107usize) , S (i (52usize))) , (s (108usize) , S (i (29usize))) , (s (110usize) , S (i (54usize))) , (s (111usize) , S (i (55usize))) , (s (70usize) , S (i (49usize))) , (s (87usize) , S (i (50usize))) , (s (103usize) , S (i (51usize)))] }
        }),
        (i(69usize), {
            g! { [(s (109usize) , S (i (53usize))) , (s (107usize) , S (i (52usize))) , (s (108usize) , S (i (29usize))) , (s (110usize) , S (i (54usize))) , (s (111usize) , S (i (55usize))) , (s (70usize) , S (i (49usize))) , (s (87usize) , S (i (50usize))) , (s (103usize) , S (i (51usize)))] }
        }),
        (i(70usize), {
            g! { [(s (109usize) , S (i (53usize))) , (s (107usize) , S (i (52usize))) , (s (108usize) , S (i (29usize))) , (s (110usize) , S (i (54usize))) , (s (111usize) , S (i (55usize))) , (s (70usize) , S (i (49usize))) , (s (87usize) , S (i (50usize))) , (s (103usize) , S (i (51usize)))] }
        }),
        (i(71usize), {
            g! { [(s (109usize) , S (i (53usize))) , (s (107usize) , S (i (52usize))) , (s (108usize) , S (i (29usize))) , (s (110usize) , S (i (54usize))) , (s (111usize) , S (i (55usize))) , (s (70usize) , S (i (49usize))) , (s (87usize) , S (i (50usize))) , (s (103usize) , S (i (51usize)))] }
        }),
        (i(72usize), {
            g! { [(s (109usize) , S (i (53usize))) , (s (107usize) , S (i (52usize))) , (s (108usize) , S (i (29usize))) , (s (110usize) , S (i (54usize))) , (s (111usize) , S (i (55usize))) , (s (70usize) , S (i (49usize))) , (s (87usize) , S (i (50usize))) , (s (103usize) , S (i (51usize)))] }
        }),
        (i(73usize), {
            g! { [(s (109usize) , S (i (53usize))) , (s (107usize) , S (i (52usize))) , (s (108usize) , S (i (29usize))) , (s (110usize) , S (i (54usize))) , (s (111usize) , S (i (55usize))) , (s (70usize) , S (i (49usize))) , (s (87usize) , S (i (50usize))) , (s (103usize) , S (i (51usize)))] }
        }),
        (i(74usize), {
            g! { [(s (109usize) , S (i (53usize))) , (s (107usize) , S (i (52usize))) , (s (108usize) , S (i (29usize))) , (s (110usize) , S (i (54usize))) , (s (111usize) , S (i (55usize))) , (s (70usize) , S (i (49usize))) , (s (87usize) , S (i (50usize))) , (s (103usize) , S (i (51usize)))] }
        }),
        (i(75usize), {
            g! { [(s (109usize) , S (i (53usize))) , (s (107usize) , S (i (52usize))) , (s (108usize) , S (i (29usize))) , (s (110usize) , S (i (54usize))) , (s (111usize) , S (i (55usize))) , (s (70usize) , S (i (49usize))) , (s (87usize) , S (i (50usize))) , (s (103usize) , S (i (51usize)))] }
        }),
        (i(76usize), {
            g! { [(s (109usize) , S (i (53usize))) , (s (107usize) , S (i (52usize))) , (s (108usize) , S (i (29usize))) , (s (110usize) , S (i (54usize))) , (s (111usize) , S (i (55usize))) , (s (70usize) , S (i (49usize))) , (s (87usize) , S (i (50usize))) , (s (103usize) , S (i (51usize)))] }
        }),
        (i(77usize), {
            g! { [(s (109usize) , S (i (53usize))) , (s (107usize) , S (i (52usize))) , (s (108usize) , S (i (29usize))) , (s (110usize) , S (i (54usize))) , (s (111usize) , S (i (55usize))) , (s (70usize) , S (i (49usize))) , (s (87usize) , S (i (50usize))) , (s (103usize) , S (i (51usize)))] }
        }),
        (i(78usize), {
            g! { [(s (109usize) , S (i (53usize))) , (s (107usize) , S (i (52usize))) , (s (108usize) , S (i (29usize))) , (s (110usize) , S (i (54usize))) , (s (111usize) , S (i (55usize))) , (s (70usize) , S (i (49usize))) , (s (87usize) , S (i (50usize))) , (s (103usize) , S (i (51usize)))] }
        }),
        (i(79usize), {
            g! { [(s (109usize) , S (i (53usize))) , (s (107usize) , S (i (52usize))) , (s (108usize) , S (i (29usize))) , (s (110usize) , S (i (54usize))) , (s (111usize) , S (i (55usize))) , (s (70usize) , S (i (49usize))) , (s (87usize) , S (i (50usize))) , (s (103usize) , S (i (51usize)))] }
        }),
        (i(80usize), {
            g! { [(s (109usize) , S (i (53usize))) , (s (107usize) , S (i (52usize))) , (s (108usize) , S (i (29usize))) , (s (110usize) , S (i (54usize))) , (s (111usize) , S (i (55usize))) , (s (70usize) , S (i (49usize))) , (s (87usize) , S (i (50usize))) , (s (103usize) , S (i (51usize)))] }
        }),
        (i(81usize), {
            g! { [(s (108usize) , R (p (78usize))) , (s (70usize) , R (p (78usize))) , (s (109usize) , R (p (78usize))) , (s (107usize) , R (p (78usize))) , (s (110usize) , R (p (78usize))) , (s (111usize) , R (p (78usize))) , (s (87usize) , R (p (78usize))) , (s (103usize) , R (p (78usize)))] }
        }),
        (i(82usize), {
            g! { [(s (108usize) , R (p (79usize))) , (s (70usize) , R (p (79usize))) , (s (109usize) , R (p (79usize))) , (s (107usize) , R (p (79usize))) , (s (110usize) , R (p (79usize))) , (s (111usize) , R (p (79usize))) , (s (87usize) , R (p (79usize))) , (s (103usize) , R (p (79usize)))] }
        }),
        (i(83usize), {
            g! { [(s (108usize) , R (p (80usize))) , (s (70usize) , R (p (80usize))) , (s (109usize) , R (p (80usize))) , (s (107usize) , R (p (80usize))) , (s (110usize) , R (p (80usize))) , (s (111usize) , R (p (80usize))) , (s (87usize) , R (p (80usize))) , (s (103usize) , R (p (80usize)))] }
        }),
        (i(84usize), {
            g! { [(s (108usize) , R (p (81usize))) , (s (70usize) , R (p (81usize))) , (s (109usize) , R (p (81usize))) , (s (107usize) , R (p (81usize))) , (s (110usize) , R (p (81usize))) , (s (111usize) , R (p (81usize))) , (s (87usize) , R (p (81usize))) , (s (103usize) , R (p (81usize)))] }
        }),
        (i(85usize), {
            g! { [(s (108usize) , R (p (82usize))) , (s (70usize) , R (p (82usize))) , (s (109usize) , R (p (82usize))) , (s (107usize) , R (p (82usize))) , (s (110usize) , R (p (82usize))) , (s (111usize) , R (p (82usize))) , (s (87usize) , R (p (82usize))) , (s (103usize) , R (p (82usize)))] }
        }),
        (i(86usize), {
            g! { [(s (108usize) , R (p (83usize))) , (s (70usize) , R (p (83usize))) , (s (109usize) , R (p (83usize))) , (s (107usize) , R (p (83usize))) , (s (110usize) , R (p (83usize))) , (s (111usize) , R (p (83usize))) , (s (87usize) , R (p (83usize))) , (s (103usize) , R (p (83usize)))] }
        }),
        (i(87usize), {
            g! { [(s (108usize) , R (p (84usize))) , (s (70usize) , R (p (84usize))) , (s (109usize) , R (p (84usize))) , (s (107usize) , R (p (84usize))) , (s (110usize) , R (p (84usize))) , (s (111usize) , R (p (84usize))) , (s (87usize) , R (p (84usize))) , (s (103usize) , R (p (84usize)))] }
        }),
        (i(88usize), {
            g! { [(s (108usize) , R (p (85usize))) , (s (70usize) , R (p (85usize))) , (s (109usize) , R (p (85usize))) , (s (107usize) , R (p (85usize))) , (s (110usize) , R (p (85usize))) , (s (111usize) , R (p (85usize))) , (s (87usize) , R (p (85usize))) , (s (103usize) , R (p (85usize)))] }
        }),
        (i(89usize), {
            g! { [(s (108usize) , R (p (86usize))) , (s (70usize) , R (p (86usize))) , (s (109usize) , R (p (86usize))) , (s (107usize) , R (p (86usize))) , (s (110usize) , R (p (86usize))) , (s (111usize) , R (p (86usize))) , (s (87usize) , R (p (86usize))) , (s (103usize) , R (p (86usize)))] }
        }),
        (i(90usize), {
            g! { [(s (108usize) , R (p (87usize))) , (s (70usize) , R (p (87usize))) , (s (109usize) , R (p (87usize))) , (s (107usize) , R (p (87usize))) , (s (110usize) , R (p (87usize))) , (s (111usize) , R (p (87usize))) , (s (87usize) , R (p (87usize))) , (s (103usize) , R (p (87usize)))] }
        }),
        (i(91usize), {
            g! { [(s (108usize) , R (p (88usize))) , (s (70usize) , R (p (88usize))) , (s (109usize) , R (p (88usize))) , (s (107usize) , R (p (88usize))) , (s (110usize) , R (p (88usize))) , (s (111usize) , R (p (88usize))) , (s (87usize) , R (p (88usize))) , (s (103usize) , R (p (88usize)))] }
        }),
        (i(92usize), {
            g! { [(s (108usize) , R (p (89usize))) , (s (70usize) , R (p (89usize))) , (s (109usize) , R (p (89usize))) , (s (107usize) , R (p (89usize))) , (s (110usize) , R (p (89usize))) , (s (111usize) , R (p (89usize))) , (s (87usize) , R (p (89usize))) , (s (103usize) , R (p (89usize)))] }
        }),
        (i(93usize), {
            g! { [(s (108usize) , R (p (90usize))) , (s (70usize) , R (p (90usize))) , (s (109usize) , R (p (90usize))) , (s (107usize) , R (p (90usize))) , (s (110usize) , R (p (90usize))) , (s (111usize) , R (p (90usize))) , (s (87usize) , R (p (90usize))) , (s (103usize) , R (p (90usize)))] }
        }),
        (i(94usize), {
            g! { [(s (109usize) , S (i (53usize))) , (s (107usize) , S (i (52usize))) , (s (108usize) , S (i (29usize))) , (s (110usize) , S (i (54usize))) , (s (111usize) , S (i (55usize))) , (s (70usize) , S (i (49usize))) , (s (87usize) , S (i (50usize))) , (s (103usize) , S (i (51usize)))] }
        }),
        (i(95usize), {
            g! { [(s (77usize) , S (i (126usize)))] }
        }),
        (i(96usize), {
            g! { [(s (1usize) , R (p (27usize))) , (s (65usize) , R (p (27usize))) , (s (74usize) , R (p (27usize))) , (s (66usize) , R (p (27usize))) , (s (67usize) , R (p (27usize))) , (s (68usize) , R (p (27usize))) , (s (69usize) , R (p (27usize))) , (s (71usize) , R (p (27usize))) , (s (108usize) , R (p (27usize))) , (s (72usize) , R (p (27usize))) , (s (73usize) , R (p (27usize))) , (s (101usize) , R (p (27usize))) , (s (93usize) , R (p (27usize))) , (s (94usize) , R (p (27usize))) , (s (95usize) , R (p (27usize))) , (s (96usize) , R (p (27usize))) , (s (97usize) , R (p (27usize))) , (s (98usize) , R (p (27usize))) , (s (88usize) , R (p (27usize))) , (s (89usize) , R (p (27usize))) , (s (90usize) , R (p (27usize))) , (s (91usize) , R (p (27usize))) , (s (92usize) , R (p (27usize))) , (s (99usize) , R (p (27usize))) , (s (100usize) , R (p (27usize))) , (s (84usize) , R (p (27usize))) , (s (105usize) , R (p (27usize))) , (s (104usize) , R (p (27usize))) , (s (81usize) , R (p (27usize))) , (s (106usize) , R (p (27usize)))] }
        }),
        (i(97usize), {
            g! { [(s (108usize) , R (p (67usize))) , (s (70usize) , R (p (67usize))) , (s (109usize) , R (p (67usize))) , (s (107usize) , R (p (67usize))) , (s (110usize) , R (p (67usize))) , (s (111usize) , R (p (67usize))) , (s (87usize) , R (p (67usize))) , (s (103usize) , R (p (67usize)))] }
        }),
        (i(98usize), {
            g! { [(s (1usize) , R (p (70usize))) , (s (65usize) , R (p (70usize))) , (s (74usize) , R (p (70usize))) , (s (66usize) , R (p (70usize))) , (s (67usize) , R (p (70usize))) , (s (68usize) , R (p (70usize))) , (s (69usize) , R (p (70usize))) , (s (71usize) , R (p (70usize))) , (s (108usize) , R (p (70usize))) , (s (72usize) , R (p (70usize))) , (s (73usize) , R (p (70usize))) , (s (101usize) , R (p (70usize))) , (s (93usize) , R (p (70usize))) , (s (94usize) , R (p (70usize))) , (s (95usize) , R (p (70usize))) , (s (96usize) , R (p (70usize))) , (s (97usize) , R (p (70usize))) , (s (98usize) , R (p (70usize))) , (s (88usize) , R (p (70usize))) , (s (89usize) , R (p (70usize))) , (s (90usize) , R (p (70usize))) , (s (91usize) , R (p (70usize))) , (s (92usize) , R (p (70usize))) , (s (99usize) , R (p (70usize))) , (s (100usize) , R (p (70usize))) , (s (84usize) , R (p (70usize))) , (s (105usize) , R (p (70usize))) , (s (104usize) , R (p (70usize))) , (s (81usize) , R (p (70usize))) , (s (106usize) , R (p (70usize)))] }
        }),
        (i(99usize), {
            g! { [(s (77usize) , R (p (75usize)))] }
        }),
        (i(100usize), {
            g! { [(s (1usize) , R (p (39usize))) , (s (65usize) , R (p (39usize))) , (s (74usize) , R (p (39usize))) , (s (66usize) , R (p (39usize))) , (s (67usize) , R (p (39usize))) , (s (68usize) , R (p (39usize))) , (s (69usize) , R (p (39usize))) , (s (71usize) , R (p (39usize))) , (s (108usize) , R (p (39usize))) , (s (72usize) , R (p (39usize))) , (s (73usize) , R (p (39usize))) , (s (101usize) , R (p (39usize))) , (s (93usize) , R (p (39usize))) , (s (94usize) , R (p (39usize))) , (s (95usize) , R (p (39usize))) , (s (96usize) , R (p (39usize))) , (s (97usize) , R (p (39usize))) , (s (98usize) , R (p (39usize))) , (s (88usize) , R (p (39usize))) , (s (89usize) , R (p (39usize))) , (s (90usize) , R (p (39usize))) , (s (91usize) , R (p (39usize))) , (s (92usize) , R (p (39usize))) , (s (99usize) , R (p (39usize))) , (s (100usize) , R (p (39usize))) , (s (84usize) , R (p (39usize))) , (s (105usize) , R (p (39usize))) , (s (104usize) , R (p (39usize))) , (s (81usize) , R (p (39usize))) , (s (106usize) , R (p (39usize)))] }
        }),
        (i(101usize), {
            g! { [(s (104usize) , S (i (128usize))) , (s (93usize) , S (i (86usize))) , (s (94usize) , S (i (87usize))) , (s (95usize) , S (i (88usize))) , (s (96usize) , S (i (89usize))) , (s (97usize) , S (i (90usize))) , (s (98usize) , S (i (91usize))) , (s (88usize) , S (i (81usize))) , (s (89usize) , S (i (82usize))) , (s (90usize) , S (i (83usize))) , (s (91usize) , S (i (84usize))) , (s (92usize) , S (i (85usize))) , (s (99usize) , S (i (92usize))) , (s (100usize) , S (i (93usize)))] }
        }),
        (i(102usize), {
            g! { [(s (1usize) , R (p (12usize))) , (s (65usize) , R (p (12usize))) , (s (74usize) , R (p (12usize))) , (s (66usize) , R (p (12usize))) , (s (67usize) , R (p (12usize))) , (s (68usize) , R (p (12usize))) , (s (69usize) , R (p (12usize))) , (s (71usize) , R (p (12usize))) , (s (108usize) , R (p (12usize))) , (s (72usize) , R (p (12usize))) , (s (73usize) , R (p (12usize))) , (s (101usize) , R (p (12usize))) , (s (106usize) , R (p (12usize))) , (s (93usize) , S (i (86usize))) , (s (94usize) , S (i (87usize))) , (s (95usize) , S (i (88usize))) , (s (96usize) , S (i (89usize))) , (s (97usize) , S (i (90usize))) , (s (98usize) , S (i (91usize))) , (s (88usize) , S (i (81usize))) , (s (89usize) , S (i (82usize))) , (s (90usize) , S (i (83usize))) , (s (91usize) , S (i (84usize))) , (s (92usize) , S (i (85usize))) , (s (99usize) , S (i (92usize))) , (s (100usize) , S (i (93usize)))] }
        }),
        (i(103usize), {
            g! { [(s (76usize) , S (i (130usize)))] }
        }),
        (i(104usize), {
            g! { [(s (76usize) , R (p (74usize)))] }
        }),
        (i(105usize), {
            g! { [(s (1usize) , R (p (20usize))) , (s (65usize) , R (p (20usize))) , (s (74usize) , R (p (20usize))) , (s (66usize) , R (p (20usize))) , (s (67usize) , R (p (20usize))) , (s (68usize) , R (p (20usize))) , (s (69usize) , R (p (20usize))) , (s (71usize) , R (p (20usize))) , (s (108usize) , R (p (20usize))) , (s (72usize) , R (p (20usize))) , (s (73usize) , R (p (20usize))) , (s (101usize) , R (p (20usize))) , (s (106usize) , R (p (20usize)))] }
        }),
        (i(106usize), {
            g! { [(s (1usize) , R (p (21usize))) , (s (65usize) , R (p (21usize))) , (s (74usize) , R (p (21usize))) , (s (66usize) , R (p (21usize))) , (s (67usize) , R (p (21usize))) , (s (68usize) , R (p (21usize))) , (s (69usize) , R (p (21usize))) , (s (71usize) , R (p (21usize))) , (s (108usize) , R (p (21usize))) , (s (72usize) , R (p (21usize))) , (s (73usize) , R (p (21usize))) , (s (101usize) , R (p (21usize))) , (s (106usize) , R (p (21usize)))] }
        }),
        (i(107usize), {
            g! { [(s (109usize) , S (i (53usize))) , (s (107usize) , S (i (52usize))) , (s (108usize) , S (i (29usize))) , (s (110usize) , S (i (54usize))) , (s (111usize) , S (i (55usize))) , (s (70usize) , S (i (49usize))) , (s (87usize) , S (i (50usize))) , (s (103usize) , S (i (51usize)))] }
        }),
        (i(108usize), {
            g! { [(s (65usize) , S (i (19usize))) , (s (74usize) , S (i (27usize))) , (s (66usize) , S (i (20usize))) , (s (67usize) , S (i (21usize))) , (s (68usize) , S (i (22usize))) , (s (69usize) , S (i (23usize))) , (s (71usize) , S (i (24usize))) , (s (108usize) , S (i (29usize))) , (s (72usize) , S (i (25usize))) , (s (73usize) , S (i (26usize))) , (s (101usize) , S (i (28usize)))] }
        }),
        (i(109usize), {
            g! { [(s (65usize) , R (p (95usize))) , (s (74usize) , R (p (95usize))) , (s (66usize) , R (p (95usize))) , (s (67usize) , R (p (95usize))) , (s (68usize) , R (p (95usize))) , (s (69usize) , R (p (95usize))) , (s (71usize) , R (p (95usize))) , (s (108usize) , R (p (95usize))) , (s (72usize) , R (p (95usize))) , (s (73usize) , R (p (95usize))) , (s (101usize) , R (p (95usize)))] }
        }),
        (i(110usize), {
            g! { [(s (65usize) , S (i (19usize))) , (s (74usize) , S (i (27usize))) , (s (66usize) , S (i (20usize))) , (s (67usize) , S (i (21usize))) , (s (68usize) , S (i (22usize))) , (s (69usize) , S (i (23usize))) , (s (71usize) , S (i (24usize))) , (s (108usize) , S (i (29usize))) , (s (72usize) , S (i (25usize))) , (s (73usize) , S (i (26usize))) , (s (101usize) , S (i (28usize)))] }
        }),
        (i(111usize), {
            g! { [(s (1usize) , R (p (44usize))) , (s (65usize) , R (p (44usize))) , (s (74usize) , R (p (44usize))) , (s (66usize) , R (p (44usize))) , (s (67usize) , R (p (44usize))) , (s (68usize) , R (p (44usize))) , (s (69usize) , R (p (44usize))) , (s (71usize) , R (p (44usize))) , (s (108usize) , R (p (44usize))) , (s (72usize) , R (p (44usize))) , (s (73usize) , R (p (44usize))) , (s (101usize) , R (p (44usize))) , (s (93usize) , S (i (86usize))) , (s (94usize) , S (i (87usize))) , (s (95usize) , S (i (88usize))) , (s (96usize) , S (i (89usize))) , (s (97usize) , S (i (90usize))) , (s (98usize) , S (i (91usize))) , (s (88usize) , S (i (81usize))) , (s (89usize) , S (i (82usize))) , (s (90usize) , S (i (83usize))) , (s (91usize) , S (i (84usize))) , (s (92usize) , S (i (85usize))) , (s (99usize) , S (i (92usize))) , (s (100usize) , S (i (93usize))) , (s (84usize) , R (p (44usize))) , (s (105usize) , R (p (44usize))) , (s (104usize) , R (p (44usize))) , (s (81usize) , R (p (44usize))) , (s (106usize) , R (p (44usize)))] }
        }),
        (i(112usize), {
            g! { [(s (1usize) , R (p (45usize))) , (s (65usize) , R (p (45usize))) , (s (74usize) , R (p (45usize))) , (s (66usize) , R (p (45usize))) , (s (67usize) , R (p (45usize))) , (s (68usize) , R (p (45usize))) , (s (69usize) , R (p (45usize))) , (s (71usize) , R (p (45usize))) , (s (108usize) , R (p (45usize))) , (s (72usize) , R (p (45usize))) , (s (73usize) , R (p (45usize))) , (s (101usize) , R (p (45usize))) , (s (93usize) , S (i (86usize))) , (s (94usize) , S (i (87usize))) , (s (95usize) , S (i (88usize))) , (s (96usize) , S (i (89usize))) , (s (97usize) , S (i (90usize))) , (s (98usize) , S (i (91usize))) , (s (88usize) , S (i (81usize))) , (s (89usize) , S (i (82usize))) , (s (90usize) , S (i (83usize))) , (s (91usize) , S (i (84usize))) , (s (92usize) , S (i (85usize))) , (s (99usize) , S (i (92usize))) , (s (100usize) , S (i (93usize))) , (s (84usize) , R (p (45usize))) , (s (105usize) , R (p (45usize))) , (s (104usize) , R (p (45usize))) , (s (81usize) , R (p (45usize))) , (s (106usize) , R (p (45usize)))] }
        }),
        (i(113usize), {
            g! { [(s (1usize) , R (p (46usize))) , (s (65usize) , R (p (46usize))) , (s (74usize) , R (p (46usize))) , (s (66usize) , R (p (46usize))) , (s (67usize) , R (p (46usize))) , (s (68usize) , R (p (46usize))) , (s (69usize) , R (p (46usize))) , (s (71usize) , R (p (46usize))) , (s (108usize) , R (p (46usize))) , (s (72usize) , R (p (46usize))) , (s (73usize) , R (p (46usize))) , (s (101usize) , R (p (46usize))) , (s (93usize) , S (i (86usize))) , (s (94usize) , S (i (87usize))) , (s (95usize) , S (i (88usize))) , (s (96usize) , S (i (89usize))) , (s (97usize) , S (i (90usize))) , (s (98usize) , S (i (91usize))) , (s (88usize) , S (i (81usize))) , (s (89usize) , S (i (82usize))) , (s (90usize) , S (i (83usize))) , (s (91usize) , S (i (84usize))) , (s (92usize) , S (i (85usize))) , (s (99usize) , S (i (92usize))) , (s (100usize) , S (i (93usize))) , (s (84usize) , R (p (46usize))) , (s (105usize) , R (p (46usize))) , (s (104usize) , R (p (46usize))) , (s (81usize) , R (p (46usize))) , (s (106usize) , R (p (46usize)))] }
        }),
        (i(114usize), {
            g! { [(s (1usize) , R (p (47usize))) , (s (65usize) , R (p (47usize))) , (s (74usize) , R (p (47usize))) , (s (66usize) , R (p (47usize))) , (s (67usize) , R (p (47usize))) , (s (68usize) , R (p (47usize))) , (s (69usize) , R (p (47usize))) , (s (71usize) , R (p (47usize))) , (s (108usize) , R (p (47usize))) , (s (72usize) , R (p (47usize))) , (s (73usize) , R (p (47usize))) , (s (101usize) , R (p (47usize))) , (s (93usize) , S (i (86usize))) , (s (94usize) , S (i (87usize))) , (s (95usize) , S (i (88usize))) , (s (96usize) , S (i (89usize))) , (s (97usize) , S (i (90usize))) , (s (98usize) , S (i (91usize))) , (s (88usize) , S (i (81usize))) , (s (89usize) , S (i (82usize))) , (s (90usize) , S (i (83usize))) , (s (91usize) , S (i (84usize))) , (s (92usize) , S (i (85usize))) , (s (99usize) , S (i (92usize))) , (s (100usize) , S (i (93usize))) , (s (84usize) , R (p (47usize))) , (s (105usize) , R (p (47usize))) , (s (104usize) , R (p (47usize))) , (s (81usize) , R (p (47usize))) , (s (106usize) , R (p (47usize)))] }
        }),
        (i(115usize), {
            g! { [(s (1usize) , R (p (48usize))) , (s (65usize) , R (p (48usize))) , (s (74usize) , R (p (48usize))) , (s (66usize) , R (p (48usize))) , (s (67usize) , R (p (48usize))) , (s (68usize) , R (p (48usize))) , (s (69usize) , R (p (48usize))) , (s (71usize) , R (p (48usize))) , (s (108usize) , R (p (48usize))) , (s (72usize) , R (p (48usize))) , (s (73usize) , R (p (48usize))) , (s (101usize) , R (p (48usize))) , (s (93usize) , S (i (86usize))) , (s (94usize) , S (i (87usize))) , (s (95usize) , S (i (88usize))) , (s (96usize) , S (i (89usize))) , (s (97usize) , S (i (90usize))) , (s (98usize) , S (i (91usize))) , (s (88usize) , S (i (81usize))) , (s (89usize) , S (i (82usize))) , (s (90usize) , S (i (83usize))) , (s (91usize) , S (i (84usize))) , (s (92usize) , S (i (85usize))) , (s (99usize) , S (i (92usize))) , (s (100usize) , S (i (93usize))) , (s (84usize) , R (p (48usize))) , (s (105usize) , R (p (48usize))) , (s (104usize) , R (p (48usize))) , (s (81usize) , R (p (48usize))) , (s (106usize) , R (p (48usize)))] }
        }),
        (i(116usize), {
            g! { [(s (1usize) , R (p (49usize))) , (s (65usize) , R (p (49usize))) , (s (74usize) , R (p (49usize))) , (s (66usize) , R (p (49usize))) , (s (67usize) , R (p (49usize))) , (s (68usize) , R (p (49usize))) , (s (69usize) , R (p (49usize))) , (s (71usize) , R (p (49usize))) , (s (108usize) , R (p (49usize))) , (s (72usize) , R (p (49usize))) , (s (73usize) , R (p (49usize))) , (s (101usize) , R (p (49usize))) , (s (93usize) , S (i (86usize))) , (s (94usize) , S (i (87usize))) , (s (95usize) , S (i (88usize))) , (s (96usize) , S (i (89usize))) , (s (97usize) , S (i (90usize))) , (s (98usize) , S (i (91usize))) , (s (88usize) , S (i (81usize))) , (s (89usize) , S (i (82usize))) , (s (90usize) , S (i (83usize))) , (s (91usize) , S (i (84usize))) , (s (92usize) , S (i (85usize))) , (s (99usize) , S (i (92usize))) , (s (100usize) , S (i (93usize))) , (s (84usize) , R (p (49usize))) , (s (105usize) , R (p (49usize))) , (s (104usize) , R (p (49usize))) , (s (81usize) , R (p (49usize))) , (s (106usize) , R (p (49usize)))] }
        }),
        (i(117usize), {
            g! { [(s (1usize) , R (p (50usize))) , (s (65usize) , R (p (50usize))) , (s (74usize) , R (p (50usize))) , (s (66usize) , R (p (50usize))) , (s (67usize) , R (p (50usize))) , (s (68usize) , R (p (50usize))) , (s (69usize) , R (p (50usize))) , (s (71usize) , R (p (50usize))) , (s (108usize) , R (p (50usize))) , (s (72usize) , R (p (50usize))) , (s (73usize) , R (p (50usize))) , (s (101usize) , R (p (50usize))) , (s (93usize) , S (i (86usize))) , (s (94usize) , S (i (87usize))) , (s (95usize) , S (i (88usize))) , (s (96usize) , S (i (89usize))) , (s (97usize) , S (i (90usize))) , (s (98usize) , S (i (91usize))) , (s (88usize) , S (i (81usize))) , (s (89usize) , S (i (82usize))) , (s (90usize) , S (i (83usize))) , (s (91usize) , S (i (84usize))) , (s (92usize) , S (i (85usize))) , (s (99usize) , S (i (92usize))) , (s (100usize) , S (i (93usize))) , (s (84usize) , R (p (50usize))) , (s (105usize) , R (p (50usize))) , (s (104usize) , R (p (50usize))) , (s (81usize) , R (p (50usize))) , (s (106usize) , R (p (50usize)))] }
        }),
        (i(118usize), {
            g! { [(s (1usize) , R (p (51usize))) , (s (65usize) , R (p (51usize))) , (s (74usize) , R (p (51usize))) , (s (66usize) , R (p (51usize))) , (s (67usize) , R (p (51usize))) , (s (68usize) , R (p (51usize))) , (s (69usize) , R (p (51usize))) , (s (71usize) , R (p (51usize))) , (s (108usize) , R (p (51usize))) , (s (72usize) , R (p (51usize))) , (s (73usize) , R (p (51usize))) , (s (101usize) , R (p (51usize))) , (s (93usize) , S (i (86usize))) , (s (94usize) , S (i (87usize))) , (s (95usize) , S (i (88usize))) , (s (96usize) , S (i (89usize))) , (s (97usize) , S (i (90usize))) , (s (98usize) , S (i (91usize))) , (s (88usize) , S (i (81usize))) , (s (89usize) , S (i (82usize))) , (s (90usize) , S (i (83usize))) , (s (91usize) , S (i (84usize))) , (s (92usize) , S (i (85usize))) , (s (99usize) , S (i (92usize))) , (s (100usize) , S (i (93usize))) , (s (84usize) , R (p (51usize))) , (s (105usize) , R (p (51usize))) , (s (104usize) , R (p (51usize))) , (s (81usize) , R (p (51usize))) , (s (106usize) , R (p (51usize)))] }
        }),
        (i(119usize), {
            g! { [(s (1usize) , R (p (52usize))) , (s (65usize) , R (p (52usize))) , (s (74usize) , R (p (52usize))) , (s (66usize) , R (p (52usize))) , (s (67usize) , R (p (52usize))) , (s (68usize) , R (p (52usize))) , (s (69usize) , R (p (52usize))) , (s (71usize) , R (p (52usize))) , (s (108usize) , R (p (52usize))) , (s (72usize) , R (p (52usize))) , (s (73usize) , R (p (52usize))) , (s (101usize) , R (p (52usize))) , (s (93usize) , S (i (86usize))) , (s (94usize) , S (i (87usize))) , (s (95usize) , S (i (88usize))) , (s (96usize) , S (i (89usize))) , (s (97usize) , S (i (90usize))) , (s (98usize) , S (i (91usize))) , (s (88usize) , S (i (81usize))) , (s (89usize) , S (i (82usize))) , (s (90usize) , S (i (83usize))) , (s (91usize) , S (i (84usize))) , (s (92usize) , S (i (85usize))) , (s (99usize) , S (i (92usize))) , (s (100usize) , S (i (93usize))) , (s (84usize) , R (p (52usize))) , (s (105usize) , R (p (52usize))) , (s (104usize) , R (p (52usize))) , (s (81usize) , R (p (52usize))) , (s (106usize) , R (p (52usize)))] }
        }),
        (i(120usize), {
            g! { [(s (1usize) , R (p (53usize))) , (s (65usize) , R (p (53usize))) , (s (74usize) , R (p (53usize))) , (s (66usize) , R (p (53usize))) , (s (67usize) , R (p (53usize))) , (s (68usize) , R (p (53usize))) , (s (69usize) , R (p (53usize))) , (s (71usize) , R (p (53usize))) , (s (108usize) , R (p (53usize))) , (s (72usize) , R (p (53usize))) , (s (73usize) , R (p (53usize))) , (s (101usize) , R (p (53usize))) , (s (93usize) , S (i (86usize))) , (s (94usize) , S (i (87usize))) , (s (95usize) , S (i (88usize))) , (s (96usize) , S (i (89usize))) , (s (97usize) , S (i (90usize))) , (s (98usize) , S (i (91usize))) , (s (88usize) , S (i (81usize))) , (s (89usize) , S (i (82usize))) , (s (90usize) , S (i (83usize))) , (s (91usize) , S (i (84usize))) , (s (92usize) , S (i (85usize))) , (s (99usize) , S (i (92usize))) , (s (100usize) , S (i (93usize))) , (s (84usize) , R (p (53usize))) , (s (105usize) , R (p (53usize))) , (s (104usize) , R (p (53usize))) , (s (81usize) , R (p (53usize))) , (s (106usize) , R (p (53usize)))] }
        }),
        (i(121usize), {
            g! { [(s (1usize) , R (p (54usize))) , (s (65usize) , R (p (54usize))) , (s (74usize) , R (p (54usize))) , (s (66usize) , R (p (54usize))) , (s (67usize) , R (p (54usize))) , (s (68usize) , R (p (54usize))) , (s (69usize) , R (p (54usize))) , (s (71usize) , R (p (54usize))) , (s (108usize) , R (p (54usize))) , (s (72usize) , R (p (54usize))) , (s (73usize) , R (p (54usize))) , (s (101usize) , R (p (54usize))) , (s (93usize) , S (i (86usize))) , (s (94usize) , S (i (87usize))) , (s (95usize) , S (i (88usize))) , (s (96usize) , S (i (89usize))) , (s (97usize) , S (i (90usize))) , (s (98usize) , S (i (91usize))) , (s (88usize) , S (i (81usize))) , (s (89usize) , S (i (82usize))) , (s (90usize) , S (i (83usize))) , (s (91usize) , S (i (84usize))) , (s (92usize) , S (i (85usize))) , (s (99usize) , S (i (92usize))) , (s (100usize) , S (i (93usize))) , (s (84usize) , R (p (54usize))) , (s (105usize) , R (p (54usize))) , (s (104usize) , R (p (54usize))) , (s (81usize) , R (p (54usize))) , (s (106usize) , R (p (54usize)))] }
        }),
        (i(122usize), {
            g! { [(s (1usize) , R (p (55usize))) , (s (65usize) , R (p (55usize))) , (s (74usize) , R (p (55usize))) , (s (66usize) , R (p (55usize))) , (s (67usize) , R (p (55usize))) , (s (68usize) , R (p (55usize))) , (s (69usize) , R (p (55usize))) , (s (71usize) , R (p (55usize))) , (s (108usize) , R (p (55usize))) , (s (72usize) , R (p (55usize))) , (s (73usize) , R (p (55usize))) , (s (101usize) , R (p (55usize))) , (s (93usize) , S (i (86usize))) , (s (94usize) , S (i (87usize))) , (s (95usize) , S (i (88usize))) , (s (96usize) , S (i (89usize))) , (s (97usize) , S (i (90usize))) , (s (98usize) , S (i (91usize))) , (s (88usize) , S (i (81usize))) , (s (89usize) , S (i (82usize))) , (s (90usize) , S (i (83usize))) , (s (91usize) , S (i (84usize))) , (s (92usize) , S (i (85usize))) , (s (99usize) , S (i (92usize))) , (s (100usize) , S (i (93usize))) , (s (84usize) , R (p (55usize))) , (s (105usize) , R (p (55usize))) , (s (104usize) , R (p (55usize))) , (s (81usize) , R (p (55usize))) , (s (106usize) , R (p (55usize)))] }
        }),
        (i(123usize), {
            g! { [(s (1usize) , R (p (56usize))) , (s (65usize) , R (p (56usize))) , (s (74usize) , R (p (56usize))) , (s (66usize) , R (p (56usize))) , (s (67usize) , R (p (56usize))) , (s (68usize) , R (p (56usize))) , (s (69usize) , R (p (56usize))) , (s (71usize) , R (p (56usize))) , (s (108usize) , R (p (56usize))) , (s (72usize) , R (p (56usize))) , (s (73usize) , R (p (56usize))) , (s (101usize) , R (p (56usize))) , (s (93usize) , S (i (86usize))) , (s (94usize) , S (i (87usize))) , (s (95usize) , S (i (88usize))) , (s (96usize) , S (i (89usize))) , (s (97usize) , S (i (90usize))) , (s (98usize) , S (i (91usize))) , (s (88usize) , S (i (81usize))) , (s (89usize) , S (i (82usize))) , (s (90usize) , S (i (83usize))) , (s (91usize) , S (i (84usize))) , (s (92usize) , S (i (85usize))) , (s (99usize) , S (i (92usize))) , (s (100usize) , S (i (93usize))) , (s (84usize) , R (p (56usize))) , (s (105usize) , R (p (56usize))) , (s (104usize) , R (p (56usize))) , (s (81usize) , R (p (56usize))) , (s (106usize) , R (p (56usize)))] }
        }),
        (i(124usize), {
            g! { [(s (81usize) , S (i (135usize))) , (s (93usize) , S (i (86usize))) , (s (94usize) , S (i (87usize))) , (s (95usize) , S (i (88usize))) , (s (96usize) , S (i (89usize))) , (s (97usize) , S (i (90usize))) , (s (98usize) , S (i (91usize))) , (s (88usize) , S (i (81usize))) , (s (89usize) , S (i (82usize))) , (s (90usize) , S (i (83usize))) , (s (91usize) , S (i (84usize))) , (s (92usize) , S (i (85usize))) , (s (99usize) , S (i (92usize))) , (s (100usize) , S (i (93usize)))] }
        }),
        (i(125usize), {
            g! { [(s (1usize) , R (p (26usize))) , (s (65usize) , R (p (26usize))) , (s (74usize) , R (p (26usize))) , (s (66usize) , R (p (26usize))) , (s (67usize) , R (p (26usize))) , (s (68usize) , R (p (26usize))) , (s (69usize) , R (p (26usize))) , (s (71usize) , R (p (26usize))) , (s (108usize) , R (p (26usize))) , (s (72usize) , R (p (26usize))) , (s (73usize) , R (p (26usize))) , (s (101usize) , R (p (26usize))) , (s (93usize) , R (p (26usize))) , (s (94usize) , R (p (26usize))) , (s (95usize) , R (p (26usize))) , (s (96usize) , R (p (26usize))) , (s (97usize) , R (p (26usize))) , (s (98usize) , R (p (26usize))) , (s (88usize) , R (p (26usize))) , (s (89usize) , R (p (26usize))) , (s (90usize) , R (p (26usize))) , (s (91usize) , R (p (26usize))) , (s (92usize) , R (p (26usize))) , (s (99usize) , R (p (26usize))) , (s (100usize) , R (p (26usize))) , (s (84usize) , R (p (26usize))) , (s (105usize) , R (p (26usize))) , (s (104usize) , R (p (26usize))) , (s (81usize) , R (p (26usize))) , (s (106usize) , R (p (26usize)))] }
        }),
        (i(126usize), {
            g! { [(s (1usize) , R (p (69usize))) , (s (65usize) , R (p (69usize))) , (s (74usize) , R (p (69usize))) , (s (66usize) , R (p (69usize))) , (s (67usize) , R (p (69usize))) , (s (68usize) , R (p (69usize))) , (s (69usize) , R (p (69usize))) , (s (71usize) , R (p (69usize))) , (s (108usize) , R (p (69usize))) , (s (72usize) , R (p (69usize))) , (s (73usize) , R (p (69usize))) , (s (101usize) , R (p (69usize))) , (s (93usize) , R (p (69usize))) , (s (94usize) , R (p (69usize))) , (s (95usize) , R (p (69usize))) , (s (96usize) , R (p (69usize))) , (s (97usize) , R (p (69usize))) , (s (98usize) , R (p (69usize))) , (s (88usize) , R (p (69usize))) , (s (89usize) , R (p (69usize))) , (s (90usize) , R (p (69usize))) , (s (91usize) , R (p (69usize))) , (s (92usize) , R (p (69usize))) , (s (99usize) , R (p (69usize))) , (s (100usize) , R (p (69usize))) , (s (84usize) , R (p (69usize))) , (s (105usize) , R (p (69usize))) , (s (104usize) , R (p (69usize))) , (s (81usize) , R (p (69usize))) , (s (106usize) , R (p (69usize)))] }
        }),
        (i(127usize), {
            g! { [(s (1usize) , R (p (40usize))) , (s (65usize) , R (p (40usize))) , (s (74usize) , R (p (40usize))) , (s (66usize) , R (p (40usize))) , (s (67usize) , R (p (40usize))) , (s (68usize) , R (p (40usize))) , (s (69usize) , R (p (40usize))) , (s (71usize) , R (p (40usize))) , (s (108usize) , R (p (40usize))) , (s (72usize) , R (p (40usize))) , (s (73usize) , R (p (40usize))) , (s (101usize) , R (p (40usize))) , (s (93usize) , R (p (40usize))) , (s (94usize) , R (p (40usize))) , (s (95usize) , R (p (40usize))) , (s (96usize) , R (p (40usize))) , (s (97usize) , R (p (40usize))) , (s (98usize) , R (p (40usize))) , (s (88usize) , R (p (40usize))) , (s (89usize) , R (p (40usize))) , (s (90usize) , R (p (40usize))) , (s (91usize) , R (p (40usize))) , (s (92usize) , R (p (40usize))) , (s (99usize) , R (p (40usize))) , (s (100usize) , R (p (40usize))) , (s (84usize) , R (p (40usize))) , (s (105usize) , R (p (40usize))) , (s (104usize) , R (p (40usize))) , (s (81usize) , R (p (40usize))) , (s (106usize) , R (p (40usize)))] }
        }),
        (i(128usize), {
            g! { [(s (1usize) , R (p (94usize))) , (s (65usize) , R (p (94usize))) , (s (74usize) , R (p (94usize))) , (s (66usize) , R (p (94usize))) , (s (67usize) , R (p (94usize))) , (s (68usize) , R (p (94usize))) , (s (69usize) , R (p (94usize))) , (s (71usize) , R (p (94usize))) , (s (108usize) , R (p (94usize))) , (s (72usize) , R (p (94usize))) , (s (73usize) , R (p (94usize))) , (s (101usize) , R (p (94usize))) , (s (93usize) , R (p (94usize))) , (s (94usize) , R (p (94usize))) , (s (95usize) , R (p (94usize))) , (s (96usize) , R (p (94usize))) , (s (97usize) , R (p (94usize))) , (s (98usize) , R (p (94usize))) , (s (88usize) , R (p (94usize))) , (s (89usize) , R (p (94usize))) , (s (90usize) , R (p (94usize))) , (s (91usize) , R (p (94usize))) , (s (92usize) , R (p (94usize))) , (s (99usize) , R (p (94usize))) , (s (100usize) , R (p (94usize))) , (s (84usize) , R (p (94usize))) , (s (105usize) , R (p (94usize))) , (s (104usize) , R (p (94usize))) , (s (81usize) , R (p (94usize))) , (s (106usize) , R (p (94usize)))] }
        }),
        (i(129usize), {
            g! { [(s (109usize) , S (i (53usize))) , (s (107usize) , S (i (52usize))) , (s (108usize) , S (i (29usize))) , (s (110usize) , S (i (54usize))) , (s (111usize) , S (i (55usize))) , (s (70usize) , S (i (49usize))) , (s (87usize) , S (i (50usize))) , (s (103usize) , S (i (51usize)))] }
        }),
        (i(130usize), {
            g! { [(s (108usize) , R (p (68usize))) , (s (70usize) , R (p (68usize))) , (s (109usize) , R (p (68usize))) , (s (107usize) , R (p (68usize))) , (s (110usize) , R (p (68usize))) , (s (111usize) , R (p (68usize))) , (s (87usize) , R (p (68usize))) , (s (103usize) , R (p (68usize)))] }
        }),
        (i(131usize), {
            g! { [(s (105usize) , S (i (109usize))) , (s (93usize) , S (i (86usize))) , (s (94usize) , S (i (87usize))) , (s (95usize) , S (i (88usize))) , (s (96usize) , S (i (89usize))) , (s (97usize) , S (i (90usize))) , (s (98usize) , S (i (91usize))) , (s (88usize) , S (i (81usize))) , (s (89usize) , S (i (82usize))) , (s (90usize) , S (i (83usize))) , (s (91usize) , S (i (84usize))) , (s (92usize) , S (i (85usize))) , (s (99usize) , S (i (92usize))) , (s (100usize) , S (i (93usize)))] }
        }),
        (i(132usize), {
            g! { [(s (106usize) , S (i (139usize)))] }
        }),
        (i(133usize), {
            g! { [(s (106usize) , S (i (139usize)))] }
        }),
        (i(134usize), {
            g! { [(s (76usize) , S (i (130usize)))] }
        }),
        (i(135usize), {
            g! { [(s (76usize) , R (p (72usize)))] }
        }),
        (i(136usize), {
            g! { [(s (1usize) , R (p (14usize))) , (s (65usize) , R (p (14usize))) , (s (74usize) , R (p (14usize))) , (s (66usize) , R (p (14usize))) , (s (67usize) , R (p (14usize))) , (s (68usize) , R (p (14usize))) , (s (69usize) , R (p (14usize))) , (s (71usize) , R (p (14usize))) , (s (108usize) , R (p (14usize))) , (s (72usize) , R (p (14usize))) , (s (73usize) , R (p (14usize))) , (s (101usize) , R (p (14usize))) , (s (106usize) , R (p (14usize))) , (s (93usize) , S (i (86usize))) , (s (94usize) , S (i (87usize))) , (s (95usize) , S (i (88usize))) , (s (96usize) , S (i (89usize))) , (s (97usize) , S (i (90usize))) , (s (98usize) , S (i (91usize))) , (s (88usize) , S (i (81usize))) , (s (89usize) , S (i (82usize))) , (s (90usize) , S (i (83usize))) , (s (91usize) , S (i (84usize))) , (s (92usize) , S (i (85usize))) , (s (99usize) , S (i (92usize))) , (s (100usize) , S (i (93usize)))] }
        }),
        (i(137usize), {
            g! { [(s (65usize) , S (i (19usize))) , (s (74usize) , S (i (27usize))) , (s (66usize) , S (i (20usize))) , (s (67usize) , S (i (21usize))) , (s (68usize) , S (i (22usize))) , (s (69usize) , S (i (23usize))) , (s (71usize) , S (i (24usize))) , (s (108usize) , S (i (29usize))) , (s (72usize) , S (i (25usize))) , (s (73usize) , S (i (26usize))) , (s (101usize) , S (i (28usize)))] }
        }),
        (i(138usize), {
            g! { [(s (1usize) , R (p (24usize))) , (s (65usize) , R (p (24usize))) , (s (74usize) , R (p (24usize))) , (s (66usize) , R (p (24usize))) , (s (67usize) , R (p (24usize))) , (s (68usize) , R (p (24usize))) , (s (69usize) , R (p (24usize))) , (s (71usize) , R (p (24usize))) , (s (108usize) , R (p (24usize))) , (s (72usize) , R (p (24usize))) , (s (73usize) , R (p (24usize))) , (s (101usize) , R (p (24usize))) , (s (106usize) , R (p (24usize)))] }
        }),
        (i(139usize), {
            g! { [(s (1usize) , R (p (96usize))) , (s (65usize) , R (p (96usize))) , (s (74usize) , R (p (96usize))) , (s (66usize) , R (p (96usize))) , (s (67usize) , R (p (96usize))) , (s (68usize) , R (p (96usize))) , (s (69usize) , R (p (96usize))) , (s (71usize) , R (p (96usize))) , (s (108usize) , R (p (96usize))) , (s (72usize) , R (p (96usize))) , (s (73usize) , R (p (96usize))) , (s (101usize) , R (p (96usize))) , (s (102usize) , R (p (96usize))) , (s (106usize) , R (p (96usize)))] }
        }),
        (i(140usize), {
            g! { [(s (1usize) , R (p (22usize))) , (s (102usize) , R (p (22usize))) , (s (65usize) , R (p (22usize))) , (s (74usize) , R (p (22usize))) , (s (66usize) , R (p (22usize))) , (s (67usize) , R (p (22usize))) , (s (68usize) , R (p (22usize))) , (s (69usize) , R (p (22usize))) , (s (71usize) , R (p (22usize))) , (s (108usize) , R (p (22usize))) , (s (72usize) , R (p (22usize))) , (s (73usize) , R (p (22usize))) , (s (101usize) , R (p (22usize))) , (s (106usize) , R (p (22usize)))] }
        }),
        (i(141usize), {
            g! { [(s (109usize) , S (i (53usize))) , (s (107usize) , S (i (52usize))) , (s (108usize) , S (i (29usize))) , (s (110usize) , S (i (54usize))) , (s (111usize) , S (i (55usize))) , (s (70usize) , S (i (49usize))) , (s (87usize) , S (i (50usize))) , (s (103usize) , S (i (51usize)))] }
        }),
        (i(142usize), {
            g! { [(s (106usize) , S (i (139usize)))] }
        }),
        (i(143usize), {
            g! { [(s (1usize) , R (p (25usize))) , (s (65usize) , R (p (25usize))) , (s (74usize) , R (p (25usize))) , (s (66usize) , R (p (25usize))) , (s (67usize) , R (p (25usize))) , (s (68usize) , R (p (25usize))) , (s (69usize) , R (p (25usize))) , (s (71usize) , R (p (25usize))) , (s (108usize) , R (p (25usize))) , (s (72usize) , R (p (25usize))) , (s (73usize) , R (p (25usize))) , (s (101usize) , R (p (25usize))) , (s (93usize) , S (i (86usize))) , (s (94usize) , S (i (87usize))) , (s (95usize) , S (i (88usize))) , (s (96usize) , S (i (89usize))) , (s (97usize) , S (i (90usize))) , (s (98usize) , S (i (91usize))) , (s (88usize) , S (i (81usize))) , (s (89usize) , S (i (82usize))) , (s (90usize) , S (i (83usize))) , (s (91usize) , S (i (84usize))) , (s (92usize) , S (i (85usize))) , (s (99usize) , S (i (92usize))) , (s (100usize) , S (i (93usize))) , (s (84usize) , R (p (25usize))) , (s (105usize) , R (p (25usize))) , (s (104usize) , R (p (25usize))) , (s (81usize) , R (p (25usize))) , (s (106usize) , R (p (25usize)))] }
        }),
        (i(144usize), {
            g! { [(s (1usize) , R (p (23usize))) , (s (65usize) , R (p (23usize))) , (s (74usize) , R (p (23usize))) , (s (66usize) , R (p (23usize))) , (s (67usize) , R (p (23usize))) , (s (68usize) , R (p (23usize))) , (s (69usize) , R (p (23usize))) , (s (71usize) , R (p (23usize))) , (s (108usize) , R (p (23usize))) , (s (72usize) , R (p (23usize))) , (s (73usize) , R (p (23usize))) , (s (101usize) , R (p (23usize))) , (s (102usize) , R (p (23usize))) , (s (106usize) , R (p (23usize)))] }
        }),
    ])
}
