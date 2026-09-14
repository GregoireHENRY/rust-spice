//! Events kernels: writing them, loading them, and querying them.

use crate::common;

/// Number of rows the `DATAORDERS` table is written with.
const ROWS: usize = 9;

/// The path of a file the test is about to write, cleared so that a rerun starts from nothing.
fn scratch(name: &str) -> String {
    let path = common::kernel(name);
    let _ = std::fs::remove_file(&path);
    path
}

/// Read one of the fixed size, null terminated names a segment summary reports.
fn name(buffer: &[spice::c::SpiceChar]) -> String {
    let end = buffer
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(buffer.len());
    buffer[..end]
        .iter()
        .map(|&byte| byte as u8 as char)
        .collect()
}

#[test]
#[serial]
fn written_a_column_at_a_time() {
    common::load();
    let path = scratch("orders.bes");

    let handle = spice::ekopn(&path, "rust-spice test EK", 0);
    common::assert_ok("ekopn");

    // Every column is scalar, and only the cost may be missing.
    let names = ["ORDER_ID", "CUSTOMER_ID", "LAST_NAME", "ORDER_DATE", "COST"];
    let decls = [
        "DATATYPE = INTEGER, INDEXED = TRUE",
        "DATATYPE = INTEGER, INDEXED = TRUE",
        "DATATYPE = CHARACTER*(*), INDEXED = TRUE",
        "DATATYPE = TIME, INDEXED = TRUE",
        "DATATYPE = DOUBLE PRECISION, INDEXED = TRUE, NULLS_OK = TRUE",
    ];
    let (segno, rcptrs) = spice::ekifld(handle, "DATAORDERS", ROWS as i32, &names, &decls);
    common::assert_ok("ekifld");
    assert_eq!(segno, 0, "the first segment of the file");
    assert_eq!(rcptrs.len(), ROWS, "one record pointer per row");

    let orders = (0..ROWS as i32).collect::<Vec<_>>();
    let customers = orders.iter().map(|&order| order * 100).collect::<Vec<_>>();
    let surnames = orders
        .iter()
        .map(|order| format!("Customer {order}"))
        .collect::<Vec<_>>();
    let dates = orders
        .iter()
        .map(|order| spice::str2et(&format!("1998 MAR {}", order + 1)))
        .collect::<Vec<_>>();
    let costs = orders
        .iter()
        .map(|&order| 100.0 * order as f64)
        .collect::<Vec<_>>();

    // The sizes are read only for columns of varying size, and the null flags only for the columns
    // that allow nulls, but both have to be there.
    let sizes = vec![1; ROWS];
    let present = vec![false; ROWS];
    let mut missing = vec![false; ROWS];
    missing[1] = true;

    spice::ekacli(
        handle, segno, "ORDER_ID", &orders, &sizes, &present, &rcptrs,
    );
    spice::ekacli(
        handle,
        segno,
        "CUSTOMER_ID",
        &customers,
        &sizes,
        &present,
        &rcptrs,
    );
    spice::ekaclc(
        handle,
        segno,
        "LAST_NAME",
        &surnames,
        &sizes,
        &present,
        &rcptrs,
    );
    spice::ekacld(
        handle,
        segno,
        "ORDER_DATE",
        &dates,
        &sizes,
        &present,
        &rcptrs,
    );
    spice::ekacld(handle, segno, "COST", &costs, &sizes, &missing, &rcptrs);
    common::assert_ok("writing the columns");

    let mut rcptrs = rcptrs;
    spice::ekffld(handle, segno, &mut rcptrs);
    spice::ekcls(handle);
    common::assert_ok("finishing the segment");

    // The file can be summarised without being loaded.
    let reader = spice::ekopr(&path);
    common::assert_ok("ekopr");
    assert_eq!(spice::eknseg(reader), 1);
    let summary = spice::ekssum(reader, 0);
    common::assert_ok("ekssum");
    assert_eq!(name(&summary.tabnam), "DATAORDERS");
    assert_eq!(summary.nrows, ROWS as i32);
    assert_eq!(summary.ncols, names.len() as i32);
    for (index, expected) in names.iter().enumerate() {
        assert_eq!(&name(&summary.cnames[index]), expected);
    }
    assert_eq!(summary.cdescrs[4].dtype, spice::EK_DP);
    assert_ne!(summary.cdescrs[4].nullok, 0, "the cost may be missing");
    assert_eq!(summary.cdescrs[0].nullok, 0, "the order number may not");
    spice::ekcls(reader);
    common::assert_ok("closing the reader");

    // Loading it is what makes the table visible to the queries.
    let loaded = spice::eklef(&path);
    common::assert_ok("eklef");
    assert_eq!(spice::ekntab(), 1);
    assert_eq!(spice::ektnam(0, spice::MAX_LEN_OUT as i32), "DATAORDERS");
    assert_eq!(spice::ekccnt("DATAORDERS"), names.len() as i32);

    let (column, attributes) = spice::ekcii("DATAORDERS", 3, spice::MAX_LEN_OUT as i32);
    common::assert_ok("ekcii");
    assert_eq!(column, "ORDER_DATE");
    assert_eq!(attributes.dtype, spice::EK_TIME);
    assert_eq!(attributes.size, 1, "a scalar column");
    assert_ne!(attributes.indexd, 0);

    // Six of the nine orders cost more than two hundred and fifty.
    let query = "SELECT ORDER_ID, LAST_NAME, COST FROM DATAORDERS \
                 WHERE COST > 250 ORDER BY ORDER_ID";
    let (rows, failed, message) = spice::ekfind(query, spice::MAX_LEN_OUT as i32);
    common::assert_ok("ekfind");
    assert!(!failed, "the query should parse: {message}");
    assert_eq!(rows, 6);

    for row in 0..rows {
        assert_eq!(spice::eknelt(0, row), 1, "a scalar entry holds one value");

        let (order, null, found) = spice::ekgi(0, row, 0);
        assert!(found && !null);
        assert_eq!(order, row + 3);

        let (surname, null, found) = spice::ekgc(1, row, 0, spice::MAX_LEN_OUT as i32);
        assert!(found && !null);
        assert_eq!(surname, format!("Customer {}", row + 3));

        let (cost, null, found) = spice::ekgd(2, row, 0);
        assert!(found && !null);
        assert_relative_eq!(cost, 100.0 * (row + 3) as f64, epsilon = 1e-9);
    }
    common::assert_ok("reading the matched rows");

    // The one cost that was written as missing reads back as null, and the epoch column reads back
    // as the number it was given.
    let (rows, failed, message) = spice::ekfind(
        "SELECT COST, ORDER_DATE FROM DATAORDERS WHERE ORDER_ID = 1",
        spice::MAX_LEN_OUT as i32,
    );
    assert!(!failed, "{message}");
    assert_eq!(rows, 1);
    let (_, null, found) = spice::ekgd(0, 0, 0);
    assert!(found && null, "the cost of the second order is missing");
    let (when, null, found) = spice::ekgd(1, 0, 0);
    assert!(found && !null);
    assert_relative_eq!(when, dates[1], epsilon = 1e-6);

    spice::ekuef(loaded);
    common::assert_ok("ekuef");
    assert_eq!(spice::ekntab(), 0, "nothing is loaded any more");

    common::unload();
}

