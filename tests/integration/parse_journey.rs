use chrono::NaiveDateTime;
use helsinki_bike_app::journey::Journey;

#[test]
fn parsing_well_formed_journey_data_works() -> Result<(), csv::Error> {
    let csv = r#"
Departure,Return,Departure station id,Departure station name,Return station id,Return station name,Covered distance (m),Duration (sec.)
2021-05-31T23:57:25,2021-06-01T00:05:46,094,Laajalahden aukio,100,Teljäntie,2043,500
2021-05-31T23:56:59,2021-06-01T00:07:14,082,Töölöntulli,113,Pasilan asema,1870,611
2021-05-31T23:56:44,2021-06-01T00:03:26,123,Näkinsilta,121,Vilhonvuorenkatu,1025,399
"#;
    let journeys = vec![
        Journey {
            departure_time: "2021-05-31T23:57:25"
                .parse::<NaiveDateTime>()
                .unwrap(),
            return_time: "2021-06-01T00:05:46"
                .parse::<NaiveDateTime>()
                .unwrap(),
            departure_station_id: "094".to_string(),
            return_station_id: "100".to_string(),
            distance_m: 2043.0,
            duration_sec: 500.0,
        },
        Journey {
            departure_time: "2021-05-31T23:56:59"
                .parse::<NaiveDateTime>()
                .unwrap(),
            return_time: "2021-06-01T00:07:14"
                .parse::<NaiveDateTime>()
                .unwrap(),
            departure_station_id: "082".to_string(),
            return_station_id: "113".to_string(),
            distance_m: 1870.0,
            duration_sec: 611.0,
        },
        Journey {
            departure_time: "2021-05-31T23:56:44"
                .parse::<NaiveDateTime>()
                .unwrap(),
            return_time: "2021-06-01T00:03:26"
                .parse::<NaiveDateTime>()
                .unwrap(),
            departure_station_id: "123".to_string(),
            return_station_id: "121".to_string(),
            distance_m: 1025.0,
            duration_sec: 399.0,
        },
    ];

    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    for (actual, expected) in reader.deserialize().zip(journeys.into_iter()) {
        let actual: Journey = actual?;

        assert_eq!(actual, expected);
    }

    Ok(())
}

#[test]
fn parsing_missing_timestamps_works() -> Result<(), csv::Error> {
    let csv = r#"
Departure,Return,Departure station id,Departure station name,Return station id,Return station name,Covered distance (m),Duration (sec.)
2021-05-31,2021-06-01T00:05:46,094,Laajalahden aukio,100,Teljäntie,2043,500
2021-05-31T23:56:59,2021-06-01,082,Töölöntulli,113,Pasilan asema,1870,611
2021-05-31,2021-06-01,123,Näkinsilta,121,Vilhonvuorenkatu,1025,399
"#;
    let journeys = vec![
        Journey {
            departure_time: "2021-05-31T00:00:00"
                .parse::<NaiveDateTime>()
                .unwrap(),
            return_time: "2021-06-01T00:05:46"
                .parse::<NaiveDateTime>()
                .unwrap(),
            departure_station_id: "094".to_string(),
            return_station_id: "100".to_string(),
            distance_m: 2043.0,
            duration_sec: 500.0,
        },
        Journey {
            departure_time: "2021-05-31T23:56:59"
                .parse::<NaiveDateTime>()
                .unwrap(),
            return_time: "2021-06-01T00:00:00"
                .parse::<NaiveDateTime>()
                .unwrap(),
            departure_station_id: "082".to_string(),
            return_station_id: "113".to_string(),
            distance_m: 1870.0,
            duration_sec: 611.0,
        },
        Journey {
            departure_time: "2021-05-31T00:00:00"
                .parse::<NaiveDateTime>()
                .unwrap(),
            return_time: "2021-06-01T00:00:00"
                .parse::<NaiveDateTime>()
                .unwrap(),
            departure_station_id: "123".to_string(),
            return_station_id: "121".to_string(),
            distance_m: 1025.0,
            duration_sec: 399.0,
        },
    ];
    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    for (actual, expected) in reader.deserialize().zip(journeys.into_iter()) {
        let actual: Journey = actual?;
        let expected = Journey { ..expected };

        assert_eq!(actual, expected);
    }

    Ok(())
}

#[test]
fn parsing_missing_distance_works() -> csv::Result<()> {
    let csv = r#"
Departure,Return,Departure station id,Departure station name,Return station id,Return station name,Covered distance (m),Duration (sec.)
2021-05-31T23:57:25,2021-06-01T00:05:46,094,Laajalahden aukio,100,Teljäntie,,500
"#;
    let expected = Journey {
        departure_time: "2021-05-31T23:57:25".parse::<NaiveDateTime>().unwrap(),
        return_time: "2021-06-01T00:05:46".parse::<NaiveDateTime>().unwrap(),
        departure_station_id: "094".to_string(),
        return_station_id: "100".to_string(),
        distance_m: 0.0,
        duration_sec: 500.0,
    };
    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    let actual: Journey = reader.deserialize().into_iter().next().unwrap()?;

    assert_eq!(actual, expected);

    Ok(())
}

#[test]
fn parsing_missing_duration_works() -> csv::Result<()> {
    let csv = r#"
Departure,Return,Departure station id,Departure station name,Return station id,Return station name,Covered distance (m),Duration (sec.)
2021-05-31T23:57:25,2021-06-01T00:05:46,094,Laajalahden aukio,100,Teljäntie,2043,
"#;
    let expected = Journey {
        departure_time: "2021-05-31T23:57:25".parse::<NaiveDateTime>().unwrap(),
        return_time: "2021-06-01T00:05:46".parse::<NaiveDateTime>().unwrap(),
        departure_station_id: "094".to_string(),
        return_station_id: "100".to_string(),
        distance_m: 2043.0,
        duration_sec: 0.0,
    };
    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    let actual: Journey = reader.deserialize().into_iter().next().unwrap()?;

    assert_eq!(actual, expected);

    Ok(())
}
