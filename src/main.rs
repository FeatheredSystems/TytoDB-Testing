use std::{fs::File, io::Read, path::PathBuf};
use tytodb_client::{
    AlbaTypes, BIGINT, EMAIL, FLOAT, GEO, HUGE_INT, INT, LARGE_BYTES, ToAlbaAlbaTypes, U_HUGE_INT,
    UBIGINT, UINT, alba,
    client_thread::Client,
    handler::{
        BatchBuilder, CreateContainerBuilder, CreateRowBuilder, DeleteContainerBuilder,
        DeleteRowBuilder, EditRowBuilder, SearchBuilder,
    },
    lo,
    logical_operators::LogicalOperator,
};
const COUNT: i32 = 10;
fn main() {
    // --- Setup ---
    println!("\n=== TytoDB Test Routine ===\n");

    let mut path: PathBuf = dirs::home_dir().expect("Could not find home directory");
    path.push("TytoDB/.secret");

    let mut secret = [0u8; 32];
    File::open(path).unwrap().read_exact(&mut secret).unwrap();

    let client = Client::connect("127.0.0.1:4287", secret).unwrap();
    println!("[+] Connected to server");

    // --- Container creation ---
    let _ = client.execute(
        DeleteContainerBuilder::new()
            .put_container("CCCCCC".to_string())
            .finish()
            .unwrap(),
    );
    println!("[*] Dropped old container (if existed)");

    client
        .execute(
            CreateContainerBuilder::new()
                .put_container("CCCCCC".to_string())
                .insert_header("a".to_string(), UBIGINT)
                .insert_header("b".to_string(), FLOAT)
                .insert_header("c".to_string(), BIGINT)
                .insert_header("d".to_string(), U_HUGE_INT)
                .insert_header("e".to_string(), HUGE_INT)
                .insert_header("f".to_string(), INT)
                .insert_header("g".to_string(), UINT)
                .insert_header("h".to_string(), LARGE_BYTES)
                .insert_header("i".to_string(), EMAIL)
                .insert_header("j".to_string(), GEO)
                .insert_header("k".to_string(), EMAIL)
                .finish()
                .unwrap(),
        )
        .unwrap();
    println!("[+] Created container CCCCCCC");

    // --- Insert rows ---
    let mut batch = BatchBuilder::new().transaction(true);
    for i in 0..COUNT {
        let b: Vec<u8> = i.to_le_bytes().to_vec();
        batch = batch.push(
            CreateRowBuilder::new()
                .put_container("CCCCCC".to_string())
                .insert_value("a".to_string(), AlbaTypes::U64(i as u64))
                .insert_value("b".to_string(), AlbaTypes::F64(i as f64))
                .insert_value("c".to_string(), AlbaTypes::I64(i as i64))
                .insert_value("d".to_string(), AlbaTypes::U128(i as u128))
                .insert_value("e".to_string(), AlbaTypes::I128(i as i128))
                .insert_value("f".to_string(), AlbaTypes::I32(i))
                .insert_value("g".to_string(), AlbaTypes::U32(i as u32))
                .insert_value("h".to_string(), AlbaTypes::Bytes(b.clone()))
                .insert_value("i".to_string(), AlbaTypes::String(String::from_utf8_lossy(&b).to_string()))
                .insert_value("j".to_string(), AlbaTypes::Geo((i as f64, i as f64)))
                .insert_value("k".to_string(), AlbaTypes::String(String::from_utf8_lossy(&b).to_string())),
        );
    }
    client.execute(batch.finish().unwrap()).unwrap();
    println!("[+] Inserted {COUNT} rows");

    // --- Edit rows ---
    let mut edit_batch = BatchBuilder::new().transaction(true);
    for i in 0..COUNT {
        let b: Vec<u8> = i.to_le_bytes().to_vec();
        edit_batch = edit_batch.push(
            EditRowBuilder::new()
                .put_container("CCCCCC".to_string())
                .edit_column("a".to_string(), AlbaTypes::U64((i * 2) as u64))
                .edit_column("b".to_string(), AlbaTypes::F64((i * 2) as f64))
                .edit_column("c".to_string(), AlbaTypes::I64((i * 2) as i64))
                .edit_column("d".to_string(), AlbaTypes::U128((i * 2) as u128))
                .edit_column("e".to_string(), AlbaTypes::I128((i * 2) as i128))
                .edit_column("f".to_string(), AlbaTypes::I32(i * 2))
                .edit_column("g".to_string(), AlbaTypes::U32((i * 2) as u32))
                .edit_column("h".to_string(), AlbaTypes::Bytes(b.clone()))
                .edit_column("i".to_string(), AlbaTypes::String(String::from_utf8_lossy(&b).to_string()))
                .edit_column("j".to_string(), AlbaTypes::Geo(((i * 2) as f64, (i * 2) as f64)))
                .edit_column("k".to_string(), AlbaTypes::String(String::from_utf8_lossy(&b).to_string()))
                .add_conditions(("a".to_string(), lo!(=), AlbaTypes::U64(i as u64)), true),
        );
    }
    client.execute(edit_batch.finish().unwrap()).unwrap();
    println!("[+] Edited rows");

    // --- Verify rows ---
    for i in 0..COUNT {
        let s = SearchBuilder::new()
            .add_container(String::from("CCCCCC"))
            .add_conditions(("a".to_string(), lo!(=), alba!((i * 2) as u64)), true)
            .add_column_name(String::from("a"));
        let r = client.execute(s.finish().unwrap()).unwrap();
        assert_eq!(r.row_list.len(), 1);
    }
    println!("[+] Verified row edits");

    // --- Delete half the rows ---
    let _ = client
        .execute(
            BatchBuilder::new()
                .transaction(true)
                .push(
                    DeleteRowBuilder::new()
                        .put_container(String::from("CCCCCC"))
                        .add_conditions((String::from("a"), lo!(>), alba!((COUNT / 2) * 2)), true),
                )
                .finish()
                .unwrap(),
        )
        .unwrap();
    println!("[*] Deleted half the rows");

    // --- Final verification ---
    let all_rows = client
        .execute(
            SearchBuilder::new()
                .add_container("CCCCCC".to_string())
                .add_column_name("a".to_string())
                .finish()
                .unwrap(),
        )
        .unwrap();
    assert_eq!((COUNT / 2) as usize + 1, all_rows.row_list.len());
    println!("[+] Final verification passed ({} rows)", all_rows.row_list.len());

    // --- Cleanup ---
    let _ = client.execute(
        DeleteContainerBuilder::new()
            .put_container("CCCCCC".to_string())
            .finish()
            .unwrap(),
    );
    println!("[*] Dropped container");

    println!("\n=== Finished testing successfully ===\n");
}