#[test]
#[serial]
fn written_a_record_at_a_time() {
    common::load();
    let path = scratch("items.bes");

    let handle = spice::ekopn(&path, "rust-spice test EK", 0);
    let names = ["ITEM_ID", "ITEM_NAME", "PRICE", "TAGS"];
    let decls = [
        "DATATYPE = INTEGER, INDEXED = TRUE",
        "DATATYPE = CHARACTER*(32), INDEXED = TRUE",
        "DATATYPE = DOUBLE PRECISION, NULLS_OK = TRUE",
        "DATATYPE = CHARACTER*(16), SIZE = VARIABLE",
    ];
    let segno = spice::ekbseg(handle, "DATAITEMS", &names, &decls);
    common::assert_ok("ekbseg");
    assert_eq!(segno, 0);

    // Two records appended, then one inserted between them.
    let write = |recno: i32, id: i32, item: &str, price: Option<f64>, tags: &[&str]| {
        spice::ekacei(handle, segno, recno, "ITEM_ID", &[id], false);
        spice::ekacec(handle, segno, recno, "ITEM_NAME", &[item], false);
        spice::ekaced(
            handle,
            segno,
            recno,
            "PRICE",
            &[price.unwrap_or_default()],
            price.is_none(),
        );
        spice::ekacec(handle, segno, recno, "TAGS", tags, false);
    };

    assert_eq!(spice::ekappr(handle, segno), 0);
    write(0, 10, "bolt", Some(1.5), &["metal", "small"]);
    assert_eq!(spice::ekappr(handle, segno), 1);
    write(1, 30, "beam", None, &["metal"]);
    spice::ekinsr(handle, segno, 1);
    write(1, 20, "rope", Some(12.25), &["fibre", "long", "coiled"]);
    common::assert_ok("filling the records");

    // The inserted record pushed the third one along.
    let (ids, null) = spice::ekrcei(handle, segno, 2, "ITEM_ID", 4);
    common::assert_ok("ekrcei");
    assert_eq!((ids.as_slice(), null), ([30].as_slice(), false));

    let (items, null) = spice::ekrcec(handle, segno, 1, "ITEM_NAME", 4, 33);
    common::assert_ok("ekrcec");
    assert_eq!(
        (items.as_slice(), null),
        (["rope".to_string()].as_slice(), false)
    );

    // A column of varying size reports as many values as the record holds.
    let (tags, null) = spice::ekrcec(handle, segno, 1, "TAGS", 8, 17);
    assert_eq!(tags, vec!["fibre", "long", "coiled"]);
    assert!(!null);

    // The price that was written as missing reads back as null.
    let (_, null) = spice::ekrced(handle, segno, 2, "PRICE", 4);
    common::assert_ok("ekrced");
    assert!(null);
    let (prices, null) = spice::ekrced(handle, segno, 0, "PRICE", 4);
    assert_eq!((prices.as_slice(), null), ([1.5].as_slice(), false));

    // Deleting the last record leaves the two before it alone.
    spice::ekdelr(handle, segno, 2);
    common::assert_ok("ekdelr");
    let (ids, _) = spice::ekrcei(handle, segno, 1, "ITEM_ID", 4);
    assert_eq!(ids, vec![20]);

    spice::ekcls(handle);
    common::assert_ok("closing the new EK");

    // Reopening for writing is what lets a record be corrected.
    let handle = spice::ekopw(&path);
    common::assert_ok("ekopw");
    spice::ekucei(handle, 0, 1, "ITEM_ID", &[21], false);
    spice::ekucec(handle, 0, 1, "ITEM_NAME", &["cord"], false);
    spice::ekuced(handle, 0, 1, "PRICE", &[9.75], false);
    common::assert_ok("updating the record");

    let (ids, _) = spice::ekrcei(handle, 0, 1, "ITEM_ID", 4);
    assert_eq!(ids, vec![21]);
    let (items, _) = spice::ekrcec(handle, 0, 1, "ITEM_NAME", 4, 33);
    assert_eq!(items, vec!["cord"]);
    let (prices, null) = spice::ekrced(handle, 0, 1, "PRICE", 4);
    assert_eq!((prices.as_slice(), null), ([9.75].as_slice(), false));
    spice::ekcls(handle);
    common::assert_ok("closing the corrected EK");

    // What the queries see is what was written.
    let loaded = spice::eklef(&path);
    let (rows, failed, message) = spice::ekfind(
        "SELECT ITEM_NAME, TAGS FROM DATAITEMS ORDER BY ITEM_ID",
        spice::MAX_LEN_OUT as i32,
    );
    assert!(!failed, "{message}");
    assert_eq!(rows, 2);
    assert_eq!(spice::ekgc(0, 0, 0, spice::MAX_LEN_OUT as i32).0, "bolt");
    assert_eq!(spice::ekgc(0, 1, 0, spice::MAX_LEN_OUT as i32).0, "cord");
    assert_eq!(spice::eknelt(1, 0), 2, "the first item carries two tags");
    assert_eq!(spice::eknelt(1, 1), 3, "the second carries three");
    assert_eq!(spice::ekgc(1, 1, 2, spice::MAX_LEN_OUT as i32).0, "coiled");
    common::assert_ok("querying the corrected EK");
    spice::ekuef(loaded);

    common::unload();
}

