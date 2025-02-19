use salah::prelude::*;

fn main() {
    let city = Coordinates::new(24.383_144, 88.583_183); // Rajshahi
    let date = Local::today();
    let params = Parameters::with(Method::Karachi, Madhab::Hanafi);
    let prayers = PrayerTimes::calculate(date, city, params);

    println!(
        "Fajr: {}",
        prayers.time(Prayer::Fajr).format("%-l:%M %p").to_string()
    );
    println!(
        "Sunrise: {}",
        prayers
            .time(Prayer::Sunrise)
            .format("%-l:%M %p")
            .to_string()
    );
    println!(
        "Dhuhr: {}",
        prayers.time(Prayer::Dhuhr).format("%-l:%M %p").to_string()
    );
    println!(
        "Asr: {}",
        prayers.time(Prayer::Asr).format("%-l:%M %p").to_string()
    );
    println!(
        "Maghrib: {}",
        prayers
            .time(Prayer::Maghrib)
            .format("%-l:%M %p")
            .to_string()
    );
    println!(
        "Isha: {}",
        prayers.time(Prayer::Isha).format("%-l:%M %p").to_string()
    );

    println!();

    println!("Fajr   : {}", prayers.time(Prayer::Fajr).to_rfc3339());
    println!("Sunrise: {}", prayers.time(Prayer::Sunrise).to_rfc3339());
    println!("Dhuhr  : {}", prayers.time(Prayer::Dhuhr).to_rfc3339());
    println!("Asr    : {}", prayers.time(Prayer::Asr).to_rfc3339());
    println!("Maghrib: {}", prayers.time(Prayer::Maghrib).to_rfc3339());
    println!("Isha   : {}", prayers.time(Prayer::Isha).to_rfc3339());
    println!("Qiyam  : {}", prayers.time(Prayer::Qiyam).to_rfc3339());

    println!();
    println!("Now    : {}", Local::now().to_rfc3339());
    println!();

    println!(
        "Current prayer: {} ({} minutes remaining)",
        prayers.current().name(),
        prayers.time_remaining().num_minutes()
    );

    println!(
        "Next prayer: {} @ {}",
        prayers.next().name(),
        prayers.time(prayers.next()).format("%-l:%M %p").to_string()
    );

    let t11am = Local::today().and_hms(5, 0, 0);
    let pat11am = prayers.prayer_at(t11am);

    assert_eq!(pat11am, Prayer::Fajr);

    println!(
        "Prayer @ 11am: {} ({} minutes remaining)",
        pat11am.name(),
        (prayers.time(pat11am.next()) - t11am).num_minutes()
    );

    println!(
        "Next prayer @ 11am: {} @ {}",
        pat11am.next().name(),
        prayers.time(pat11am.next()).format("%-l:%M %p").to_string()
    );
}
