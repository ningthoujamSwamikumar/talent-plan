use iterator_forge::{CsvReader, Query, Row};

// ---------------------------------------------------------------------------
// Helper: sample CSV data used across many tests
// ---------------------------------------------------------------------------

fn sample_csv() -> &'static str {
    "name,age,city\nAlice,30,New York\nBob,25,London\nCarol,35,Paris\n"
}

fn numeric_csv() -> &'static str {
    "product,price,quantity\nApples,1.50,10\nBananas,0.75,20\nCherries,3.00,5\n"
}

// ===========================================================================
// Part 1 — CsvReader basic parsing
// ===========================================================================

#[test]
fn test_csv_reader_parses_headers_and_rows() {
    let rows: Vec<Row> = CsvReader::new(sample_csv()).collect();
    println!("rows: {:?}", rows);
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].get("name"), Some("Alice"));
    assert_eq!(rows[0].get("age"), Some("30"));
    assert_eq!(rows[0].get("city"), Some("New York"));
}

#[test]
fn test_csv_reader_second_and_third_rows() {
    let rows: Vec<Row> = CsvReader::new(sample_csv()).collect();
    assert_eq!(rows[1].get("name"), Some("Bob"));
    assert_eq!(rows[2].get("city"), Some("Paris"));
}

#[test]
fn test_csv_reader_empty_input() {
    let rows: Vec<Row> = CsvReader::new("").collect();
    println!("row: {:?}", rows);
    assert!(rows.is_empty());
}

#[test]
fn test_csv_reader_header_only() {
    let rows: Vec<Row> = CsvReader::new("a,b,c\n").collect();
    assert!(rows.is_empty());
}

// ===========================================================================
// Part 1 — Laziness proof
// ===========================================================================

#[test]
fn test_csv_reader_is_lazy() {
    // Creating a CsvReader and taking only the first element should NOT
    // require the entire input to be processed.

    let csv = "x\n1\n2\n3\n4\n5\n";
    let mut reader = CsvReader::new(csv);

    // We only pull one row — the rest must remain unconsumed.
    let first = reader.next();
    assert!(first.is_some());
    assert_eq!(first.unwrap().get("x"), Some("1"));

    // Pull a second row to make sure the iterator keeps progressing correctly.
    let second = reader.next();
    assert!(second.is_some());
    assert_eq!(second.unwrap().get("x"), Some("2"));
}

#[test]
fn test_query_chain_is_lazy_with_side_effects() {
    // A side-effect counter proves that filter/map are not eagerly evaluated.
    use std::cell::Cell;

    let counter = Cell::new(0u32);
    let csv = "val\n1\n2\n3\n4\n5\n";

    let mut _query_iter = Query::new(CsvReader::new(csv))
        .filter_rows(|row| {
            counter.set(counter.get() + 1);
            let v: i32 = row.get("val").unwrap().parse().unwrap();
            v > 2
        })
        .collect_rows()
        .into_iter();

    // After collect_rows we know all rows were visited, but if we instead
    // build the pipeline and only pull one element, fewer rows are visited.
    let counter2 = Cell::new(0u32);
    let reader = CsvReader::new(csv);
    let _query = Query::new(reader).filter_rows(|row| {
        counter2.set(counter2.get() + 1);
        let v: i32 = row.get("val").unwrap().parse().unwrap();
        v > 3
    });
    // The pipeline has been built but counter2 is still 0 — no work done yet.
    assert_eq!(
        counter2.get(),
        0,
        "Lazy pipeline must not evaluate until consumed"
    );
}

// ===========================================================================
// Part 2 — filter_rows
// ===========================================================================

#[test]
fn test_filter_rows_by_column_value() {
    let rows = Query::new(CsvReader::new(sample_csv()))
        .filter_rows(|row| row.get("city") == Some("London"))
        .collect_rows();

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].get("name"), Some("Bob"));
}

#[test]
fn test_filter_rows_no_match() {
    let rows = Query::new(CsvReader::new(sample_csv()))
        .filter_rows(|row| row.get("city") == Some("Tokyo"))
        .collect_rows();

    assert!(rows.is_empty());
}

// ===========================================================================
// Part 2 — map_column
// ===========================================================================

#[test]
fn test_map_column_transformation() {
    let rows = Query::new(CsvReader::new(sample_csv()))
        .map_column("name", |name| name.to_uppercase())
        .collect_rows();

    assert_eq!(rows[0].get("name"), Some("ALICE"));
    assert_eq!(rows[1].get("name"), Some("BOB"));
    assert_eq!(rows[2].get("name"), Some("CAROL"));
}