#[test]
#[serial]
fn scratch_kernels_and_query_parsing() {
    common::load();

    // A scratch kernel is written the same way, but never appears under a name and is gone when it
    // is closed.
    let handle = spice::ekops();
    common::assert_ok("ekops");
    let (segno, rcptrs) = spice::ekifld(
        handle,
        "SCRATCH",
        3,
        &["ID"],
        &["DATATYPE = INTEGER, INDEXED = TRUE"],
    );
    spice::ekacli(
        handle,
        segno,
        "ID",
        &[1, 2, 3],
        &[1; 3],
        &[false; 3],
        &rcptrs,
    );
    let mut rcptrs = rcptrs;
    spice::ekffld(handle, segno, &mut rcptrs);
    common::assert_ok("writing the scratch EK");
    assert_eq!(spice::eknseg(handle), 1);
    let (ids, _) = spice::ekrcei(handle, segno, 2, "ID", 4);
    assert_eq!(ids, vec![3]);
    spice::ekcls(handle);
    common::assert_ok("closing the scratch EK");

    // Parsing the SELECT clause reports where each item is and what it is.
    let path = scratch("parsed.bes");
    let writer = spice::ekopn(&path, "rust-spice test EK", 0);
    let (segno, rcptrs) = spice::ekifld(
        writer,
        "PARSED",
        2,
        &["ID", "LABEL"],
        &[
            "DATATYPE = INTEGER, INDEXED = TRUE",
            "DATATYPE = CHARACTER*(*), INDEXED = TRUE",
        ],
    );
    spice::ekacli(writer, segno, "ID", &[1, 2], &[1; 2], &[false; 2], &rcptrs);
    spice::ekaclc(
        writer,
        segno,
        "LABEL",
        &["first", "second"],
        &[1; 2],
        &[false; 2],
        &rcptrs,
    );
    let mut rcptrs = rcptrs;
    spice::ekffld(writer, segno, &mut rcptrs);
    spice::ekcls(writer);
    let loaded = spice::eklef(&path);
    common::assert_ok("loading the EK to parse against");

    let query = "SELECT ID, LABEL FROM PARSED";
    let (begins, ends, types, classes, tables, columns, failed, message) =
        spice::ekpsel(query, 256, 64, 64);
    common::assert_ok("ekpsel");
    assert!(!failed, "the query should parse: {message}");
    assert_eq!(columns, vec!["ID", "LABEL"]);
    assert_eq!(tables, vec!["PARSED", "PARSED"]);
    assert_eq!(types, vec![spice::EK_INT, spice::EK_CHR]);
    assert_eq!(classes, vec![spice::EK_EXP_COL, spice::EK_EXP_COL]);
    // The positions are indices into the query itself.
    assert_eq!(&query[begins[0] as usize..=ends[0] as usize], "ID");
    assert_eq!(&query[begins[1] as usize..=ends[1] as usize], "LABEL");

    // A query naming a table nobody has loaded does not signal; it reports.
    let (_, failed, message) =
        spice::ekfind("SELECT NOTHING FROM NOWHERE", spice::MAX_LEN_OUT as i32);
    common::assert_ok("ekfind over a table that is not there");
    assert!(failed);
    assert!(!message.is_empty(), "the parse error should say why");

    spice::ekuef(loaded);
    common::unload();
}
