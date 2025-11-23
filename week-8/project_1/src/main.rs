

#[derive(Debug)]
struct Staff {
    role: String,
    years_of_experience: u8,
}

fn get_aps_level(staff: &Staff) -> String {
    // Vector of APS mappings (role, min_years, max_years, APS level)
    let aps_table = vec![
        ("Paralegal", 0, 2, "APS 1-2"),
        ("Intern", 0, 2, "APS 1-2"),
        ("Administrator", 3, 5, "APS 3-5"),
        ("Research Assistant", 3, 5, "APS 3-5"),
        ("Junior Associate", 3, 5, "APS 3-5"),
        ("Classroom Teacher", 3, 5, "APS 3-5"),
        ("Associate", 5, 8, "APS 5-8"),
        ("Snr Teacher", 5, 8, "APS 5-8"),
        ("Office Manager", 8, 10, "EL 1 8-10"),
        ("Post-Doc Researcher", 8, 10, "EL 1 8-10"),
        ("Senior Associate 1-2", 8, 10, "EL 1 8-10"),
        ("Director", 10, 13, "EL 2 10-13"),
        ("Senior Lecturer", 10, 13, "EL 2 10-13"),
        ("Senior Associate 3-4", 10, 13, "EL 2 10-13"),
        ("CEO", 13, 50, "SES"),
        ("Dean", 13, 50, "SES"),
        ("Partner", 13, 50, "SES"),
        ("Principal", 13, 50, "SES"),
    ];

    for (role, min, max, aps) in aps_table {
        if staff.role == role && (staff.years_of_experience >= min && staff.years_of_experience <= max) {
            return aps.to_string();
        }
    }

    "No APS Level Found".to_string()
}

fn main() {
    let staff = Staff {
        role: "Associate".to_string(),
        years_of_experience: 6,
    };

    println!("Staff Details: {:?}", staff);
    println!("APS Level: {}", get_aps_level(&staff));
}