// ===========================================================================
// Part 3 — select (column projection)
// ===========================================================================

#[test]
fn test_select_columns() {
    let rows = Query::new(CsvReader::new(sample_csv()))
        .select(&["name", "city"])
        .collect_rows();

    assert_eq!(rows[0].headers(), &["name", "city"]);
    assert_eq!(rows[0].get("name"), Some("Alice"));
    assert_eq!(rows[0].get("city"), Some("New York"));
    assert_eq!(rows[0].get("age"), None); // projected away
}

// ===========================================================================
// Part 3 — chaining multiple operations
// ===========================================================================

#[test]
fn test_chained_filter_map_select() {
    let rows = Query::new(CsvReader::new(sample_csv()))
        .filter_rows(|row| {
            let age: i32 = row.get("age").unwrap().parse().unwrap();
            age >= 30
        })
        .map_column("name", |n| n.to_uppercase())
        .select(&["name", "age"])
        .collect_rows();

    println!("rows: {:?}", rows);
    assert_eq!(rows.len(), 2); // Alice (30) and Carol (35)
    assert_eq!(rows[0].get("name"), Some("ALICE"));
    assert_eq!(rows[1].get("name"), Some("CAROL"));
    // city column should be gone
    assert_eq!(rows[0].get("city"), None);
}

// ===========================================================================
// Part 4 — Aggregations
// ===========================================================================

#[test]
fn test_count() {
    let c = Query::new(CsvReader::new(sample_csv())).count();
    assert_eq!(c, 3);
}

#[test]
fn test_sum() {
    let s = Query::new(CsvReader::new(numeric_csv())).sum("price");
    assert!((s - 5.25).abs() < f64::EPSILON);
}

#[test]
fn test_avg() {
    let a = Query::new(CsvReader::new(numeric_csv())).avg("price");
    assert!((a - 1.75).abs() < f64::EPSILON);
}

#[test]
fn test_min() {
    let m = Query::new(CsvReader::new(numeric_csv())).min("price");
    assert_eq!(m, Some(0.75));
}

#[test]
fn test_max() {
    let m = Query::new(CsvReader::new(numeric_csv())).max("quantity");
    assert_eq!(m, Some(20.0));
}

#[test]
fn test_aggregation_on_empty_result() {
    let empty = "x\n";
    println!("rows: {:?}", CsvReader::new(empty).collect::<Vec<_>>());
    assert_eq!(Query::new(CsvReader::new(empty)).count(), 0);
    assert_eq!(Query::new(CsvReader::new(empty)).sum("x"), 0.0);
    assert_eq!(Query::new(CsvReader::new(empty)).avg("x"), 0.0);
    assert_eq!(Query::new(CsvReader::new(empty)).min("x"), None);
    assert_eq!(Query::new(CsvReader::new(empty)).max("x"), None);
}

// ===========================================================================
// Part 5 — FromIterator (collect rows back to CSV string)
// ===========================================================================

#[test]
fn test_collect_rows_to_csv_string() {
    let csv_out: String = CsvReader::new(sample_csv()).collect();
    println!("csv_out: {:?}", csv_out);

    // The output should contain the header and all data rows.
    let lines: Vec<&str> = csv_out.trim().lines().collect();
    assert_eq!(lines.len(), 4); // 1 header + 3 data rows
    assert_eq!(lines[0], "name,age,city");
    assert_eq!(lines[1], "Alice,30,New York");
}

// ===========================================================================
// Edge cases
// ===========================================================================

#[test]
fn test_malformed_rows_fewer_columns() {
    // Row with fewer values than headers — missing values should still
    // produce a Row (implementation-defined behavior: could pad with empty
    // strings or skip the row). This test verifies no panic occurs.
    let csv = "a,b,c\n1,2\n4,5,6\n";
    let rows: Vec<Row> = CsvReader::new(csv).collect();
    // At minimum the well-formed row must survive.
    assert!(rows.iter().any(|r| r.get("c") == Some("6")));
}

#[test]
fn test_large_dataset() {
    // Generate a CSV with 100+ rows and verify aggregation.
    let mut csv = String::from("index,value\n");
    for i in 1..=150 {
        csv.push_str(&format!("{},{}\n", i, i * 2));
    }

    let count = Query::new(CsvReader::new(&csv)).count();
    assert_eq!(count, 150);

    let sum = Query::new(CsvReader::new(&csv)).sum("value");
    // sum of 2+4+6+...+300 = 2*(1+2+...+150) = 2 * (150*151/2) = 22650
    assert!((sum - 22650.0).abs() < f64::EPSILON);
}
